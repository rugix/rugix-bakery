"""Artifact selection and directory assembly connect reusable rule interfaces."""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder


def invocation(project: Path, outputs: str) -> None:
    """Create an artifact-only composition with no executable nodes."""
    rule = project / "rules/artifacts"
    rule.mkdir(parents=True)
    (rule / "rule.toml").write_text(
        '[execution]\ntype = "graph"\n[execution.rules]\n[execution.outputs]\n'
        + outputs
    )
    (project / "invocation.toml").write_text('uses = "artifacts"\n')


def test_selected_tree_preserves_artifacts_and_cache(
    env_project: BakeryBuilder,
) -> None:
    """Selected files and subtrees preserve metadata; changes invalidate consuming rules."""
    project = env_project.project_dir
    source = project / "source-tree"
    source.mkdir()
    (source / "settings").write_text("first\n")
    (source / "settings").chmod(0o640)
    os.setxattr(source / "settings", "user.artifact-test", b"preserved")
    (source / "nested").mkdir()
    (source / "nested/a").write_text("linked\n")
    os.link(source / "nested/a", source / "nested/b")
    invocation(
        project,
        'tree = { tree = { "configuration/settings.txt" = { select = { '
        'source = { path = "source-tree" }, path = "settings", kind = "file" } }, '
        '"assets" = { select = { source = { path = "source-tree" }, '
        'path = "nested", kind = "directory" } } } }\n',
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/result"])
    tree = project / "build/result/tree"
    assert (tree / "configuration/settings.txt").read_text() == "first\n"
    assert (tree / "configuration/settings.txt").stat().st_mode & 0o777 == 0o640
    assert (
        os.getxattr(tree / "configuration/settings.txt", "user.artifact-test")
        == b"preserved"
    )
    assert (tree / "assets/a").stat().st_ino == (tree / "assets/b").stat().st_ino
    assert tree.stat().st_mtime_ns == 0
    assert (tree / "configuration").stat().st_mtime_ns == 0

    consumer = project / "rules/consume"
    consumer.mkdir()
    (consumer / "rule.toml").write_text(
        '[inputs]\ntree = "directory"\n[execution]\ntype = "run"\n'
        'command = ["/bin/sh", "-ec", "cat /run/rugix/bakery/work/inputs/tree/configuration/settings.txt '
        '> /run/rugix/bakery/work/outputs/value; head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token"]\n'
        '[execution.outputs.value]\nkind = "file"\npath = "value"\n'
        '[execution.outputs.token]\nkind = "file"\npath = "token"\n'
    )
    graph = project / "rules/artifacts/rule.toml"
    original = graph.read_text()
    source_binding = original.split("tree = ", 1)[1].strip()
    graph.write_text(
        '[execution]\ntype = "graph"\n[execution.rules.consume]\nuses = "consume"\n'
        f"inputs.tree = {source_binding}\n[execution.outputs]\n"
        'value = { rule = { node = "consume", output = "value" } }\n'
        'token = { rule = { node = "consume", output = "token" } }\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/result"])
    token = (project / "build/result/token").read_bytes()
    env_project.run(["bake", "rule", "invocation.toml", "build/result"])
    assert (project / "build/result/token").read_bytes() == token
    (source / "settings").write_text("changed\n")
    env_project.run(["bake", "rule", "invocation.toml", "build/result"])
    assert (project / "build/result/value").read_text() == "changed\n"
    assert (project / "build/result/token").read_bytes() != token


@pytest.mark.parametrize(
    "source",
    [
        '{ tree = { "../escape" = { path = "source-tree/settings" } } }',
        (
            '{ tree = { "dir" = { path = "source-tree" }, '
            '"dir/file" = { path = "source-tree/settings" } } }'
        ),
        (
            '{ tree = { "a/b" = { path = "source-tree" }, '
            '"a//b" = { path = "source-tree" } } }'
        ),
        '{ select = { source = { path = "source-tree" }, path = "../outside", kind = "file" } }',
        '{ select = { source = { path = "source-tree" }, path = "escape", kind = "file" } }',
        '{ select = { source = { path = "source-tree" }, path = "settings", kind = "directory" } }',
        '{ select = { source = { path = "source-tree/settings" }, path = "child", kind = "file" } }',
    ],
)
def test_invalid_artifact_paths_do_not_publish(
    env_project: BakeryBuilder, source: str
) -> None:
    """Reject traversal, overlapping normalized paths, escaped symlinks, and wrong kinds."""
    project = env_project.project_dir
    directory = project / "source-tree"
    directory.mkdir()
    (directory / "settings").write_text("configuration")
    (project / "outside").write_text("must not escape")
    (directory / "escape").symlink_to("../outside")
    invocation(project, "result = " + source + "\n")
    with pytest.raises(subprocess.CalledProcessError):
        env_project.run(["bake", "rule", "invocation.toml", "build/result"])
    assert not (project / "build/result").exists()


@pytest.mark.parametrize(
    ("name", "parameters", "extract"),
    [
        (
            "image-ext4",
            'label = "custom-data"\nuuid = "b32f1716-5925-4b29-a28c-b07ba91fa486"\n',
            "debugfs -R 'cat /configuration/settings.txt' /run/rugix/bakery/work/inputs/image",
        ),
        (
            "image-fat",
            'label = "CUSTOM"\nvolume-id = "1234abcd"\n',
            "mtype -i /run/rugix/bakery/work/inputs/image ::/configuration/settings.txt",
        ),
        (
            "image-squashfs",
            'compression = "xz"\nblock-size = 65536\n',
            "unsquashfs -cat /run/rugix/bakery/work/inputs/image configuration/settings.txt",
        ),
    ],
)
def test_filesystem_rules_accept_generic_trees(
    env_project: BakeryBuilder, name: str, parameters: str, extract: str
) -> None:
    """Each filesystem packages an arbitrary data tree and accepts format-specific options."""
    project = env_project.project_dir
    (project / "settings").write_text("arbitrary application data\n")
    (project / "invocation.toml").write_text(
        f'uses = "core/{name}"\n'
        'inputs.tree = { tree = { "configuration/settings.txt" = { path = "settings" } } }\n'
        "[parameters]\n" + parameters
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/image"])
    verifier = project / "rules/extract"
    verifier.mkdir(parents=True)
    (verifier / "rule.toml").write_text(
        '[inputs]\nimage = "file"\n[execution]\ntype = "run"\ncommand = '
        + json.dumps(
            ["/bin/sh", "-ec", extract + " > /run/rugix/bakery/work/outputs/value"]
        )
        + '\n[execution.outputs.value]\nkind = "file"\npath = "value"\n'
    )
    (project / "invocation.toml").write_text(
        'uses = "extract"\ninputs.image = { path = "build/image/image" }\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/extracted"])
    assert (
        project / "build/extracted/value"
    ).read_text() == "arbitrary application data\n"


@pytest.mark.parametrize("name", ["image-ext4", "image-fat"])
@pytest.mark.parametrize("content", ["empty", "large", "fixed"])
def test_filesystem_sizing(env_project: BakeryBuilder, name: str, content: str) -> None:
    """Auto sizing handles empty trees and growing data; explicit capacity remains available."""
    project = env_project.project_dir
    directory = project / "tree"
    directory.mkdir()
    if content == "large":
        with (directory / "large-application-data.bin").open("wb") as data:
            for _ in range(48):
                data.write(b"data" * (1024 * 256))
        for number in range(300):
            (directory / f"long-configuration-filename-{number:04}.txt").write_text(
                "setting"
            )
    invocation = f'uses = "core/{name}"\ninputs.tree = {{ path = "tree" }}\n'
    if content == "fixed":
        invocation += "parameters.size = 67108864\n"
    (project / "invocation.toml").write_text(invocation)
    env_project.run(["bake", "rule", "invocation.toml", "build/image"])
    image = project / "build/image/image"
    if content == "fixed":
        assert image.stat().st_size == 67108864
    if content != "large":
        return
    assert image.stat().st_size > 48 * 1024 * 1024
    extract = (
        "debugfs -R 'cat /large-application-data.bin' /run/rugix/bakery/work/inputs/image"
        if name == "image-ext4"
        else "mtype -i /run/rugix/bakery/work/inputs/image ::/large-application-data.bin"
    )
    verifier = project / "rules/digest"
    verifier.mkdir(parents=True)
    (verifier / "rule.toml").write_text(
        '[inputs]\nimage = "file"\n[execution]\ntype = "run"\ncommand = '
        + json.dumps(
            [
                "/bin/sh",
                "-ec",
                extract + " | sha256sum > /run/rugix/bakery/work/outputs/digest",
            ]
        )
        + '\n[execution.outputs.digest]\nkind = "file"\npath = "digest"\n'
    )
    (project / "invocation.toml").write_text(
        'uses = "digest"\ninputs.image = { path = "build/image/image" }\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/digest"])
    with (directory / "large-application-data.bin").open("rb") as data:
        expected = hashlib.file_digest(data, "sha256").hexdigest()
    assert (project / "build/digest/digest").read_text().split()[0] == expected
