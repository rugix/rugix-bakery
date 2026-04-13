"""KAS project generation and Yocto build execution."""

from __future__ import annotations

import os
import shutil
import subprocess
from pathlib import Path
from typing import Any

from rugix_bsp.board import Board

TEMPLATES_DIR = Path(__file__).resolve().parent / "templates"


def generate_kas_config(
    board: Board, meta_bakery_bsp_rel: str | None = None
) -> dict[str, Any]:
    """Generate a KAS YAML configuration dict from a Board definition."""
    config: dict[str, Any] = {
        "header": {"version": 14},
        "machine": board.machine,
        "distro": board.kas_distro,
        "target": list(board.kas_targets),
    }

    if board.kas_includes:
        config["header"]["includes"] = board.kas_includes

    local_conf_parts: dict[str, str] = {}

    if board.kas_local_conf:
        local_conf_parts["board-conf"] = board.kas_local_conf

    config["local_conf_header"] = local_conf_parts

    repos: dict[str, Any] = {}
    for name, repo_def in board.kas_repos.items():
        repos[name] = repo_def

    if meta_bakery_bsp_rel is not None:
        repos["meta-bakery-bsp"] = {
            "path": "/work/meta-bakery-bsp",
        }

    config["repos"] = repos

    return config


def write_kas_config(config: dict[str, Any], path: Path) -> Path:
    """Write a KAS configuration dict to a YAML file."""
    import yaml

    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w") as f:
        yaml.dump(config, f, default_flow_style=False, sort_keys=False)
    return path


class YoctoBuild:
    """Drives a KAS-based Yocto build to produce BSP artifacts.

    Respects the standard Yocto environment variables DL_DIR and SSTATE_DIR
    for download and shared-state caches. Set them in the environment (e.g.,
    via the justfile) before invoking a build.

    Directory layout per board:
        <work_dir>/
          kas-<board>.yaml              # Generated KAS config
          _kas/                         # KAS_WORK_DIR
            meta-bakery-bsp/            # Copied so it's inside the container mount
          build/                        # KAS_BUILD_DIR
            tmp/deploy/images/<machine>
    """

    def __init__(
        self,
        board: Board,
        work_dir: Path,
        meta_bakery_bsp: Path | None = None,
    ) -> None:
        self.board = board
        self.work_dir = work_dir.resolve()
        self.meta_bakery_bsp = meta_bakery_bsp.resolve() if meta_bakery_bsp else None

        self.work_dir.mkdir(parents=True, exist_ok=True)

    @property
    def kas_work_dir(self) -> Path:
        return self.work_dir / "_kas"

    @property
    def kas_config_path(self) -> Path:
        return self.work_dir / f"kas-{self.board.name}.yaml"

    @property
    def build_dir(self) -> Path:
        return self.work_dir / "build"

    @property
    def deploy_dir(self) -> Path:
        return self.build_dir / "tmp" / "deploy" / "images" / self.board.machine

    def _setup_meta_bakery_bsp(self, boot_cmd: Path | None = None) -> str | None:
        """Copy meta-bakery-bsp into the KAS work dir.

        Also copies the board's boot.cmd into the boot-script recipe, and
        generates a kernel bbappend that applies the rugix.cfg fragment.
        """
        if self.meta_bakery_bsp is None:
            return None
        dest = self.kas_work_dir / "meta-bakery-bsp"
        if dest.exists():
            shutil.rmtree(dest)
        shutil.copytree(self.meta_bakery_bsp, dest)

        # Place boot.cmd in the recipe's files directory.
        if boot_cmd is None:
            boot_cmd = TEMPLATES_DIR / "uboot_boot.cmd"
        files_dir = dest / "recipes-bsp" / "bakery-boot-script" / "files"
        files_dir.mkdir(parents=True, exist_ok=True)
        shutil.copy2(boot_cmd, files_dir / "boot.cmd")

        # Generate kernel bbappend for the board's kernel recipe.
        kernel_dir = dest / "recipes-kernel" / "linux"
        kernel_dir.mkdir(parents=True, exist_ok=True)
        bbappend = kernel_dir / f"{self.board.kernel_recipe}_%.bbappend"
        bbappend.write_text(
            'FILESEXTRAPATHS:prepend := "${THISDIR}/files:"\n'
            'SRC_URI:append = " file://rugix.cfg"\n'
        )

        return "_kas/meta-bakery-bsp"

    def generate_config(self, boot_cmd: Path | None = None) -> Path:
        """Generate the KAS YAML configuration for this board."""
        self.kas_work_dir.mkdir(parents=True, exist_ok=True)
        meta_rel = self._setup_meta_bakery_bsp(boot_cmd)
        config = generate_kas_config(self.board, meta_bakery_bsp_rel=meta_rel)
        return write_kas_config(config, self.kas_config_path)

    def build(self, boot_cmd: Path | None = None) -> Path:
        """Run the Yocto build via kas-container and return the deploy directory."""
        config_path = self.generate_config(boot_cmd)

        env = {
            **os.environ,
            "KAS_WORK_DIR": str(self.kas_work_dir),
            "KAS_BUILD_DIR": str(self.build_dir),
        }

        subprocess.run(
            ["kas-container", "build", str(config_path)],
            check=True,
            env=env,
        )
        return self.deploy_dir
