"""CLI entry point for rugix-bsp."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from rugix_bsp.board import Board

BOARDS_DIR = Path(__file__).resolve().parent.parent.parent / "boards"
META_BAKERY_BSP = Path(__file__).resolve().parent.parent.parent / "meta-bakery-bsp"


def _discover_boards() -> dict[str, Board]:
    """Import all board modules and collect Board instances."""
    import importlib
    import inspect

    sys.path.insert(0, str(BOARDS_DIR.parent))
    boards: dict[str, Board] = {}

    for py_file in sorted(BOARDS_DIR.glob("*.py")):
        if py_file.name.startswith("_"):
            continue
        module_name = f"boards.{py_file.stem}"
        module = importlib.import_module(module_name)
        for _name, obj in inspect.getmembers(module):
            if isinstance(obj, Board):
                boards[obj.name] = obj

    return boards


def cmd_list(args: argparse.Namespace) -> None:
    """List all available board definitions."""
    boards = _discover_boards()
    if not boards:
        print("No boards found.")
        return
    for name, board in sorted(boards.items()):
        blobs = ", ".join(
            f"{b.yocto_deploy_file} @ {b.offset}" for b in board.disk_layout.raw_blobs
        )
        print(f"  {name:<30} {board.machine:<25} {blobs}")


def cmd_build(args: argparse.Namespace) -> None:
    """Build a BSP for a specific board."""
    from rugix_bsp.extract import extract_bsp
    from rugix_bsp.kas import YoctoBuild

    boards = _discover_boards()
    board = boards.get(args.board)
    if board is None:
        print(f"Unknown board: {args.board}")
        print(f"Available: {', '.join(sorted(boards))}")
        sys.exit(1)

    work_dir = Path(args.work_dir)
    output_dir = Path(args.output_dir)

    build = YoctoBuild(
        board,
        work_dir=work_dir / board.name,
        meta_bakery_bsp=META_BAKERY_BSP,
    )

    print(f"Building Yocto for {board.name} (machine={board.machine})...")
    deploy_dir = build.build()

    output = output_dir / f"{board.name}.bsp.tar.gz"
    print(f"Extracting BSP artifacts from {deploy_dir}...")
    bsp = extract_bsp(board, deploy_dir, output)
    print(f"BSP archive: {bsp}")


def cmd_build_all(args: argparse.Namespace) -> None:
    """Build BSPs for all defined boards."""
    from rugix_bsp.extract import extract_bsp
    from rugix_bsp.kas import YoctoBuild

    boards = _discover_boards()
    if not boards:
        print("No boards found.")
        return

    work_dir = Path(args.work_dir)
    output_dir = Path(args.output_dir)

    for name, board in sorted(boards.items()):
        print(f"\n{'=' * 60}")
        print(f"Building BSP: {name} (machine={board.machine})")
        print(f"{'=' * 60}\n")

        build = YoctoBuild(
            board,
            work_dir=work_dir / board.name,
            meta_bakery_bsp=META_BAKERY_BSP,
        )

        deploy_dir = build.build()
        output = output_dir / f"{name}.bsp.tar.gz"
        bsp = extract_bsp(board, deploy_dir, output)
        print(f"BSP archive: {bsp}")

    print(f"\nAll {len(boards)} BSPs built.")


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(description="Rugix BSP builder")
    sub = parser.add_subparsers(dest="command", required=True)

    # list
    sub.add_parser("list", help="List available boards")

    # build
    p_build = sub.add_parser("build", help="Build BSP for a board")
    p_build.add_argument("board", help="Board name")
    p_build.add_argument("--work-dir", default="build", help="Working directory")
    p_build.add_argument("--output-dir", default="output", help="Output directory")

    # build-all
    p_all = sub.add_parser("build-all", help="Build BSPs for all boards")
    p_all.add_argument("--work-dir", default="build", help="Working directory")
    p_all.add_argument("--output-dir", default="output", help="Output directory")

    args = parser.parse_args(argv)

    if args.command == "list":
        cmd_list(args)
    elif args.command == "build":
        cmd_build(args)
    elif args.command == "build-all":
        cmd_build_all(args)


if __name__ == "__main__":
    main()
