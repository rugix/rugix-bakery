"""Core artifact rules reproduce bytes across fresh caches and filesystem instances."""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import time
import tomllib
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder


@pytest.mark.parametrize(
    ("rule", "parameters"),
    [
        ("image-ext4", ""),
        ("image-ext4", "source-date-epoch = 2100000000\n"),
        ("image-ext4", "size = 67108864\nsource-date-epoch = 1700000000\n"),
        ("image-fat", ""),
        ("image-fat", "size = 67108864\nsource-date-epoch = 1700000000\n"),
        ("image-fat", "size = 16777216\nsource-date-epoch = 1700000000\n"),
        ("image-squashfs", ""),
        ("image-squashfs", 'compression = "xz"\nsource-date-epoch = 1700000000\n'),
    ],
)
def test_core_rule_reproducibility(
    env_project: BakeryBuilder, tmp_path: Path, rule: str, parameters: str
) -> None:
    """Independent builds ignore wall time, inode allocation, and directory insertion order."""
    project = env_project.project_dir
    (project / "data").write_bytes(hashlib.shake_256(b"payload").digest(128 * 1024))
    (project / "layout").write_text('type = "gpt"\npartitions = [{ name = "data" }]\n')
    inputs = 'inputs.tree = { path = "tree" }\n'
    (project / "invocation.toml").write_text(
        f'uses = "core/{rule}"\n' + inputs + "[parameters]\n" + parameters
    )
    second = tmp_path / "independent project"
    shutil.copytree(project, second)
    for index, root in enumerate((project, second)):
        tree = root / "tree"
        tree.mkdir()
        names = ["nested", "other", "empty", ".config"]
        for name in names if index == 0 else reversed(names):
            (tree / name).mkdir()
        (tree / ".config/.nested").mkdir()
        names = [
            "nested/z.txt",
            "nested/a.txt",
            "other/long application filename.txt",
            ".config/loader.conf",
            ".config/.nested/settings",
        ]
        for name in names if index == 0 else reversed(names):
            (tree / name).write_text(name + "\n")
        (tree / ".config/link").symlink_to("../nested/a.txt")
        (tree / ".config/directory-link").symlink_to(
            "../other", target_is_directory=True
        )
        if rule != "image-fat":
            (tree / "link").symlink_to("nested/a.txt")
            os.link(tree / "nested/z.txt", tree / "hard-link")
            os.setxattr(tree / "nested/a.txt", "user.reproducibility", b"preserved")
            attributes = [("user.first", b"one"), ("user.second", b"two")]
            for name, value in attributes if index == 0 else reversed(attributes):
                os.setxattr(tree / "nested/a.txt", name, value)
            (tree / "nested/a.txt").chmod(0o640)
        for path in [tree, *tree.rglob("*"), root / "data", root / "layout"]:
            os.utime(
                path,
                ns=(1000000000000000000 + index * 123456789, 1600000000123456789),
                follow_symlinks=False,
            )
        bakery = (
            env_project if index == 0 else BakeryBuilder(root, tmp_path / "second-run")
        )
        bakery.run(["bake", "rule", "invocation.toml", "build/result"])
        if index == 0:
            time.sleep(2)
    first_outputs = project / "build/result"
    second_outputs = second / "build/result"
    assert sorted(p.name for p in first_outputs.iterdir()) == sorted(
        p.name for p in second_outputs.iterdir()
    )
    for output in first_outputs.iterdir():
        with (
            output.open("rb") as first,
            (second_outputs / output.name).open("rb") as other,
        ):
            assert (
                hashlib.file_digest(first, "sha256").digest()
                == hashlib.file_digest(other, "sha256").digest()
            ), f"{rule}/{output.name} differs between independent builds"

    if rule in ("image-ext4", "image-squashfs", "image-fat"):
        extract = project / "rules/extract"
        extract.mkdir(parents=True)
        commands = {
            "image-ext4": [
                "/bin/sh",
                "-ec",
                "mkdir /run/rugix/bakery/work/outputs/tree; debugfs -R 'rdump / /run/rugix/bakery/work/outputs/tree' /run/rugix/bakery/work/inputs/image",
            ],
            "image-squashfs": [
                "unsquashfs",
                "-no-progress",
                "-d",
                "/run/rugix/bakery/work/outputs/tree",
                "/run/rugix/bakery/work/inputs/image",
            ],
            "image-fat": [
                "/bin/sh",
                "-ec",
                "mkdir /run/rugix/bakery/work/outputs/tree; mcopy -s -i /run/rugix/bakery/work/inputs/image ::/.config /run/rugix/bakery/work/outputs/tree/",
            ],
        }
        command = commands[rule]
        (extract / "rule.toml").write_text(
            '[inputs]\nimage = "file"\n[execution]\ntype = "run"\ncommand = '
            + json.dumps(command)
            + '\n[execution.outputs.tree]\nkind = "directory"\npath = "tree"\n'
        )
        (project / "invocation.toml").write_text(
            'uses = "extract"\ninputs.image = { path = "build/result/image" }\n'
        )
        env_project.run(["bake", "rule", "invocation.toml", "build/extracted"])
        extracted = project / "build/extracted/tree"
        assert (
            extracted / ".config/loader.conf"
        ).read_text() == ".config/loader.conf\n"
        assert (
            extracted / ".config/.nested/settings"
        ).read_text() == ".config/.nested/settings\n"
        if rule == "image-fat":
            assert (extracted / ".config/link").read_text() == "nested/a.txt\n"
            assert (
                extracted / ".config/directory-link/long application filename.txt"
            ).read_text() == "other/long application filename.txt\n"
            return
        epoch = tomllib.loads(parameters).get("source-date-epoch", 0)
        assert (extracted / "nested/a.txt").stat().st_mtime == min(epoch, 1600000000)
        assert (extracted / "nested/a.txt").stat().st_mode & 0o777 == 0o640
        assert os.readlink(extracted / "link") == "nested/a.txt"
        assert (extracted / "hard-link").stat().st_ino == (
            extracted / "nested/z.txt"
        ).stat().st_ino
        assert (
            os.getxattr(extracted / "nested/a.txt", "user.reproducibility")
            == b"preserved"
        )
