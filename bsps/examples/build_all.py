#!/usr/bin/env python3
"""Build Rugix Bakery BSPs for all defined boards."""

from pathlib import Path

from rugix_bsp.extract import extract_bsp
from rugix_bsp.kas import YoctoBuild

from boards.nxp_imx import compulab_ucm_imx8mp, imx8mp_evk, imx91_frdm
from boards.rockchip import radxa_rock5a, radxa_rock5b

WORK_DIR = Path("build")
OUTPUT_DIR = Path("output")
BOOT_CMD = Path(__file__).resolve().parent.parent / "src" / "rugix_bsp" / "templates" / "uboot_boot.cmd"
META_BAKERY_BSP = Path(__file__).resolve().parent.parent / "meta-bakery-bsp"

ALL_BOARDS = [
    imx91_frdm,
    imx8mp_evk,
    compulab_ucm_imx8mp,
    radxa_rock5b,
    radxa_rock5a,
]


def main() -> None:
    for board in ALL_BOARDS:
        print(f"\n{'=' * 60}")
        print(f"Building BSP: {board.name} (machine={board.machine})")
        print(f"{'=' * 60}\n")

        build = YoctoBuild(board, work_dir=WORK_DIR / board.name, meta_bakery_bsp=META_BAKERY_BSP)
        deploy_dir = build.build()

        output = OUTPUT_DIR / f"{board.name}.bsp.tar.gz"
        bsp = extract_bsp(board, deploy_dir, output, boot_cmd=BOOT_CMD)
        print(f"BSP archive: {bsp}")

    print(f"\nAll {len(ALL_BOARDS)} BSPs built successfully.")


if __name__ == "__main__":
    main()
