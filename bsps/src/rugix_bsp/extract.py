"""Artifact extraction from Yocto deploy directory."""

from __future__ import annotations

import json
import shutil
import tarfile
from pathlib import Path

from rugix_bsp.archive import pack_bsp
from rugix_bsp.board import Board


def _find_file(deploy_dir: Path, name: str) -> Path | None:
    """Find a file in deploy_dir, following symlinks."""
    candidate = deploy_dir / name
    if candidate.exists():
        return candidate
    matches = list(deploy_dir.glob(f"{name}*"))
    return matches[0] if matches else None


def _find_modules_tar(deploy_dir: Path) -> Path | None:
    """Find the kernel modules tarball deployed by the kernel recipe."""
    for pattern in ["modules-*.tgz", "modules-*.tar.gz"]:
        matches = list(deploy_dir.glob(pattern))
        if matches:
            return matches[0]
    return None


def _extract_modules(modules_tar: Path, dest: Path) -> None:
    """Extract kernel modules from the kernel recipe's modules tarball."""
    dest.mkdir(parents=True, exist_ok=True)
    with tarfile.open(modules_tar) as tar:
        tar.extractall(dest, filter="data")


def extract_bsp(
    board: Board,
    deploy_dir: Path,
    output: Path,
) -> Path:
    """Extract BSP artifacts from a Yocto deploy dir and pack a BSP archive.

    Expects deploy_dir to contain artifacts from building individual recipes
    (virtual/kernel, imx-boot, virtual/bootloader, bakery-boot-script, etc.).
    """
    staging = output.parent / f".staging-{board.name}"
    if staging.exists():
        shutil.rmtree(staging)
    staging.mkdir(parents=True)

    boot_dir = staging / "boot"
    blobs_dir = staging / "blobs"
    boot_dir.mkdir()
    blobs_dir.mkdir()

    # Firmware blobs (imx-boot, idbloader.img, u-boot.itb, etc.).
    for blob in board.disk_layout.raw_blobs:
        src = _find_file(deploy_dir, blob.yocto_deploy_file)
        if src is None:
            raise FileNotFoundError(
                f"blob {blob.yocto_deploy_file!r} not found in {deploy_dir}"
            )
        shutil.copy2(src, blobs_dir / blob.yocto_deploy_file)

    # Kernel image (deployed by virtual/kernel).
    for kernel_name in ["Image", "zImage", "bzImage"]:
        src = _find_file(deploy_dir, kernel_name)
        if src is not None:
            shutil.copy2(src, boot_dir / kernel_name)
            break

    # Device trees (deployed by virtual/kernel).
    dtb_dir = boot_dir / "dtbs"
    dtb_dir.mkdir()
    for dtb in deploy_dir.glob("*.dtb"):
        if not dtb.is_symlink():
            shutil.copy2(dtb, dtb_dir / dtb.name)

    # Boot script (deployed by bakery-boot-script recipe).
    boot_scr = _find_file(deploy_dir, "boot.scr")
    if boot_scr is not None:
        shutil.copy2(boot_scr, boot_dir / "boot.scr")

    # Extra deploy files.
    for extra in board.extra_deploy_files:
        src = _find_file(deploy_dir, extra)
        if src is not None:
            shutil.copy2(src, boot_dir / extra)

    # Kernel modules (deployed by virtual/kernel as modules-<machine>.tgz).
    modules_tar = _find_modules_tar(deploy_dir)
    if modules_tar is not None:
        modules_dir = staging / "modules"
        _extract_modules(modules_tar, modules_dir)

    # Optional metadata from meta-bakery-bsp layer.
    metadata_file = deploy_dir / "bakery-bsp-metadata.json"
    if metadata_file.exists():
        with open(metadata_file) as f:
            metadata = json.load(f)
        _ = metadata

    result = pack_bsp(board, staging, output)
    shutil.rmtree(staging)
    return result
