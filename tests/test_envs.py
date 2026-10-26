"""Independent tooling environments use repository discovery and Buildah's image cache."""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import time
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


def test_env_discovery(
    env_project: BakeryBuilder, capfd: pytest.CaptureFixture[str]
) -> None:
    """Discover local and repository envs without building or loading recipes and layers."""
    env_project.run(["envs", "list"])
    names = capfd.readouterr().out.splitlines()
    assert names == ["tools", "vendor/tools"]

    env_project.run(["envs", "inspect", "vendor/tools", "--json"])
    info = json.loads(capfd.readouterr().out)
    assert info["name"] == "vendor/tools"
    assert info["context"] == "/project/vendor/envs/tools"
    assert info["dockerfile"] == f"{info['context']}/Dockerfile"

    for name in ("missing", "missing/tools", "vendor/../tools"):
        with pytest.raises(subprocess.CalledProcessError):
            env_project.run(["envs", "inspect", name])
        assert "unable to find env" in capfd.readouterr().err


@pytest.mark.parametrize("storage", ["auto", "vfs"])
def test_env_build_and_shell(
    env_project: BakeryBuilder,
    capfd: pytest.CaptureFixture[str],
    storage: str,
) -> None:
    """Cache across outer restarts, honor contexts/defaults, and discard shell changes."""
    options = ["tools", "--storage", storage]

    def build(*extra: str) -> str:
        env_project.run(["envs", "build", *options, *extra])
        image = capfd.readouterr().out.strip()
        assert re.fullmatch(r"sha256:[0-9a-f]{64}", image)
        return image

    image = build()
    assert build() == image
    env_project.run(
        [
            "envs",
            "shell",
            *options,
            "--",
            "/bin/sh",
            "-ec",
            (
                'test "$PWD" = /work; test "$ENV_TEST_DEFAULT" = image-default; '
                'test ! -e /assets/ignored.txt; test "$(cat /assets/included.txt)" = original; '
                "touch /invocation-only; printf persisted > /project/shell-output"
            ),
        ]
    )
    capfd.readouterr()
    assert (env_project.project_dir / "shell-output").read_text() == "persisted"
    env_project.run(
        [
            "envs",
            "shell",
            *options,
            "--",
            "/bin/sh",
            "-ec",
            "test ! -e /invocation-only",
        ]
    )
    capfd.readouterr()

    with pytest.raises(subprocess.CalledProcessError) as failure:
        env_project.run(["envs", "shell", *options, "--", "/bin/sh", "-c", "exit 37"])
    assert failure.value.returncode == 37
    capfd.readouterr()

    context = env_project.project_dir / "envs" / "tools"
    (context / "ignored.txt").write_text("changed but still ignored\n")
    assert build() == image
    (context / "included.txt").write_text("changed\n")
    changed = build()
    assert changed != image
    assert build("--no-cache") != changed


def test_env_build_failure(
    env_project: BakeryBuilder, capfd: pytest.CaptureFixture[str]
) -> None:
    """A failed Dockerfile fails construction and a corrected definition can build afterward."""
    dockerfile = env_project.project_dir / "envs" / "tools" / "Dockerfile"
    original = dockerfile.read_text()
    dockerfile.write_text(original + "RUN false\n")
    with pytest.raises(subprocess.CalledProcessError):
        env_project.run(["envs", "build", "tools"])
    assert "building env failed" in capfd.readouterr().err
    dockerfile.write_text(original)
    env_project.run(["envs", "build", "tools"])
    assert re.fullmatch(r"sha256:[0-9a-f]{64}", capfd.readouterr().out.strip())


def test_env_build_cancellation(
    env_project: BakeryBuilder, tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """Interrupting a RUN lets Buildah remove its persistent working container."""
    dockerfile = env_project.project_dir / "envs" / "tools" / "Dockerfile"
    dockerfile.write_text(
        "FROM docker.io/library/alpine:3.23\nRUN echo RUGIX_CANCEL_READY; sleep 120\n"
    )
    runtime = os.environ.get("DOCKER") or shutil.which("podman") or "docker"
    name = f"rugix-env-cancel-{uuid.uuid4().hex}"
    monkeypatch.setenv(
        "DOCKER_FLAGS", f"{os.environ.get('DOCKER_FLAGS', '')} --name {name}"
    )
    log = tmp_path / "cancel.log"
    with log.open("w") as output:
        process = subprocess.Popen(
            ["./run-bakery", "envs", "build", "tools", "--storage", "vfs"],
            cwd=env_project.project_dir,
            stdout=output,
            stderr=subprocess.STDOUT,
        )
        try:
            deadline = time.monotonic() + 60
            while "RUGIX_CANCEL_READY\n" not in log.read_text():
                assert process.poll() is None, log.read_text()
                assert time.monotonic() < deadline, log.read_text()
                time.sleep(0.1)
            subprocess.run(
                [runtime, "kill", "--signal", "INT", name],
                check=True,
                capture_output=True,
            )
            assert process.wait(timeout=15) != 0
        finally:
            if process.poll() is None:
                subprocess.run(
                    [runtime, "rm", "--force", name], check=False, capture_output=True
                )
                process.wait(timeout=15)

    result = subprocess.run(
        [
            runtime,
            "run",
            "--rm",
            "--volume",
            f"{os.environ['RUGIX_CACHE_VOLUME']}:/cache:ro",
            "--entrypoint",
            "/bin/sh",
            os.environ.get("RUGIX_BAKERY_IMAGE", "ghcr.io/rugix/rugix-bakery:dev"),
            "-c",
            "find /cache -path '*/vfs-containers/containers.json' -exec cat {} +",
        ],
        check=True,
        capture_output=True,
        text=True,
    )
    assert json.loads(result.stdout) == [], log.read_text()
