"""Generated template keys match the configured signing and trust inputs."""

from __future__ import annotations

import shutil
import stat
import subprocess
import tomllib
from pathlib import Path

import pytest


def test_development_keys(tmp_path: Path) -> None:
    """Prepare a usable signing chain and SSH key without overwriting existing keys."""
    template = (
        Path(__file__).resolve().parents[1] / "container/templates/rpi-debian-secure"
    )
    project = tmp_path / "project with spaces"
    shutil.copytree(template, project)
    script = project / "prepare.sh"
    subprocess.run([str(script)], check=True, capture_output=True)
    keys = project / "keys"

    def openssl(*args: str) -> bytes:
        return subprocess.check_output(["openssl", *args], cwd=project)

    openssl("verify", "-CAfile", "keys/bundle-root.crt", "keys/bundle.crt")
    signer = openssl("x509", "-in", "keys/bundle.crt", "-text", "-noout")
    assert b"CA:FALSE" in signer
    assert openssl("pkey", "-in", "keys/bundle.key", "-pubout") == openssl(
        "x509", "-in", "keys/bundle.crt", "-pubkey", "-noout"
    )
    assert (
        openssl("pkey", "-in", "keys/boot.key", "-pubout")
        == (keys / "boot.pub").read_bytes()
    )
    assert b"2048 bit" in openssl("pkey", "-in", "keys/boot.key", "-text", "-noout")
    ssh = subprocess.check_output(["ssh-keygen", "-y", "-f", str(keys / "ssh")])
    assert (keys / "ssh.pub").read_text().startswith(ssh.decode().strip())
    config = tomllib.loads((project / "rugix-bakery.toml").read_text())
    for system in config["systems"].values():
        build = system["rules"]["release"]
        secrets = [
            ref
            for signer in build["rules"].values()
            for ref in signer["secrets"].values()
        ]
        for ref in [*build["inputs"].values(), *secrets]:
            assert (project / ref["path"]).is_file()
    for key in [*keys.glob("*.key"), keys / "ssh"]:
        assert stat.S_IMODE(key.stat().st_mode) == 0o600
    before = {p.name: p.read_bytes() for p in keys.iterdir()}
    with pytest.raises(subprocess.CalledProcessError):
        subprocess.run([str(script)], check=True, capture_output=True)
    assert {p.name: p.read_bytes() for p in keys.iterdir()} == before
    assert not list(project.glob(".keys.*"))


def test_setup_runs_after_dependencies() -> None:
    """Platform packages and SSH must be installed before template customization."""
    repository = Path(__file__).resolve().parents[1]
    template = repository / "container/templates/rpi-debian-secure"
    setup = tomllib.loads((template / "recipes/secure-setup/recipe.toml").read_text())

    recipes = repository / "container/repositories/core/recipes"
    for name in ["debian-rpi-verified-setup", "debian-verified-root", "ssh"]:
        dependency = tomllib.loads((recipes / name / "recipe.toml").read_text())
        assert setup["priority"] < dependency["priority"], name
    cleanup = tomllib.loads((recipes / "pkg-cleanup/recipe.toml").read_text())
    assert setup["priority"] > cleanup["priority"]
