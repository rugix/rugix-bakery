export KAS_CONTAINER_ENGINE := env("KAS_CONTAINER_ENGINE", "podman")
export KAS_CONTAINER_IMAGE_DISTRO := "debian-bookworm"
export SSTATE_DIR := env("SSTATE_DIR", justfile_directory() + "/cache/sstate-cache")
export DL_DIR := env("DL_DIR", justfile_directory() + "/cache/downloads")

_uv_run := "uv run"
_uv_dev := "uv run --group dev"

[private]
_default:
    @just --list

# Build a BSP for a specific board (e.g., `just build nxp-imx91-frdm`).
build board *args:
    {{ _uv_run }} python -m rugix_bsp.cli build {{ board }} {{ args }}

# Build BSPs for all defined boards.
build-all *args:
    {{ _uv_run }} python -m rugix_bsp.cli build-all {{ args }}

# List all available board definitions.
list:
    {{ _uv_run }} python -m rugix_bsp.cli list

# Run type checker.
typecheck:
    {{ _uv_dev }} mypy src/

# Run linter.
lint:
    {{ _uv_dev }} ruff check src/ boards/
    {{ _uv_dev }} ruff format --check src/ boards/

# Auto-format code.
fmt:
    {{ _uv_dev }} ruff check --fix src/ boards/
    {{ _uv_dev }} ruff format src/ boards/

# Run all checks.
check: lint typecheck
