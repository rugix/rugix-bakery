"""Clean bakes of a pinned Debian snapshot produce identical partitions."""

from __future__ import annotations

import filecmp
import os
import shutil
import subprocess
from pathlib import Path

import pytest

SYSTEM = "old-debian-snapshot"
RELEASE_ARGS = [
    "--release-version",
    "20250629165915",
    "--source-date",
    "2025-06-29T16:20:02Z",
]
PROJECT_FILES = ["rugix-bakery.toml", "run-bakery", "layers", "recipes"]

pytestmark = pytest.mark.extended


@pytest.fixture(scope="module")
def clean_bakes(
    rugix_project: Path, tmp_path_factory: pytest.TempPathFactory
) -> tuple[Path, Path]:
    """Filesystem directories of two bakes without shared caches."""
    bakes = (
        _bake_clean_copy(rugix_project, tmp_path_factory.mktemp("first")),
        _bake_clean_copy(rugix_project, tmp_path_factory.mktemp("second")),
    )
    for bake in bakes:
        for name in ("partition-2.tar", "partition-4.tar", "partition-4.img"):
            assert (bake / name).is_file(), f"bake is missing {name}"
    return bakes


def test_system_partition_is_reproducible(clean_bakes: tuple[Path, Path]) -> None:
    """The system partition's contents and filesystem are identical."""
    _assert_identical(clean_bakes, ["partition-4.tar", "partition-4.img"])


@pytest.mark.xfail(
    strict=True,
    reason="Bakery generates random partition IDs and writes the root PARTUUID "
    "into the GRUB environment on the boot partition.",
)
def test_boot_partition_is_reproducible(clean_bakes: tuple[Path, Path]) -> None:
    """The boot partition's contents are identical."""
    _assert_identical(clean_bakes, ["partition-2.tar"])


def _assert_identical(bakes: tuple[Path, Path], names: list[str]) -> None:
    _, mismatches, errors = filecmp.cmpfiles(*bakes, names, shallow=False)
    assert (mismatches, errors) == ([], [])


def _bake_clean_copy(project: Path, copy: Path) -> Path:
    for name in PROJECT_FILES:
        source = project / name
        if source.is_dir():
            shutil.copytree(source, copy / name)
        else:
            shutil.copy2(source, copy / name)
    env = {**os.environ, "RUGIX_DEV": "true", "RUGIX_CACHE_VOLUME": ""}
    subprocess.run(
        ["./run-bakery", "bake", "image", *RELEASE_ARGS, SYSTEM],
        cwd=copy,
        env=env,
        check=True,
    )
    return copy / "build" / SYSTEM / "filesystems"
