"""The predefined system pipeline uses cached rules for images and bundles."""

from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import time
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder

RELEASE = [
    "--release-version",
    "pipeline-test",
    "--source-date",
    "2026-10-26T00:00:00Z",
]


def prepare(project: Path) -> None:
    """Supply a BSP layout and raw firmware as outputs of an ordinary layer."""
    (project / "layers/unfinished.toml").unlink()
    (project / "layers/base.toml").write_text('root = true\nrecipes = ["base"]\n')
    recipe = project / "recipes/base"
    (recipe / "steps").mkdir(parents=True)
    (recipe / "recipe.toml").write_text('description = "Pipeline fixture"\n')
    (recipe / "bsp.toml").write_text(
        """[image.layout]
type = "gpt"
[[image.layout.raw-blobs]]
file = "bsp/firmware"
offset = 32768
size = 4096
[[image.layout.partitions]]
name = "system"
root = "system"
filesystem = { type = "ext4" }
[[image.layout.partitions]]
name = "extra"
root = "extra"
size = "64MiB"
filesystem = { type = "fat32" }
[[bundle.payloads]]
slot = "system"
partition = 1
[[bundle.payloads]]
slot = "boot"
file = "kernel.itb"
"""
    )
    script = recipe / "steps/00-run.sh"
    script.write_text(
        """#!/bin/sh
set -eu
mkdir -p "$RUGIX_ROOT_DIR/etc" "$RUGIX_LAYER_DIR/bsp" "$RUGIX_LAYER_DIR/roots/extra" "$RUGIX_LAYER_DIR/artifacts"
printf signed-fit > "$RUGIX_LAYER_DIR/artifacts/kernel.itb"
printf original > "$RUGIX_ROOT_DIR/etc/message"
printf firmware > "$RUGIX_LAYER_DIR/bsp/firmware"
printf data > "$RUGIX_LAYER_DIR/roots/extra/value"
cp "$RECIPE_DIR/bsp.toml" "$RUGIX_LAYER_DIR/bsp/rugix-bsp.toml"
"""
    )
    script.chmod(0o755)
    with (project / "rugix-bakery.toml").open("a") as config:
        config.write(
            '\n[systems.device]\nlayer = "base"\narchitecture = "amd64"\ntarget = "bsp"\n'
        )


def digest(path: Path) -> str:
    """Hash sparse images without retaining their contents in memory."""
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def test_standard_pipeline_rebuilds_and_reproduces(
    env_project: BakeryBuilder, tmp_path: Path
) -> None:
    """BSP discovery, firmware placement, caches, and clean image rebuilds compose."""
    project = env_project.project_dir
    prepare(project)
    env_project.run(["bake", "image", "device", *RELEASE])
    output = project / "build/device"
    with (output / "system.img").open("rb") as image:
        image.seek(32768)
        assert image.read(8) == b"firmware"
    assert (output / "filesystems/partition-1.tar").is_file()
    assert (output / "filesystems/partition-2.img").is_file()
    info = json.loads((output / "system-build-info.json").read_text())
    assert info["release"]["version"] == "pipeline-test"
    original = digest(output / "system.img")
    env_project.run(["bake", "image", "device", *RELEASE])
    assert digest(output / "system.img") == original

    second = tmp_path / "independent"
    shutil.copytree(
        project,
        second,
        ignore=shutil.ignore_patterns(".rugix", "build", "envs", "vendor"),
    )
    shutil.copytree(project / "vendor", second / "vendor")
    other = BakeryBuilder(second, tmp_path / "independent-run")
    other.run(["bake", "image", "device", *RELEASE])
    assert digest(second / "build/device/system.img") == original
    for name in ("sbom.spdx.json", "sbom.cdx.json"):
        assert (second / "build/device" / name).read_bytes() == (
            output / name
        ).read_bytes()

    script = project / "recipes/base/steps/00-run.sh"
    script.write_text(script.read_text().replace("printf original", "printf changed"))
    env_project.run(["bake", "image", "device", *RELEASE])
    assert digest(output / "system.img") != original
    changed = digest(output / "system.img")

    config = project / "rugix-bakery.toml"
    config.write_text(
        config.read_text() + '\n[systems.device.image]\nsize = "256MiB"\n'
    )
    env_project.run(["bake", "image", "device", *RELEASE])
    assert (output / "system.img").stat().st_size == 256 * 1024 * 1024
    assert digest(output / "system.img") != changed

    env_project.run(
        [
            "bake",
            "image",
            "device",
            "--release-version",
            "pipeline-test",
            "--source-date",
            "2026-10-27T00:00:00Z",
        ]
    )
    assert digest(output / "system.img") != changed
    assert (
        json.loads((output / "build-context.json").read_text())["source-date-epoch"]
        == 1793059200
    )
    successful = digest(output / "system.img")
    config.write_text(config.read_text().replace('size = "256MiB"', 'size = "1MiB"'))
    with pytest.raises(subprocess.CalledProcessError):
        env_project.run(["bake", "image", "device", *RELEASE])
    assert digest(output / "system.img") == successful


def test_standard_bundle_uses_graph_payloads(env_project: BakeryBuilder) -> None:
    """Bundle construction uses the same filesystem artifacts and honors CLI encoding."""
    project = env_project.project_dir
    prepare(project)
    env_project.run(
        [
            "bake",
            "bundle",
            "device",
            "--release-version",
            "pipeline-test",
            "--disable-compression",
        ]
    )
    output = project / "build/device"
    assert (output / "system.rugixb").is_file()
    assert (output / "system.rugixb-hash").is_file()
    env_project.run(
        ["bundler", "extract", "build/device/system.rugixb", "1", "fit-extracted"]
    )
    assert (project / "fit-extracted").read_bytes() == b"signed-fit"
    # The public command also exports an already assembled bundle to another path.
    env_project.run(
        ["bake", "bundle", "device", "export.rugixb", "--disable-compression"]
    )
    assert digest(project / "export.rugixb") == digest(output / "system.rugixb")


def test_implicit_release_date_survives_noop_builds(
    env_project: BakeryBuilder, capfd: pytest.CaptureFixture[str]
) -> None:
    """An unchanged layer retains its implicit release date and cached image artifacts."""
    project = env_project.project_dir
    prepare(project)
    env_project.run(["bake", "image", "device"])
    output = project / "build/device"
    original = digest(output / "system.img")
    context = (output / "build-context.json").read_bytes()
    capfd.readouterr()
    time.sleep(2)
    env_project.run(["bake", "image", "device"])
    assert 'Reusing rule "image-ext4"' in capfd.readouterr().err
    assert (output / "build-context.json").read_bytes() == context
    assert digest(output / "system.img") == original


def test_dynamic_fat_layout_counts_hardlink_copies(env_project: BakeryBuilder) -> None:
    """FAT geometry reserves separate space for files sharing a source inode."""
    project = env_project.project_dir
    tree = project / "fat-tree"
    tree.mkdir()
    with (tree / "first").open("wb") as source:
        source.truncate(100 * 1024 * 1024)
    (tree / "second").hardlink_to(tree / "first")
    (project / "layout.toml").write_text(
        'type = "gpt"\npartitions = [{ root = "data", filesystem = { type = "fat32" } }]\n'
    )
    (project / "layout-rule.toml").write_text(
        'uses = "core/image-layout"\ninputs.layout = { file = "layout.toml" }\n'
        'inputs.roots = { tree = { data = { file = "fat-tree" } } }\n'
        "inputs.layer = { tree = {} }\n"
    )
    env_project.run(["bake", "rule", "layout-rule.toml", "build/layout"])
    assert (project / "build/layout/image").stat().st_size > 200 * 1024 * 1024


def test_nested_boot_root_uses_prepared_files(env_project: BakeryBuilder) -> None:
    """A partition sourced from config/EFI includes initialized bootloader files."""
    project = env_project.project_dir
    prepare(project)
    config = project / "rugix-bakery.toml"
    config.write_text(
        config.read_text().replace('target = "bsp"', 'target = "generic-grub-efi"')
        + """
[systems.device.image.layout]
type = "gpt"
[[systems.device.image.layout.partitions]]
root = "config/EFI"
filesystem = { type = "fat32" }
[[systems.device.image.layout.partitions]]
size = "16MiB"
[[systems.device.image.layout.partitions]]
size = "16MiB"
[[systems.device.image.layout.partitions]]
root = "system"
filesystem = { type = "ext4" }
"""
    )
    env_project.run(["bake", "image", "device", *RELEASE])
    rule = project / "rules/extract"
    rule.mkdir(parents=True)
    (rule / "rule.toml").write_text(
        '[inputs]\nimage = "file"\n[execution]\ntype = "run"\n'
        'command = ["mcopy", "-i", "/run/rugix/bakery/work/inputs/image", "::/BOOT/BOOTX64.EFI", "/run/rugix/bakery/work/outputs/efi"]\n'
        '[execution.outputs.efi]\nkind = "file"\npath = "efi"\n'
    )
    (project / "extract.toml").write_text(
        'uses = "extract"\ninputs.image = { file = "build/device/filesystems/partition-1.img" }\n'
    )
    env_project.run(["bake", "rule", "extract.toml", "build/extracted"])
    assert (project / "build/extracted/efi").read_bytes().startswith(b"MZ")


def test_small_fat_partition_dereferences_boot_links(
    env_project: BakeryBuilder,
) -> None:
    """Legacy fixed FAT partitions accept symlinked boot files and select a fitting format."""
    project = env_project.project_dir
    prepare(project)
    layout = project / "recipes/base/bsp.toml"
    layout.write_text(layout.read_text().replace('size = "64MiB"', 'size = "16MiB"'))
    script = project / "recipes/base/steps/00-run.sh"
    with script.open("a") as source:
        source.write('ln -s value "$RUGIX_LAYER_DIR/roots/extra/kernel.img"\n')
    env_project.run(["bake", "image", "device", *RELEASE])
    rule = project / "rules/extract"
    rule.mkdir(parents=True)
    (rule / "rule.toml").write_text(
        '[inputs]\nimage = "file"\n[execution]\ntype = "run"\n'
        'command = ["mcopy", "-i", "/run/rugix/bakery/work/inputs/image", "::/kernel.img", "/run/rugix/bakery/work/outputs/kernel"]\n'
        '[execution.outputs.kernel]\nkind = "file"\npath = "kernel"\n'
    )
    (project / "extract.toml").write_text(
        'uses = "extract"\ninputs.image = { file = "build/device/filesystems/partition-2.img" }\n'
    )
    env_project.run(["bake", "rule", "extract.toml", "build/extracted"])
    assert (project / "build/extracted/kernel").read_bytes() == b"data"
    assert (
        project / "build/device/filesystems/partition-2.img"
    ).stat().st_size == 16 * 1024 * 1024
