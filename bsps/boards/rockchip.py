"""Rockchip board family definitions (Radxa, etc.)."""

from __future__ import annotations

from typing import Any

from rugix_bsp.board import Board, BootFlow, DiskLayout, Partition, RawBlob

# Rockchip disk layout reference (sector = 512 bytes):
#   loader1 (idbloader/SPL): offset 64 sectors = 32 KiB
#   loader2 (U-Boot proper): offset 16384 sectors = 8 MiB
# The first partition must start after the bootloader area.
# See https://opensource.rock-chips.com/wiki_Partitions

_SYSTEM_TOML = """\
[config-partition]
partition = 1
protected = true

[data-partition]
partition = 4

[boot-flow]
type = "uboot"

[boot-groups.a]
slots = { system = "system-a" }

[boot-groups.b]
slots = { system = "system-b" }

[slots.system-a]
type = "block"
partition = 2
immutable = true

[slots.system-b]
type = "block"
partition = 3
immutable = true
"""

_BOOTSTRAPPING_TOML = """\
[layout]
type = "gpt"
partitions = [
    { name = "config", size = "64MiB" },
    { name = "system-a", size = "4GiB" },
    { name = "system-b", size = "4GiB" },
    { name = "data", filesystem = { type = "ext4" } },
]
"""

_ROCKCHIP_LOCAL_CONF = """\
LICENSE_FLAGS_ACCEPTED += "commercial"
"""

# Base repos for Rockchip Yocto builds (scarthgap).
_ROCKCHIP_COMMON_REPOS: dict[str, Any] = {
    "poky": {
        "url": "https://git.yoctoproject.org/poky",
        "branch": "scarthgap",
        "layers": {"meta": None, "meta-poky": None, "meta-yocto-bsp": None},
    },
    "meta-openembedded": {
        "url": "https://github.com/openembedded/meta-openembedded.git",
        "branch": "scarthgap",
        "layers": {
            "meta-oe": None,
            "meta-python": None,
            "meta-networking": None,
        },
    },
    "meta-rockchip": {
        "url": "https://git.yoctoproject.org/meta-rockchip",
        "branch": "scarthgap",
    },
}


def rockchip_board(
    name: str,
    machine: str,
    *,
    idbloader_offset: str = "32KiB",
    uboot_offset: str = "8MiB",
    idbloader_file: str = "idbloader.img",
    uboot_file: str = "u-boot.itb",
    config_partition_size: str = "64MiB",
    extra_repos: dict[str, Any] | None = None,
    extra_local_conf: str = "",
    system_toml: str = _SYSTEM_TOML,
    bootstrapping_toml: str = _BOOTSTRAPPING_TOML,
) -> Board:
    """Factory for Rockchip boards."""
    repos = dict(_ROCKCHIP_COMMON_REPOS)
    if extra_repos:
        repos.update(extra_repos)

    local_conf = _ROCKCHIP_LOCAL_CONF
    if extra_local_conf:
        local_conf += extra_local_conf

    return Board(
        name=name,
        machine=machine,
        architecture="arm64",
        boot_flow=BootFlow(type="uboot"),
        disk_layout=DiskLayout(
            raw_blobs=[
                RawBlob(idbloader_file, idbloader_offset),
                RawBlob(uboot_file, uboot_offset),
            ],
            partitions=[
                Partition("config", config_partition_size, filesystem="fat32"),
                Partition("system-a"),
            ],
        ),
        kas_repos=repos,
        kas_targets=["virtual/kernel", "virtual/bootloader", "bakery-boot-script"],
        kas_local_conf=local_conf,
        system_toml=system_toml,
        bootstrapping_toml=bootstrapping_toml,
        kernel_recipe="linux-yocto",
    )


# --- Radxa boards ---

radxa_rock5b = rockchip_board("radxa-rock5b", "rock-5b-rk3588")
radxa_rock5a = rockchip_board("radxa-rock5a", "rock-5a-rk3588s")
radxa_zero3 = rockchip_board("radxa-zero3", "radxa-zero3-rk3566")
