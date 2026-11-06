"""EFI setup renders bootstrap and slot mappings for both config layouts."""

from __future__ import annotations

import os
import subprocess
import tomllib
from pathlib import Path

import pytest


@pytest.mark.parametrize("secondary", [False, True])
def test_efi_config_layout(tmp_path: Path, secondary: bool) -> None:
    """Rendered bootstrap partitions agree with the system's slots and data mount."""
    recipe = (
        Path(__file__).resolve().parents[1]
        / "container/repositories/core/recipes/debian-efi-verified-setup"
    )
    inputs = tmp_path / "inputs"
    inputs.mkdir()
    for name, source in [
        ("system-config", "system.toml"),
        ("bootstrap-config", "bootstrap.toml.in"),
    ]:
        (inputs / name).write_bytes((recipe / "files" / source).read_bytes())
    tools = tmp_path / "bin"
    tools.mkdir()
    apt = tools / "apt-get"
    apt.write_text("#!/bin/sh\nexit 0\n")
    apt.chmod(0o755)
    config = tmp_path / "config"
    script = (
        (recipe / "steps/00-install.sh").read_text().replace("/etc/rugix", str(config))
    )
    subprocess.run(
        ["sh", "-eu", "-c", script],
        check=True,
        env={
            **os.environ,
            "PATH": f"{tools}:{os.environ['PATH']}",
            "RUGIX_ARCH": "amd64",
            "RECIPE_INPUTS_DIR": str(inputs),
            "RECIPE_PARAM_SECONDARY_CONFIG": str(secondary).lower(),
            "RECIPE_PARAM_SYSTEM_SIZE": "4294967296",
        },
    )
    system = tomllib.loads((config / "system.toml").read_text())
    bootstrap = tomllib.loads((config / "bootstrapping.toml").read_text())
    partitions = bootstrap["layout"]["partitions"]
    names = ["config", "system-a", "system-b", "data"]
    if secondary:
        names.insert(1, "config-secondary")
        assert partitions[1]["size"] == 0
        assert "filesystem" not in partitions[1]
    assert [p["name"] for p in partitions] == names
    for slot in ["system-a", "system-b"]:
        number = system["slots"][slot]["partition"]
        assert partitions[number - 1]["name"] == slot
        assert partitions[number - 1]["size"] == 4294967296
    assert partitions[system["data-partition"]["partition"] - 1]["name"] == "data"
    assert system["config-partition"]["partition"] == 1
