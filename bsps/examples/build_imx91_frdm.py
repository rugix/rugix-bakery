#!/usr/bin/env python3
"""Build a Rugix Bakery BSP for the NXP i.MX 91 FRDM evaluation kit."""

from pathlib import Path

from rugix_bsp.extract import extract_bsp
from rugix_bsp.kas import YoctoBuild

from boards.nxp_imx import imx91_frdm

WORK_DIR = Path("build")
OUTPUT = Path("output") / f"{imx91_frdm.name}.bsp.tar.gz"
BOOT_CMD = Path(__file__).resolve().parent.parent / "src" / "rugix_bsp" / "templates" / "uboot_boot.cmd"
META_BAKERY_BSP = Path(__file__).resolve().parent.parent / "meta-bakery-bsp"


def main() -> None:
    build = YoctoBuild(imx91_frdm, work_dir=WORK_DIR, meta_bakery_bsp=META_BAKERY_BSP)

    print(f"Building Yocto for {imx91_frdm.name} (machine={imx91_frdm.machine})...")
    deploy_dir = build.build()

    print(f"Extracting BSP artifacts from {deploy_dir}...")
    bsp = extract_bsp(imx91_frdm, deploy_dir, OUTPUT, boot_cmd=BOOT_CMD)
    print(f"BSP archive: {bsp}")


if __name__ == "__main__":
    main()
