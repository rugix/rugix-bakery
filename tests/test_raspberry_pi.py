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

UBOOT_FILES = {
    "boot.scr",
    "bootpart.default.env",
    "boot_spare.disabled.env",
    "boot_spare.enabled.env",
}


@pytest.fixture(autouse=True)
def _require_mtools() -> None:
    if shutil.which("mdir") is None:
        pytest.fail("mtools are required to inspect FAT partitions")


def test_tryboot_selects_boot_partitions(bakery: BakeryBuilder) -> None:
    """Tryboot boots partition 2 for group A and partition 3 for group B."""
    config = FatPartition(bakery.bake_image("tryboot"), CONFIG_PARTITION)

    assert _boot_partitions(config.read("autoboot.txt")) == ("2", "3")
    assert _boot_partitions(config.read("autoboot.a.txt")) == ("2", "3")
    assert _boot_partitions(config.read("autoboot.b.txt")) == ("3", "2")
    assert "pieeprom.upd" not in config.files()


def test_tryboot_includes_requested_eeprom_update(bakery: BakeryBuilder) -> None:
    """The ``core/rpi-include-firmware`` recipe adds a signed EEPROM update."""
    config = FatPartition(bakery.bake_image("tryboot-pi4"), CONFIG_PARTITION)

    assert {"pieeprom.upd", "pieeprom.sig", "recovery.bin"} <= config.files()
    assert _boot_partitions(config.read("autoboot.txt")) == ("2", "3")


@pytest.mark.parametrize("system", ["uboot-arm64", "uboot-armhf"])
def test_uboot_installs_referenced_binaries(bakery: BakeryBuilder, system: str) -> None:
    """Every U-Boot binary referenced by ``config.txt`` exists next to the scripts."""
    image = bakery.bake_image(system)
    config = FatPartition(image, CONFIG_PARTITION)
    boot = FatPartition(image, BOOT_A_PARTITION)

    kernels = {
        line.removeprefix("kernel=")
        for line in config.read("config.txt").splitlines()
        if line.startswith("kernel=")
    }
    assert kernels
    assert kernels | UBOOT_FILES <= config.files()
    assert "second.scr" in boot.files()


class FatPartition:
    """A FAT partition of a disk image with an MBR partition table."""

    def __init__(self, image: Path, number: int) -> None:
        with image.open("rb") as file:
            mbr = file.read(512)
        assert mbr[510:512] == b"\x55\xaa", f"{image} has no MBR"
        (start,) = struct.unpack_from("<I", mbr, 446 + 16 * (number - 1) + 8)
        self._target = f"{image}@@{start * 512}"

    def files(self) -> set[str]:
        output = self._mtools("mdir", "-b", "::/")
        return {line.removeprefix("::/") for line in output.splitlines()}

    def read(self, name: str) -> str:
        return self._mtools("mtype", f"::/{name}")

    def _mtools(self, command: str, *args: str) -> str:
        result = subprocess.run(
            [command, "-i", self._target, *args],
            check=True,
            capture_output=True,
            text=True,
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
