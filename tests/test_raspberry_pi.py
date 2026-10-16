"""Images for Raspberry Pi targets contain what their boot flows need.

The systems use a minimal layer without binaries, so baking them works on any
host without emulating the target architecture. Reading the FAT partitions
requires mtools.
"""

from __future__ import annotations

import shutil
import struct
import subprocess
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder

CONFIG_PARTITION = 1
BOOT_A_PARTITION = 2
# Rugix Ctrl's system partition of group A in the default MBR layout.
SYSTEM_A_PARTITION = 5

UBOOT_FILES = {
    "boot.scr",
    "bootpart.default.env",
    "boot_spare.disabled.env",
    "boot_spare.enabled.env",
    "bootcode.bin",
    "start.elf",
    "fixup.dat",
}


@pytest.fixture(autouse=True)
def _require_mtools() -> None:
    if shutil.which("mdir") is None:
        pytest.fail("mtools are required to inspect FAT partitions")


def test_tryboot_selects_boot_partitions(bakery: BakeryBuilder) -> None:
    """Tryboot boots partition 2 for group A and partition 3 for group B, which
    starts Rugix Ctrl as init on the system partition of group A."""
    image = bakery.bake_image("tryboot")
    config = FatPartition(image, CONFIG_PARTITION)

    assert _boot_partitions(config.read("autoboot.txt")) == ("2", "3")
    assert _boot_partitions(config.read("autoboot.a.txt")) == ("2", "3")
    assert _boot_partitions(config.read("autoboot.b.txt")) == ("3", "2")
    assert "pieeprom.upd" not in config.files()
    _assert_boots_system_a(image)


def test_tryboot_includes_requested_eeprom_update(bakery: BakeryBuilder) -> None:
    """The ``core/rpi-include-firmware`` recipe adds a signed EEPROM update."""
    config = FatPartition(bakery.bake_image("tryboot-pi4"), CONFIG_PARTITION)

    assert {"pieeprom.upd", "pieeprom.sig", "recovery.bin"} <= config.files()


@pytest.mark.parametrize("system", ["uboot-arm64", "uboot-armhf"])
def test_uboot_installs_firmware_and_scripts(
    bakery: BakeryBuilder, system: str
) -> None:
    """The config partition contains the firmware, every U-Boot binary that
    ``config.txt`` references, and the boot scripts. U-Boot starts Rugix Ctrl
    as init on the system partition of group A."""
    image = bakery.bake_image(system)
    config = FatPartition(image, CONFIG_PARTITION)

    kernels = {
        line.removeprefix("kernel=")
        for line in config.read("config.txt").splitlines()
        if line.startswith("kernel=")
    }
    assert kernels
    assert kernels | UBOOT_FILES <= config.files()
    assert "second.scr" in FatPartition(image, BOOT_A_PARTITION).files()
    _assert_boots_system_a(image)


def _assert_boots_system_a(image: Path) -> None:
    """Assert the kernel command line in ``cmdline.txt`` and ``boot.env``."""
    with image.open("rb") as file:
        (disk_id,) = struct.unpack("<I", file.read(512)[440:444])
    root = f"root=partuuid={disk_id:08x}-{SYSTEM_A_PARTITION:02d}"
    boot = FatPartition(image, BOOT_A_PARTITION)
    cmdline = boot.read("cmdline.txt").lower()
    assert root in cmdline.split()
    assert "init=/usr/bin/rugix-ctrl" in cmdline.split()
    assert f"bootargs={cmdline.strip()}" in boot.read("boot.env").lower()


class FatPartition:
    """A FAT partition of a disk image with an MBR partition table."""

    def __init__(self, image: Path, number: int) -> None:
        with image.open("rb") as file:
            mbr = file.read(512)
        assert mbr[510:512] == b"\x55\xaa", f"{image} has no MBR"
        (start,) = struct.unpack_from("<I", mbr, 446 + 16 * (number - 1) + 8)
        self._target = f"{image}@@{start * 512}"

    def files(self) -> set[str]:
        output = self._mtools("mdir", "-b", "::/").decode()
        return {line.removeprefix("::/") for line in output.splitlines()}

    def read(self, name: str) -> str:
        """Return the file's content. Binary content is decoded leniently."""
        return self._mtools("mtype", f"::/{name}").decode(errors="replace")

    def _mtools(self, command: str, *args: str) -> bytes:
        result = subprocess.run(
            [command, "-i", self._target, *args], check=True, capture_output=True
        )
        return result.stdout


def _boot_partitions(autoboot: str) -> tuple[str, str]:
    """Return the default and tryboot partitions of an ``autoboot.txt``."""
    partitions: dict[str, str] = {}
    section = ""
    for line in autoboot.splitlines():
        if line.startswith("["):
            section = line.strip("[]")
        elif line.startswith("boot_partition="):
            partitions[section] = line.removeprefix("boot_partition=")
    return partitions["all"], partitions["tryboot"]
