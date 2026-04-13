"""NXP i.MX board family definitions."""

from __future__ import annotations

from typing import Any

from rugix_bsp.board import Board, BootFlow, DiskLayout, Partition, RawBlob

# Partition type UUID for Microsoft Basic Data — used for the FAT config
# partition so that Linux doesn't auto-mount it as an EFI System Partition.
_BASIC_DATA_TYPE = "EBD0A0A2-B9E5-4433-87C0-68B6B72699C7"

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
    { name = "boot", size = "64MiB", type = "EBD0A0A2-B9E5-4433-87C0-68B6B72699C7" },
    { name = "system-a", size = "4GiB" },
    { name = "system-b", size = "4GiB" },
    { name = "data", filesystem = { type = "ext4" } },
]
"""

# Common local.conf lines for NXP downstream BSP.
_NXP_LOCAL_CONF = """\
IMX_DEFAULT_BSP = "nxp"
LICENSE_FLAGS_ACCEPTED += "commercial"
ACCEPT_FSL_EULA = "1"
PREFERRED_PROVIDER_virtual/kernel = "linux-imx"
PREFERRED_PROVIDER_u-boot = "u-boot-imx"
PREFERRED_PROVIDER_virtual/bootloader = "u-boot-imx"
"""

# Repo pins matching NXP i.MX Yocto Release 6.6.36-2.1.0.
_NXP_COMMON_REPOS: dict[str, Any] = {
    "poky": {
        "url": "https://git.yoctoproject.org/poky",
        "commit": "f43f393ef0246b7bee6eed8bcf8271cf2b8cdf40",
        "layers": {"meta": None, "meta-poky": None, "meta-yocto-bsp": None},
    },
    "meta-openembedded": {
        "url": "https://github.com/openembedded/meta-openembedded.git",
        "commit": "80e01188fa822d87d301ee71973c462d7a865493",
        "layers": {
            "meta-oe": None,
            "meta-python": None,
            "meta-networking": None,
            "meta-multimedia": None,
            "meta-filesystems": None,
        },
    },
    "meta-clang": {
        "url": "https://github.com/kraj/meta-clang.git",
        "commit": "fe561f41aef0cff9e6f96730ab59f28dca2eb682",
    },
    "meta-arm": {
        "url": "https://git.yoctoproject.org/meta-arm",
        "commit": "1b85bbb4cab9658da3cd926c62038b8559c5c64e",
        "layers": {"meta-arm": None, "meta-arm-toolchain": None},
    },
    "meta-freescale": {
        "url": "https://github.com/Freescale/meta-freescale.git",
        "commit": "0f8091c63dd8805610c09b08409bc58492a3b16f",
    },
    "meta-freescale-3rdparty": {
        "url": "https://github.com/Freescale/meta-freescale-3rdparty.git",
        "commit": "6c063450d464eb2f380443c7d9af1b94ce9b9d75",
    },
    "meta-freescale-distro": {
        "url": "https://github.com/Freescale/meta-freescale-distro.git",
        "commit": "b9d6a5d9931922558046d230c1f5f4ef6ee72345",
    },
    "meta-imx": {
        "url": "https://github.com/nxp-imx/meta-imx.git",
        "tag": "rel_imx_6.6.36_2.1.0",
        "layers": {"meta-imx-bsp": None, "meta-imx-sdk": None},
    },
}


def imx_board(
    name: str,
    machine: str,
    *,
    imx_boot_seek: str = "32KiB",
    config_partition_size: str = "64MiB",
    extra_repos: dict[str, Any] | None = None,
    extra_local_conf: str = "",
    system_toml: str = _SYSTEM_TOML,
    bootstrapping_toml: str = _BOOTSTRAPPING_TOML,
) -> Board:
    """Factory for NXP i.MX boards."""
    repos = dict(_NXP_COMMON_REPOS)
    if extra_repos:
        repos.update(extra_repos)

    local_conf = _NXP_LOCAL_CONF
    if extra_local_conf:
        local_conf += extra_local_conf

    return Board(
        name=name,
        machine=machine,
        architecture="arm64",
        boot_flow=BootFlow(type="uboot"),
        disk_layout=DiskLayout(
            raw_blobs=[RawBlob("imx-boot", imx_boot_seek)],
            partitions=[
                Partition(
                    "config",
                    config_partition_size,
                    type_uuid=_BASIC_DATA_TYPE,
                    filesystem="fat32",
                ),
                Partition("system-a"),
            ],
        ),
        kas_repos=repos,
        kas_targets=["virtual/kernel", "imx-boot", "bakery-boot-script"],
        kas_local_conf=local_conf,
        system_toml=system_toml,
        bootstrapping_toml=bootstrapping_toml,
        kernel_recipe="linux-imx",
    )


# --- NXP EVK boards ---

imx91_frdm = imx_board(
    "nxp-imx91-frdm",
    "imx91frdm",
    extra_repos={
        "meta-imx-frdm": {
            "url": "https://github.com/nxp-imx-support/meta-imx-frdm.git",
            "branch": "lf-6.6.36-2.1.0",
            "layers": {"meta-imx-bsp": None, "meta-imx-sdk": None},
        },
    },
)

imx8mp_evk = imx_board("nxp-imx8mp-evk", "imx8mpevk")


# --- CompuLab i.MX boards ---

compulab_ucm_imx8mp = imx_board(
    "compulab-ucm-imx8mp",
    "ucm-imx8m-plus",
    extra_repos={
        "meta-compulab-bsp": {
            "url": "https://github.com/compulab-yokneam/meta-compulab-bsp.git",
            "branch": "kirkstone",
        },
        "meta-bsp-imx8mp": {
            "url": "https://github.com/compulab-yokneam/meta-bsp-imx8mp.git",
            "branch": "kirkstone",
        },
    },
)
