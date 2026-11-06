"""Reusable boot packaging accepts caller policy and preserves verification metadata."""

from __future__ import annotations

import json
import subprocess

import pytest
from rugix_testkit import BakeryBuilder


def test_rpi_boot_configuration(env_project: BakeryBuilder) -> None:
    """A composed text configuration reaches the signed tree without losing board settings."""
    project = env_project.project_dir
    firmware = project / "firmware"
    (firmware / "overlays").mkdir(parents=True)
    for name in ("start4.elf", "fixup4.dat", "board.dtb", "overlays/custom.dtbo"):
        (firmware / name).write_bytes(name.encode())
    for name in ("kernel", "initrd", "cmdline"):
        (project / name).write_text(name)
    rule = project / "rules/boot"
    rule.mkdir(parents=True)
    (rule / "rule.toml").write_text(
        '[inputs.boot]\nkind = "directory"\n'
        '[inputs.kernel]\nkind = "file"\n'
        '[inputs.initrd]\nkind = "file"\n'
        '[inputs.cmdline]\nkind = "file"\n'
        "[parameters.config]\n"
        '[execution]\ntype = "graph"\n'
        '[execution.rules.config]\nuses = "core/write-text"\n'
        'parameters.content = { parameter = "config" }\n'
        '[execution.rules.boot]\nuses = "core/prepare-rpi-boot"\n'
        'inputs.config = { rule = { node = "config", output = "file" } }\n'
        '[execution.outputs]\ntree = { rule = { node = "boot", output = "tree" } }\n'
    )
    config = "[all]\narm_64bit=1\nkernel=kernel8.img\ninitramfs initrd followkernel\ndtoverlay=custom\n"
    invocation = (
        'uses = "boot"\ninputs.boot = { path = "firmware" }\n'
        'inputs.kernel = { path = "kernel" }\n'
        'inputs.initrd = { path = "initrd" }\n'
        'inputs.cmdline = { path = "cmdline" }\n'
    )
    for uart in ("0", "1"):
        content = config + f"enable_uart={uart}\n"
        (project / "invoke.toml").write_text(
            invocation + f"parameters.config = {json.dumps(content)}\n"
        )
        env_project.run(["bake", "rule", "invoke.toml", f"build/uart-{uart}"])
        tree = project / f"build/uart-{uart}/tree"
        assert (tree / "config.txt").read_text() == content
        assert (tree / "kernel8.img").read_text() == "kernel"
        assert (tree / "initrd").read_text() == "initrd"
        assert (tree / "cmdline.txt").read_text() == "cmdline"
        assert (tree / "overlays/custom.dtbo").read_bytes() == b"overlays/custom.dtbo"


def test_efi_boot_timeout(env_project: BakeryBuilder) -> None:
    """ESP preparation applies the caller timeout while preserving UKI selection policy."""
    project = env_project.project_dir
    (project / "uki").write_bytes(b"signed uki")
    (project / "loader").write_bytes(b"signed loader")
    (project / "invoke.toml").write_text(
        'uses = "core/prepare-efi-esp"\n'
        'inputs.uki = { path = "uki" }\n'
        'inputs.bootloader = { path = "loader" }\n'
        'parameters.architecture = "amd64"\n'
        "parameters.timeout = 7\n"
    )
    env_project.run(["bake", "rule", "invoke.toml", "build/esp"])
    tree = project / "build/esp/tree"
    assert (tree / "loader/loader.conf").read_text() == (
        "default rugix-a.efi\ntimeout 7\neditor no\n"
    )
    assert (tree / "EFI/Linux/rugix-a.efi").read_bytes() == b"signed uki"


@pytest.mark.parametrize("panic_timeout", [0, 47])
def test_verified_command_line(env_project: BakeryBuilder, panic_timeout: int) -> None:
    """User arguments and panic timing remain configurable beside authenticated geometry."""
    project = env_project.project_dir
    for name, content in (
        ("hash", "a" * 64),
        ("offset", "1048576"),
        ("size", "1048576"),
    ):
        (project / name).write_text(content)
    invocation = (
        'uses = "core/verified-cmdline"\n'
        'inputs.root-hash = { path = "hash" }\n'
        'inputs.hash-offset = { path = "offset" }\n'
        'inputs.data-size = { path = "size" }\n'
        f"parameters.panic-timeout = {panic_timeout}\n"
        'parameters.extra = "console=ttyAMA0,115200 loglevel=4"\n'
    )
    (project / "invoke.toml").write_text(invocation)
    env_project.run(["bake", "rule", "invoke.toml", "build/cmdline"])
    arguments = (project / "build/cmdline/cmdline").read_text().split()
    assert f"panic={panic_timeout}" in arguments
    assert "console=ttyAMA0,115200" in arguments
    assert "loglevel=4" in arguments
    assert "rugix.verity-root-hash=" + "a" * 64 in arguments
    assert "rugix.verity-data-blocks=256" in arguments
    (project / "invoke.toml").write_text(
        invocation.replace("console=ttyAMA0,115200 loglevel=4", "root=/dev/unverified")
    )
    with pytest.raises(subprocess.CalledProcessError):
        env_project.run(["bake", "rule", "invoke.toml", "build/rejected"])
