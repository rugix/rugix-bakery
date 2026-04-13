"""Board definition data model."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any


@dataclass
class RawBlob:
    """Firmware blob written at a fixed disk offset."""

    yocto_deploy_file: str
    offset: str


@dataclass
class Partition:
    """Partition in the BSP disk layout."""

    name: str
    size: str | None = None
    type_uuid: str | None = None
    filesystem: str = "ext4"


@dataclass
class DiskLayout:
    """Complete disk layout for a board."""

    table_type: str = "gpt"
    raw_blobs: list[RawBlob] = field(default_factory=list)
    partitions: list[Partition] = field(default_factory=list)


@dataclass
class BootFlow:
    """Boot flow configuration."""

    type: str
    boot_cmd_template: str | None = None


@dataclass
class Board:
    """Definition of a board BSP."""

    name: str
    machine: str
    architecture: str
    boot_flow: BootFlow
    disk_layout: DiskLayout
    kas_repos: dict[str, Any]
    kas_includes: list[str] = field(default_factory=list)
    kas_local_conf: str = ""
    system_toml: str = ""
    bootstrapping_toml: str = ""
    extra_deploy_files: list[str] = field(default_factory=list)
    kas_targets: list[str] = field(default_factory=lambda: ["virtual/kernel"])
    kas_distro: str = "poky"
    kernel_recipe: str = "linux-yocto"
