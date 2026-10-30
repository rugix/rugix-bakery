"""Rules and layers discover project dependencies while running."""

from __future__ import annotations

import os
import subprocess
import tarfile
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder

from test_rules import build, prepare


@pytest.mark.parametrize("use_env", [False, True])
def test_discovered_project_dependencies(
    env_project: BakeryBuilder, use_env: bool
) -> None:
    """Track selected files, trees and absent paths, replacing registrations each run."""
    project = env_project.project_dir
    prepare(
        project,
        """set -eu
if printf forbidden > "$RUGIX_PROJECT_DIR/choice"; then exit 43; fi
printf '%s\\n' choice assets optional >> "$RUGIX_REBUILD_IF_CHANGED"
choice=$(cat "$RUGIX_PROJECT_DIR/choice")
printf '%s\\n' "$choice" >> "$RUGIX_REBUILD_IF_CHANGED"
cat "$RUGIX_PROJECT_DIR/$choice" > /run/rugix/bakery/work/outputs/result
for file in "$RUGIX_PROJECT_DIR"/assets/*; do cat "$file" >> /run/rugix/bakery/work/outputs/result; done
if [ -f "$RUGIX_PROJECT_DIR/optional" ]; then
    cat "$RUGIX_PROJECT_DIR/optional" >> /run/rugix/bakery/work/outputs/result
fi
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=use_env,
    )
    consumer = project / "vendor/rules/consume"
    consumer.mkdir()
    (consumer / "rule.toml").write_text(
        '[inputs]\nsource = { kind = "file" }\n'
        '[execution]\ntype = "run"\n'
        'command = ["/bin/sh", "/run/rugix/bakery/work/rule/run.sh"]\n'
        '[execution.outputs.result]\nkind = "file"\npath = "result"\n'
        '[execution.outputs.token]\nkind = "file"\npath = "token"\n'
    )
    (consumer / "run.sh").write_text(
        "cat /run/rugix/bakery/work/inputs/source > /run/rugix/bakery/work/outputs/result\n"
        "head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token\n"
    )
    graph = project / "vendor/rules/pair/rule.toml"
    graph.write_text(
        """[inputs]
source = { kind = "file" }
[execution]
type = "graph"
[execution.rules.copy]
uses = "copy"
[execution.rules.consume]
uses = "consume"
inputs.source = { rule = { node = "copy", output = "result" } }
[execution.outputs]
first = { rule = { node = "consume", output = "result" } }
second = { rule = { node = "consume", output = "result" } }
token = { rule = { node = "consume", output = "token" } }
"""
    )
    (project / "choice").write_text("file a")
    (project / "file a").write_text("aaaa")
    (project / "file b").write_text("bbbb")
    assets = project / "assets"
    assets.mkdir()
    (assets / "one").write_text("1111")
    output = build(env_project)
    assert (output / "first").read_text() == "aaaa1111"
    token = (output / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() == token

    (project / "unrelated").write_text("not registered")
    assert (build(env_project) / "token").read_bytes() == token

    original = (project / "file a").stat()
    (project / "file a").write_text("AAAA")
    os.utime(project / "file a", ns=(original.st_atime_ns, original.st_mtime_ns))
    output = build(env_project)
    assert (output / "first").read_text() == "AAAA1111"
    assert (output / "token").read_bytes() != token

    (assets / "two").write_text("2222")
    assert (build(env_project) / "first").read_text() == "AAAA11112222"
    (assets / "one").unlink()
    assert (build(env_project) / "first").read_text() == "AAAA2222"

    (project / "optional").write_text("optional")
    assert (build(env_project) / "first").read_text() == "AAAA2222optional"
    (project / "optional").unlink()
    assert (build(env_project) / "first").read_text() == "AAAA2222"

    (project / "choice").write_text("file b")
    output = build(env_project)
    assert (output / "first").read_text() == "bbbb2222"
    token = (output / "token").read_bytes()
    (project / "file a").write_text("no longer used")
    assert (build(env_project) / "token").read_bytes() == token
    (project / "file b").write_text("changed")
    assert (build(env_project) / "first").read_text() == "changed2222"


@pytest.mark.parametrize("use_env", [False, True])
def test_project_dependency_failure_recovery(
    env_project: BakeryBuilder, use_env: bool
) -> None:
    """A failed execution cannot replace successful outputs or dependency records."""
    project = env_project.project_dir
    prepare(
        project,
        """set -eu
printf '%s\\n' optional >> "$RUGIX_REBUILD_IF_CHANGED"
if [ -f "$RUGIX_PROJECT_DIR/optional" ]; then
    printf '%s\\n' abandoned >> "$RUGIX_REBUILD_IF_CHANGED"
    exit 37
fi
echo successful > /run/rugix/bakery/work/outputs/result
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=use_env,
    )
    token = (build(env_project) / "token").read_bytes()
    (project / "optional").touch()
    with pytest.raises(subprocess.CalledProcessError):
        build(env_project)
    assert (project / "build/result/token").read_bytes() == token
    (project / "optional").unlink()
    assert (build(env_project) / "token").read_bytes() == token
    (project / "abandoned").touch()
    assert (build(env_project) / "token").read_bytes() == token


@pytest.mark.parametrize("use_env", [False, True])
def test_project_dependency_symlinks(env_project: BakeryBuilder, use_env: bool) -> None:
    """Track contained symlink targets and reject escapes without publishing."""
    project = env_project.project_dir
    prepare(
        project,
        """set -eu
printf '%s\\n' assets >> "$RUGIX_REBUILD_IF_CHANGED"
cat "$RUGIX_PROJECT_DIR/assets/link" > /run/rugix/bakery/work/outputs/result
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=use_env,
    )
    (project / "target").write_text("first")
    assets = project / "assets"
    assets.mkdir()
    (assets / "link").symlink_to("../target")
    assert (build(env_project) / "first").read_text() == "first"
    (project / "target").write_text("changed")
    assert (build(env_project) / "first").read_text() == "changed"
    (assets / "link").unlink()
    (assets / "link").symlink_to("/etc/hostname")
    with pytest.raises(subprocess.CalledProcessError):
        build(env_project)
    assert (project / "build/result/first").read_text() == "changed"


@pytest.mark.parametrize("path", ["../outside", "/etc/hostname"])
def test_invalid_project_dependency(env_project: BakeryBuilder, path: str) -> None:
    """Invalid registrations fail even when the command produces valid outputs."""
    prepare(
        env_project.project_dir,
        f"""set -eu
printf '%s\\n' '{path}' >> "$RUGIX_REBUILD_IF_CHANGED"
echo complete > /run/rugix/bakery/work/outputs/result
echo token > /run/rugix/bakery/work/outputs/token
""",
        use_env=False,
    )
    with pytest.raises(subprocess.CalledProcessError):
        build(env_project)
    assert not (env_project.project_dir / "build/result").exists()


def test_layer_project_dependencies(env_project: BakeryBuilder) -> None:
    """Layer results are immutable, track discovered inputs, and recover after failure."""
    project = env_project.project_dir
    (project / "layers/unfinished.toml").unlink()
    (project / "layers/root.toml").write_text('root = true\nrecipes = ["populate"]\n')
    rule = project / "rules/root"
    rule.mkdir(parents=True)
    (rule / "rule.toml").write_text(
        '[execution]\ntype = "layer"\nlayer = "root"\narchitecture = "amd64"\n'
    )
    (project / "layer.toml").write_text('uses = "root"\n')
    recipe = project / "recipes/populate"
    (recipe / "steps").mkdir(parents=True)
    (recipe / "recipe.toml").write_text('description = "Test project dependencies"\n')
    script = recipe / "steps/00-run.sh"
    script.write_text(
        """#!/bin/sh
set -eu
test "$LAYER_REBUILD_IF_CHANGED" = "$RUGIX_REBUILD_IF_CHANGED"
printf '%s\\n' choice fail >> "$LAYER_REBUILD_IF_CHANGED"
if [ -f "$RUGIX_PROJECT_DIR/fail" ]; then
    printf '%s\\n' abandoned >> "$RUGIX_REBUILD_IF_CHANGED"
    echo test-layer-failure
    exit 37
fi
choice=$(cat "$RUGIX_PROJECT_DIR/choice")
printf '%s\\n' "$choice" >> "$RUGIX_REBUILD_IF_CHANGED"
mkdir -p "$RUGIX_ROOT_DIR/etc"
cat "$RUGIX_PROJECT_DIR/$choice" > "$RUGIX_ROOT_DIR/etc/result"
head -c 32 /dev/urandom > "$RUGIX_ROOT_DIR/etc/token"
"""
    )
    script.chmod(0o755)
    (project / "choice").write_text("file a")
    (project / "file a").write_text("aaaa")
    (project / "file b").write_text("bbbb")
    archive = project / "build/layer/archive"
    cache = project / ".rugix/rules"

    def bake() -> tuple[bytes, bytes]:
        env_project.run(["bake", "rule", "layer.toml", "build/layer"])
        with tarfile.open(archive) as layer:
            result = layer.extractfile("./roots/system/etc/result")
            token = layer.extractfile("./roots/system/etc/token")
            assert result is not None and token is not None
            return result.read(), token.read()

    result, token = bake()
    assert result == b"aaaa"
    assert not (project / ".rugix/layers").exists()
    original_archive = next(cache.glob("*/archive.tar"))
    original_bytes = original_archive.read_bytes()
    assert bake()[1] == token
    original = (project / "file a").stat()
    (project / "file a").write_text("AAAA")
    os.utime(project / "file a", ns=(original.st_atime_ns, original.st_mtime_ns))
    assert bake()[0] == b"AAAA"
    assert original_archive.read_bytes() == original_bytes
    (project / "choice").write_text("file b")
    result, token = bake()
    assert result == b"bbbb"
    (project / "file a").write_text("no longer used")
    assert bake()[1] == token

    published = archive.read_bytes()
    cached_archives = {path: path.read_bytes() for path in cache.glob("*/archive.tar")}
    (project / "fail").touch()
    with pytest.raises(subprocess.CalledProcessError):
        bake()
    assert archive.read_bytes() == published
    assert {
        path: path.read_bytes() for path in cache.glob("*/archive.tar")
    } == cached_archives
    assert any(
        "test-layer-failure" in path.read_text()
        for path in cache.glob("task-*/outputs/build.log")
    )
    (project / "fail").unlink()
    (project / "abandoned").touch()
    assert bake()[1] == token


@pytest.mark.parametrize("use_env", [False, True])
@pytest.mark.parametrize("read_only", [False, True])
def test_project_submounts_are_read_only(
    env_project: BakeryBuilder,
    tmp_path: Path,
    monkeypatch: pytest.MonkeyPatch,
    use_env: bool,
    read_only: bool,
) -> None:
    """Project mounts preserve source metadata and restrictions while enforcing read-only access."""
    source = tmp_path / "external-file"
    source.write_text("external")
    original = source.stat()
    (env_project.project_dir / "external").touch()
    flags = os.environ.get("DOCKER_FLAGS", "")
    mode = "ro" if read_only else "rw"
    monkeypatch.setenv(
        "DOCKER_FLAGS",
        flags + f" -v {source}:/project/external:{mode},noexec,nosuid,nodev",
    )
    prepare(
        env_project.project_dir,
        """set -eu
if printf forbidden > "$RUGIX_PROJECT_DIR/external"; then exit 43; fi
printf '%s\\n' external >> "$RUGIX_REBUILD_IF_CHANGED"
cat "$RUGIX_PROJECT_DIR/external" > /run/rugix/bakery/work/outputs/result
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=use_env,
    )
    assert (build(env_project) / "first").read_text() == "external"
    assert source.read_text() == "external"
    assert source.stat().st_mtime_ns == original.st_mtime_ns


@pytest.mark.parametrize("use_env", [False, True])
def test_registered_symlink_identity(env_project: BakeryBuilder, use_env: bool) -> None:
    """A directly registered directory symlink tracks its own text and resolved contents."""
    project = env_project.project_dir
    prepare(
        project,
        """set -eu
printf '%s\\n' link >> "$RUGIX_REBUILD_IF_CHANGED"
readlink "$RUGIX_PROJECT_DIR/link" > /run/rugix/bakery/work/outputs/result
cat "$RUGIX_PROJECT_DIR/link/content" >> /run/rugix/bakery/work/outputs/result
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=use_env,
    )
    (project / "target").mkdir()
    (project / "target/content").write_text("contents")
    (project / "link").symlink_to("target", target_is_directory=True)
    output = build(env_project)
    assert (output / "first").read_text() == "target\ncontents"
    (project / "link").unlink()
    (project / "link").symlink_to("./target", target_is_directory=True)
    output = build(env_project)
    assert (output / "first").read_text() == "./target\ncontents"
    token = (output / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() == token


def test_missing_dependency_below_regular_file(env_project: BakeryBuilder) -> None:
    """An absent descendant remains cacheable when an ancestor is a regular file."""
    project = env_project.project_dir
    prepare(
        project,
        """set -eu
printf '%s\\n' optional/child >> "$RUGIX_REBUILD_IF_CHANGED"
if [ -f "$RUGIX_PROJECT_DIR/optional/child" ]; then
    cat "$RUGIX_PROJECT_DIR/optional/child" > /run/rugix/bakery/work/outputs/result
else
    echo absent > /run/rugix/bakery/work/outputs/result
fi
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=False,
    )
    (project / "optional").touch()
    output = build(env_project)
    assert (output / "first").read_text() == "absent\n"
    token = (output / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() == token
    (project / "optional").unlink()
    (project / "optional").mkdir()
    (project / "optional/child").write_text("created")
    assert (build(env_project) / "first").read_text() == "created"


def test_symlinked_rule_directory(env_project: BakeryBuilder) -> None:
    """Rule-directory symlinks track changes to their implementation and resources."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    rule = project / "vendor/rules/copy"
    target = project / "vendor/rules/implementation"
    rule.rename(target)
    rule.symlink_to("implementation", target_is_directory=True)
    output = build(env_project)
    assert (output / "first").read_text() == "input\nvendor\n"
    token = (output / "token").read_bytes()
    (target / "included.txt").write_text("changed resource\n")
    output = build(env_project)
    assert (output / "first").read_text() == "input\nchanged resource\n"
    assert (output / "token").read_bytes() != token
    script = target / "run.sh"
    script.write_text(
        script.read_text()
        + "echo changed script >> /run/rugix/bakery/work/outputs/result\n"
    )
    assert (build(env_project) / "first").read_text().endswith("changed script\n")


@pytest.mark.parametrize("use_env", [False, True])
@pytest.mark.parametrize("register_tree", [False, True])
def test_optional_symlink_dependency(
    env_project: BakeryBuilder, use_env: bool, register_tree: bool
) -> None:
    """Dangling links remain cacheable and track target creation, removal, and escape attempts."""
    project = env_project.project_dir
    registration = "assets" if register_tree else "assets/link"
    prepare(
        project,
        f"""set -eu
printf '%s\\n' {registration} >> "$RUGIX_REBUILD_IF_CHANGED"
if [ -f "$RUGIX_PROJECT_DIR/assets/link" ]; then
    cat "$RUGIX_PROJECT_DIR/assets/link" > /run/rugix/bakery/work/outputs/result
else
    echo absent > /run/rugix/bakery/work/outputs/result
fi
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
""",
        use_env=use_env,
    )
    assets = project / "assets"
    assets.mkdir()
    link = assets / "link"
    link.symlink_to("../target")
    output = build(env_project)
    assert (output / "first").read_text() == "absent\n"
    token = (output / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() == token
    (project / "target").write_text("present")
    assert (build(env_project) / "first").read_text() == "present"
    (project / "target").unlink()
    output = build(env_project)
    assert (output / "first").read_text() == "absent\n"
    token = (output / "token").read_bytes()
    assert (build(env_project) / "token").read_bytes() == token
    link.unlink()
    link.symlink_to("../different-missing-target")
    assert (build(env_project) / "token").read_bytes() != token
    link.unlink()
    link.symlink_to("../intermediate/../target")
    (project / "target").write_text("present")
    assert (build(env_project) / "first").read_text() == "absent\n"
    token = (build(env_project) / "token").read_bytes()
    (project / "intermediate").mkdir()
    assert (build(env_project) / "first").read_text() == "present"
    (project / "intermediate").rmdir()
    (project / "intermediate").touch()
    assert (build(env_project) / "first").read_text() == "absent\n"
    (project / "intermediate").unlink()
    (project / "intermediate").mkdir()
    assert (build(env_project) / "first").read_text() == "present"
    link.unlink()
    link.symlink_to("/outside-project/missing-target")
    with pytest.raises(subprocess.CalledProcessError):
        build(env_project)
    assert (project / "build/result/first").read_text() == "present"
