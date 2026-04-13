"""BSP tar archive packing and unpacking."""

from __future__ import annotations

import tarfile
import tomllib
from pathlib import Path

from rugix_bsp.board import Board, BootFlow, DiskLayout, Partition, RawBlob


def _generate_bsp_toml(board: Board) -> str:
    """Generate bsp.toml content from a Board definition."""
    lines = [
        "[bsp]",
        f'name = "{board.name}"',
        f'architecture = "{board.architecture}"',
        "",
        "[boot-flow]",
        f'type = "{board.boot_flow.type}"',
        "",
        "[disk-layout]",
        f'table-type = "{board.disk_layout.table_type}"',
    ]

    for blob in board.disk_layout.raw_blobs:
        lines.append("")
        lines.append("[[disk-layout.raw-blobs]]")
        lines.append(f'file = "blobs/{blob.yocto_deploy_file}"')
        lines.append(f'offset = "{blob.offset}"')

    for part in board.disk_layout.partitions:
        lines.append("")
        lines.append("[[disk-layout.partitions]]")
        lines.append(f'name = "{part.name}"')
        if part.size is not None:
            lines.append(f'size = "{part.size}"')
        if part.type_uuid is not None:
            lines.append(f'type = "{part.type_uuid}"')
        lines.append(f'filesystem = "{part.filesystem}"')

    lines.append("")
    return "\n".join(lines)


def pack_bsp(board: Board, staging_dir: Path, output: Path) -> Path:
    """Pack a BSP staging directory into a tar.gz archive.

    The staging_dir should contain: boot/, blobs/, modules/ directories
    as populated by extract.py. This function adds bsp.toml, system.toml,
    and bootstrapping.toml from the Board definition.
    """
    bsp_toml = staging_dir / "bsp.toml"
    bsp_toml.write_text(_generate_bsp_toml(board))

    if board.system_toml:
        (staging_dir / "system.toml").write_text(board.system_toml)
    if board.bootstrapping_toml:
        (staging_dir / "bootstrapping.toml").write_text(board.bootstrapping_toml)

    output.parent.mkdir(parents=True, exist_ok=True)
    print(f"Packing BSP archive {output.name}...")
    with tarfile.open(output, "w:gz", compresslevel=1) as tar:
        tar.add(str(staging_dir), arcname=".", recursive=True)

    return output


def unpack_bsp(archive: Path, dest: Path) -> dict:
    """Unpack a BSP archive and return the parsed bsp.toml."""
    dest.mkdir(parents=True, exist_ok=True)
    with tarfile.open(archive, "r:gz") as tar:
        tar.extractall(dest, filter="data")

    bsp_toml = dest / "bsp.toml"
    if not bsp_toml.exists():
        raise FileNotFoundError(f"bsp.toml not found in {archive}")
    with open(bsp_toml, "rb") as f:
        return tomllib.load(f)


def load_bsp_metadata(bsp_toml_path: Path) -> tuple[Board, Path]:
    """Load a Board from an unpacked BSP's bsp.toml."""
    with open(bsp_toml_path, "rb") as f:
        data = tomllib.load(f)

    bsp = data["bsp"]
    bf = data["boot-flow"]
    dl = data["disk-layout"]

    raw_blobs = [
        RawBlob(yocto_deploy_file=Path(b["file"]).name, offset=b["offset"])
        for b in dl.get("raw-blobs", [])
    ]
    partitions = [
        Partition(
            name=p["name"],
            size=p.get("size"),
            type_uuid=p.get("type"),
            filesystem=p.get("filesystem", "ext4"),
        )
        for p in dl.get("partitions", [])
    ]

    bsp_dir = bsp_toml_path.parent
    system_toml = ""
    bootstrapping_toml = ""
    if (bsp_dir / "system.toml").exists():
        system_toml = (bsp_dir / "system.toml").read_text()
    if (bsp_dir / "bootstrapping.toml").exists():
        bootstrapping_toml = (bsp_dir / "bootstrapping.toml").read_text()

    board = Board(
        name=bsp["name"],
        machine="",
        architecture=bsp["architecture"],
        boot_flow=BootFlow(type=bf["type"]),
        disk_layout=DiskLayout(
            table_type=dl.get("table-type", "gpt"),
            raw_blobs=raw_blobs,
            partitions=partitions,
        ),
        kas_repos={},
        system_toml=system_toml,
        bootstrapping_toml=bootstrapping_toml,
    )

    return board, bsp_dir
