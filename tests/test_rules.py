"""Repository-defined builds compose isolated commands and cache declared artifacts."""

from __future__ import annotations

import os
import shutil
import subprocess
import uuid
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder


def prepare(project: Path, command: str | None = None, *, use_env: bool = True) -> None:
    """Create a vendor operation and a composition exporting its result twice."""
    operation = project / "vendor/rules/copy"
    operation.mkdir(parents=True)
    suffix = (
        "/assets/included.txt"
        if use_env
        else "/run/rugix/bakery/work/rule/included.txt"
    )
    (operation / "included.txt").write_text("vendor\n")
    (operation / "run.sh").write_text(
        command
        or """set -eu
if printf forbidden > /run/rugix/bakery/work/inputs/source; then exit 43; fi
cat /run/rugix/bakery/work/inputs/source SUFFIX > /run/rugix/bakery/work/outputs/result
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""".replace("SUFFIX", suffix)
    )
    (operation / "rule.toml").write_text(
        """[inputs]
source = "file"
[execution]
type = "run"
ENVIRONMENT
command = ["/bin/sh", "/run/rugix/bakery/work/rule/run.sh"]
[execution.outputs.result]
kind = "file"
path = "result"
[execution.outputs.token]
kind = "file"
path = "token"
""".replace("ENVIRONMENT", 'env = "tools"' if use_env else "")
    )
    graph = project / "vendor/rules/pair"
    graph.mkdir(parents=True)
    (graph / "rule.toml").write_text(
        """[inputs]
source = "file"
[execution]
type = "graph"
[execution.rules.copy]
uses = "copy"
[execution.outputs]
first = { rule = { node = "copy", output = "result" } }
second = { rule = { node = "copy", output = "result" } }
token = { rule = { node = "copy", output = "token" } }
"""
    )
    (project / "source").write_text("input\n")
    (project / "vendor/envs/tools/included.txt").write_text("vendor\n")
    (project / "invocation.toml").write_text(
        'uses = "vendor/pair"\ninputs.source = { file = "source" }\n'
    )


def build(bakery: BakeryBuilder, output: str = "build/result") -> Path:
    """Execute through the public CLI, including repository and env resolution."""
    bakery.run(["bake", "rule", "invocation.toml", output])
    return bakery.project_dir / output


@pytest.mark.parametrize("use_env", [False, True])
def test_composition_and_cache(env_project: BakeryBuilder, use_env: bool) -> None:
    """Share node outputs, enforce read-only inputs, and track inputs, scripts, and envs."""
    project = env_project.project_dir
    prepare(project, use_env=use_env)
    output = build(env_project)
    assert (output / "first").read_text() == "input\nvendor\n"
    assert (output / "second").read_bytes() == (output / "first").read_bytes()
    token = (output / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() == token

    (project / "source").write_text("changed\n")
    output = build(env_project)
    assert (output / "first").read_text() == "changed\nvendor\n"
    changed_token = (output / "token").read_bytes()
    assert changed_token != token

    script = project / "vendor/rules/copy/run.sh"
    script.write_text(
        script.read_text()
        + "printf appended >> /run/rugix/bakery/work/outputs/result\n"
    )
    output = build(env_project)
    assert (output / "first").read_text().endswith("appended")
    assert (output / "token").read_bytes() != changed_token

    resource = "envs/tools" if use_env else "rules/copy"
    (project / "vendor" / resource / "included.txt").write_text("new env\n")
    assert "new env" in (build(env_project) / "first").read_text()


@pytest.mark.parametrize("use_env", [False, True])
def test_failed_build_does_not_publish(
    env_project: BakeryBuilder, use_env: bool
) -> None:
    """Failing commands publish no partial build and can recover without cache cleanup."""
    project = env_project.project_dir
    prepare(
        project,
        "echo partial > /run/rugix/bakery/work/outputs/result; exit 37\n",
        use_env=use_env,
    )
    with pytest.raises(subprocess.CalledProcessError):
        build(env_project)
    assert not (project / "build/result").exists()
    (project / "vendor/rules/copy/run.sh").write_text(
        "echo complete > /run/rugix/bakery/work/outputs/result; echo recovered > /run/rugix/bakery/work/outputs/token\n"
    )
    assert (build(env_project) / "first").read_text() == "complete\n"


@pytest.mark.parametrize("use_env", [False, True])
@pytest.mark.parametrize("directory", [False, True])
def test_secret_cache_invalidation(
    env_project: BakeryBuilder,
    capfd: pytest.CaptureFixture[str],
    use_env: bool,
    directory: bool,
) -> None:
    """Reuse sensitive inputs, rebuild after rotation, and keep secrets out of cache metadata."""
    project = env_project.project_dir
    prepare(project, use_env=use_env)
    operation = project / "vendor/rules/copy/rule.toml"
    operation.write_text('secrets = ["key"]\n' + operation.read_text())
    (project / "vendor/rules/copy/run.sh").write_text(
        "set -eu\ntest -e /run/rugix/bakery/work/secrets/key\n"
        "printf signed > /run/rugix/bakery/work/outputs/result\n"
        "head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token\n"
    )
    graph = project / "vendor/rules/pair/rule.toml"
    graph.write_text(
        'secrets = ["key"]\n'
        + graph.read_text().replace(
            'uses = "copy"', 'uses = "copy"\nsecrets.key = { input = "key" }'
        )
    )
    binding = project / "key"
    if directory:
        binding.mkdir()
        secret = binding / "private-key"
    else:
        secret = binding
    original = b"private-test-material-first"
    secret.write_bytes(original)
    invocation = project / "invocation.toml"
    invocation.write_text(invocation.read_text() + 'secrets.key = { file = "key" }\n')
    token = (build(env_project) / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() == token
    timestamp = secret.stat().st_mtime_ns
    secret.write_bytes(b"private-test-material-other")
    os.utime(secret, ns=(timestamp, timestamp))
    rotated = (build(env_project) / "token").read_bytes()
    assert rotated != token
    assert (build(env_project) / "token").read_bytes() == rotated

    cache = project / ".rugix/rules"
    fingerprint_key = cache / "secret-fingerprint.key"
    assert fingerprint_key.stat().st_mode & 0o777 == 0o600
    fingerprint_key.unlink()
    renewed = (build(env_project) / "token").read_bytes()
    assert renewed != rotated
    assert (build(env_project) / "token").read_bytes() == renewed
    for path in cache.rglob("*"):
        if path.is_file():
            assert original not in path.read_bytes()
            assert b"private-test-material-other" not in path.read_bytes()
    captured = capfd.readouterr()
    assert "private-test-material" not in captured.out + captured.err
    secret.unlink()
    if directory:
        binding.rmdir()
    with pytest.raises(subprocess.CalledProcessError):
        build(env_project)
    assert (project / "build/result/token").read_bytes() == renewed
    if directory:
        binding.mkdir()
    secret.write_bytes(original)
    operation.write_text(
        operation.read_text().replace('type = "run"', 'type = "run"\ncache = false')
    )
    uncached = (build(env_project) / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() != uncached


@pytest.mark.parametrize("use_env", [False, True])
def test_invalid_output_does_not_publish(
    env_project: BakeryBuilder, use_env: bool
) -> None:
    """A successful command must still satisfy its declared outputs."""
    prepare(
        env_project.project_dir,
        "echo incomplete > /run/rugix/bakery/work/outputs/token\n",
        use_env=use_env,
    )
    with pytest.raises(subprocess.CalledProcessError):
        build(env_project)
    assert not (env_project.project_dir / "build/result").exists()


@pytest.mark.parametrize("use_env", [False, True])
def test_noexec_input_mount(
    env_project: BakeryBuilder,
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    use_env: bool,
) -> None:
    """A locked noexec source stays readable inside an inner working container."""
    prepare(env_project.project_dir, use_env=use_env)
    source = tmp_path / "restricted-input"
    source.write_text("restricted input\n")
    flags = os.environ.get("DOCKER_FLAGS", "")
    monkeypatch.setenv(
        "DOCKER_FLAGS", flags + f" -v {source}:/project/source:ro,noexec,nosuid,nodev"
    )
    assert (build(env_project) / "first").read_text() == "restricted input\nvendor\n"


def test_bakery_mounts_and_scratch_without_buildah(
    env_project: BakeryBuilder, tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """Native rules need no Buildah and do not leak mounts or scratch across nodes."""
    project = env_project.project_dir
    prepare(
        project,
        """set -eu
test "$(pwd)" = /run/rugix/bakery/work/outputs
test -d /usr/share/rugix
test ! -e /tmp/native-rule-marker
touch /tmp/native-rule-marker
cat /run/rugix/bakery/work/inputs/source > /run/rugix/bakery/work/outputs/result
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=False,
    )
    disabled = tmp_path / "buildah-disabled"
    disabled.write_text("#!/bin/sh\nexit 97\n")
    disabled.chmod(0o755)
    flags = os.environ.get("DOCKER_FLAGS", "")
    monkeypatch.setenv("DOCKER_FLAGS", flags + f" -v {disabled}:/usr/bin/buildah:ro")
    consumer = project / "vendor/rules/consumer"
    consumer.mkdir()
    (consumer / "rule.toml").write_text(
        """[inputs]
previous = "file"
[execution]
type = "run"
command = ["/bin/sh", "/run/rugix/bakery/work/rule/run.sh"]
[execution.outputs.result]
kind = "file"
path = "result"
"""
    )
    (consumer / "run.sh").write_text(
        """set -eu
test ! -e /tmp/native-rule-marker
test ! -e /run/rugix/bakery/work/inputs/source
test ! -e /run/rugix/bakery/work/rule/included.txt
cat /run/rugix/bakery/work/inputs/previous > /run/rugix/bakery/work/outputs/result
python3 -c 'import json; assert json.load(open("/run/rugix/bakery/work/parameters.json")) == {"message": "hello"}'
"""
    )
    with (consumer / "rule.toml").open("a") as definition:
        definition.write('\n[parameters.message]\ndefault = "hello"\n')
    graph = project / "vendor/rules/pair/rule.toml"
    graph.write_text(
        graph.read_text()
        .replace(
            "[execution.outputs]",
            '[execution.rules.consume]\nuses = "consumer"\n'
            'inputs.previous = { rule = { node = "copy", output = "result" } }\n'
            "[execution.outputs]",
        )
        .replace(
            'first = { rule = { node = "copy", output = "result" } }',
            'first = { rule = { node = "consume", output = "result" } }',
        )
    )
    assert (build(env_project) / "first").read_text() == "input\n"


def test_bakery_cache_tracks_runtime(
    env_project: BakeryBuilder, tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    """Image changes invalidate native outputs; an unidentified runtime never reuses them."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    token = (build(env_project) / "token").read_bytes()
    runtime = os.environ.get("DOCKER") or shutil.which("podman") or "docker"
    original = os.environ.get("RUGIX_BAKERY_IMAGE", "ghcr.io/rugix/rugix-bakery:dev")
    image = f"localhost/rugix-native-test-{uuid.uuid4().hex}"
    context = tmp_path / "image-context"
    context.mkdir()
    (context / "Dockerfile").write_text(
        f'FROM {original}\nLABEL native-cache-test="different-image"\n'
    )
    subprocess.run([runtime, "build", "--tag", image, str(context)], check=True)
    try:
        monkeypatch.setenv("RUGIX_BAKERY_IMAGE", image)
        changed = (build(env_project) / "token").read_bytes()
        assert changed != token
        assert (build(env_project) / "token").read_bytes() == changed

        launcher = project / "run-bakery"
        launcher.write_text(
            launcher.read_text().replace(
                '-e "RUGIX_BAKERY_IMAGE=${RUGIX_BAKERY_IMAGE}"', ""
            )
        )
        unidentified = (build(env_project) / "token").read_bytes()
        assert (build(env_project) / "token").read_bytes() != unidentified
    finally:
        subprocess.run([runtime, "image", "rm", image], check=True)
