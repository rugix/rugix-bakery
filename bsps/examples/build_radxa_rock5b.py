#!/usr/bin/env python3
"""Build a Rugix Bakery BSP for the Radxa Rock 5B."""

from pathlib import Path

from rugix_bsp.extract import extract_bsp
from rugix_bsp.kas import YoctoBuild

from boards.rockchip import radxa_rock5b

WORK_DIR = Path("build")
OUTPUT = Path("output") / f"{radxa_rock5b.name}.bsp.tar.gz"
BOOT_CMD = Path(__file__).resolve().parent.parent / "src" / "rugix_bsp" / "templates" / "uboot_boot.cmd"
META_BAKERY_BSP = Path(__file__).resolve().parent.parent / "meta-bakery-bsp"


def main() -> None:
    build = YoctoBuild(radxa_rock5b, work_dir=WORK_DIR, meta_bakery_bsp=META_BAKERY_BSP)

    print(f"Building Yocto for {radxa_rock5b.name} (machine={radxa_rock5b.machine})...")
    deploy_dir = build.build()

    print(f"Extracting BSP artifacts from {deploy_dir}...")
    bsp = extract_bsp(radxa_rock5b, deploy_dir, OUTPUT, boot_cmd=BOOT_CMD)
    print(f"BSP archive: {bsp}")


if __name__ == "__main__":
    main()
