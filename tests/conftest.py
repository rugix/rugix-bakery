"""Shared projects for rootless environment and artifact build tests."""

from __future__ import annotations

import os
import shutil
import subprocess
import uuid
from collections.abc import Iterator
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder


@pytest.fixture
def env_project(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> Iterator[BakeryBuilder]:
    """An env-only project with a local dependency and a dedicated image-cache volume."""
    project = tmp_path / "project with spaces"
    project.mkdir()
    launcher = project / "run-bakery"
    shutil.copyfile(Path(__file__).parent / "run-bakery", launcher)
    launcher.chmod(0o755)
    (project / "rugix-bakery.toml").write_text(
        '[repositories]\nvendor = { path = "vendor" }\n'
    )
    vendor = project / "vendor"
    vendor.mkdir()
    (vendor / "rugix-repository.toml").write_text('name = "test-vendor"\n')
    dockerfile = """FROM docker.io/library/alpine:3.23
WORKDIR /work
ENV ENV_TEST_DEFAULT=image-default
COPY . /assets
RUN head -c 32 /dev/urandom | sha256sum > /build-token
"""
    for repository in (project, vendor):
        context = repository / "envs" / "tools"
        context.mkdir(parents=True)
        (context / "Dockerfile").write_text(dockerfile)
        (context / ".dockerignore").write_text("ignored.txt\n")
        (context / "included.txt").write_text("original\n")
        (context / "ignored.txt").write_text("ignored\n")
    (project / "envs" / "helpers").mkdir()
    hidden = project / "envs" / ".hidden"
    hidden.mkdir()
    (hidden / "Dockerfile").write_text(dockerfile)
    # Discovery must not depend on loading unrelated, unfinished layer definitions.
    (project / "layers").mkdir()
    (project / "layers" / "unfinished.toml").write_text("this is not valid TOML")

    volume = f"rugix-bakery-env-tests-{uuid.uuid4().hex}"
    monkeypatch.setenv("RUGIX_CACHE_VOLUME", volume)
    monkeypatch.setenv("RUGIX_DEV", "true")
    runtime = os.environ.get("DOCKER") or shutil.which("podman") or "docker"
    try:
        yield BakeryBuilder(project, tmp_path / "run")
    finally:
        if (
            subprocess.run(
                [runtime, "volume", "inspect", volume], check=False, capture_output=True
            ).returncode
            == 0
        ):
            subprocess.run([runtime, "volume", "rm", volume], check=True)
