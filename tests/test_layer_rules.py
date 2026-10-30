"""Layer and recipe artifact dependencies execute within the rule graph."""

from __future__ import annotations

import json
import os
import subprocess
import tarfile
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder


def prepare(project: Path, *, use_env: bool) -> None:
    """Compose parent filesystem, vendor tooling, installation, and image assembly."""
    (project / "layers/unfinished.toml").unlink()
    (project / "layers/base.toml").write_text('root = true\nrecipes = ["base"]\n')
    recipe = project / "recipes/base"
    (recipe / "steps").mkdir(parents=True)
    (recipe / "recipe.toml").write_text('description = "Base filesystem"\n')
    script = recipe / "steps/00-run.sh"
    script.write_text(
        """#!/bin/sh
set -eu
mkdir -p "$RUGIX_ROOT_DIR/etc" "$RUGIX_LAYER_DIR/roots/boot"
printf base > "$RUGIX_ROOT_DIR/etc/base"
printf boot-context > "$RUGIX_LAYER_DIR/roots/boot/marker"
"""
    )
    script.chmod(0o755)
    recipe = project / "vendor/recipes/install-app"
    (recipe / "steps").mkdir(parents=True)
    (recipe / "recipe.toml").write_text(
        '[inputs]\napplication = "file"\ntoken = "file"\n'
    )
    script = recipe / "steps/00-run.sh"
    script.write_text(
        """#!/bin/sh
set -eu
if printf forbidden > "$RECIPE_INPUTS_DIR/application"; then exit 43; fi
cp "$RECIPE_INPUTS_DIR/application" "$RUGIX_ROOT_DIR/etc/application"
cp "$RECIPE_INPUTS_DIR/token" "$RUGIX_ROOT_DIR/etc/token"
"""
    )
    script.chmod(0o755)
    rule = project / "vendor/rules/app"
    rule.mkdir(parents=True)
    (rule / "rule.toml").write_text(
        """[inputs]
parent = "directory"
source = "file"
[parameters.architecture]
default = "unused"
[execution]
type = "run"
ENVIRONMENT
command = ["/bin/sh", "/run/rugix/bakery/work/rule/run.sh"]
[execution.outputs.application]
kind = "file"
path = "application"
[execution.outputs.token]
kind = "file"
path = "token"
""".replace("ENVIRONMENT", 'env = "tools"' if use_env else "")
    )
    (rule / "run.sh").write_text(
        """set -eu
if grep -q fail /run/rugix/bakery/work/inputs/source; then exit 37; fi
cat /run/rugix/bakery/work/inputs/parent/etc/base /run/rugix/bakery/work/inputs/source > /run/rugix/bakery/work/outputs/application
head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token
"""
    )
    (project / "source").write_text("application")
    (project / "layers/device.toml").write_text(
        """parent = "base"
recipes = ["vendor/install-app"]
[rules.application]
uses = "vendor/app"
inputs.parent = { input = "rootfs" }
inputs.source = { path = "source" }
parameters.architecture = { parameter = "architecture" }
[inputs."vendor/install-app"]
application = { rule = { node = "application", output = "application" } }
token = { rule = { node = "application", output = "token" } }
"""
    )
    rule = project / "rules/device"
    rule.mkdir(parents=True)
    (rule / "rule.toml").write_text(
        '[execution]\ntype = "layer"\nlayer = "device"\narchitecture = "amd64"\n'
    )
    (project / "invocation.toml").write_text('uses = "device"\n')
    rule = project / "rules/image"
    rule.mkdir()
    (rule / "rule.toml").write_text(
        """[inputs]
rootfs = "directory"
boot = "directory"
context = "file"
[execution]
type = "graph"
[execution.rules.filesystem]
uses = "core/image-ext4"
inputs.tree = { input = "rootfs" }
[execution.outputs]
image = { rule = { node = "filesystem", output = "image" } }
application = { select = { source = { input = "rootfs" }, path = "etc/application", kind = "file" } }
token = { select = { source = { input = "rootfs" }, path = "etc/token", kind = "file" } }
boot-context = { select = { source = { input = "boot" }, path = "marker", kind = "file" } }
release = { select = { source = { input = "rootfs" }, path = "etc/rugix/system-build-info.json", kind = "file" } }
"""
    )
    with (project / "rugix-bakery.toml").open("a") as config:
        config.write(
            '\n[systems.device]\nlayer = "device"\narchitecture = "amd64"\n'
            '[systems.device.build]\nuses = "image"\n'
        )


def build_rule(bakery: BakeryBuilder) -> Path:
    """Export a layer directly through a layer rule."""
    bakery.run(["bake", "rule", "invocation.toml", "build/layer"])
    return bakery.project_dir / "build/layer"


@pytest.mark.parametrize("use_env", [False, True])
def test_rules_feed_layers_and_images(
    env_project: BakeryBuilder, use_env: bool
) -> None:
    """Rule inputs, parent filesystems, layer caching, and release assembly compose."""
    project = env_project.project_dir
    prepare(project, use_env=use_env)
    output = build_rule(env_project)
    assert (output / "rootfs/etc/application").read_text() == "baseapplication"
    assert (output / "boot/marker").read_text() == "boot-context"
    assert not (output / "rootfs/etc/rugix/system-build-info.json").exists()
    token = (output / "rootfs/etc/token").read_bytes()
    assert (build_rule(env_project) / "rootfs/etc/token").read_bytes() == token
    (project / "unrelated").touch()
    assert (build_rule(env_project) / "rootfs/etc/token").read_bytes() == token

    original = (project / "source").stat()
    (project / "source").write_text("replacement")
    os.utime(project / "source", ns=(original.st_atime_ns, original.st_mtime_ns))
    output = build_rule(env_project)
    assert (output / "rootfs/etc/application").read_text() == "basereplacement"
    assert (output / "rootfs/etc/token").read_bytes() != token

    env_project.run(["bake", "image", "device", "--release-version", "test-release"])
    output = project / "build/device"
    assert (output / "application").read_text() == "basereplacement"
    assert (output / "boot-context").read_text() == "boot-context"
    assert (output / "system.img").read_bytes()[1080:1082] == b"\x53\xef"
    context = json.loads((output / "build-context.json").read_text())
    assert context["release-version"] == "test-release"
    assert not (
        build_rule(env_project) / "rootfs/etc/rugix/system-build-info.json"
    ).exists()

    (project / "source").write_text("fail")
    with pytest.raises(subprocess.CalledProcessError):
        build_rule(env_project)
    assert (
        project / "build/layer/rootfs/etc/application"
    ).read_text() == "basereplacement"


@pytest.mark.parametrize("problem", ["cycle", "kind", "missing", "unknown"])
def test_layer_input_validation(env_project: BakeryBuilder, problem: str) -> None:
    """Reject mixed rule/layer cycles and invalid recipe bindings before execution."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    config = project / "layers/device.toml"
    if problem == "cycle":
        config.write_text(
            config.read_text()
            .replace('uses = "vendor/app"', 'uses = "device"')
            .replace('inputs.parent = { input = "rootfs" }\n', "")
            .replace('inputs.source = { path = "source" }\n', "")
            .replace('parameters.architecture = { parameter = "architecture" }\n', "")
            .replace('output = "application"', 'output = "archive"')
            .replace('output = "token"', 'output = "archive"')
        )
    elif problem == "kind":
        recipe = project / "vendor/recipes/install-app/recipe.toml"
        recipe.write_text(recipe.read_text().replace('"file"', '"directory"', 1))
    elif problem == "missing":
        config.write_text(
            config.read_text().replace(
                'application = { rule = { node = "application", output = "application" } }\n',
                "",
            )
        )
    else:
        config.write_text(config.read_text() + 'typo = { path = "source" }\n')
    with pytest.raises(subprocess.CalledProcessError):
        build_rule(env_project)
    assert not (project / ".rugix/layers").exists()
    assert not (project / "build/layer").exists()


def test_mixin_rule_inputs(env_project: BakeryBuilder) -> None:
    """Mixins can build artifacts from their predecessor and pass them to recipes."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    (project / "addon").write_text("addon")
    (project / "mixins").mkdir()
    (project / "mixins/addon.toml").write_text(
        """recipes = ["vendor/install-app"]
[rules.addon]
uses = "vendor/app"
inputs.parent = { input = "rootfs" }
inputs.source = { path = "addon" }
[inputs."vendor/install-app"]
application = { rule = { node = "addon", output = "application" } }
token = { rule = { node = "addon", output = "token" } }
"""
    )
    env_project.run(["bake", "image", "device", "--enable-mixin", "addon"])
    assert (project / "build/device/application").read_text() == "baseaddon"


def test_equal_priority_recipe_order_and_cache(env_project: BakeryBuilder) -> None:
    """Equal-priority recipes execute in a stable order and reuse the same layer."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    (project / "layers/device.toml").write_text(
        'root = true\nrecipes = ["gamma", "alpha", "beta"]\n'
    )
    for name in ("alpha", "beta", "gamma"):
        recipe = project / "recipes" / name
        (recipe / "steps").mkdir(parents=True)
        (recipe / "recipe.toml").write_text('description = "Same priority"\n')
        script = recipe / "steps/00-run.sh"
        script.write_text(
            '#!/bin/sh\nset -eu\nmkdir -p "$RUGIX_ROOT_DIR/etc"\n'
            f'printf {name} >> "$RUGIX_ROOT_DIR/etc/order"\n'
            'head -c 32 /dev/urandom > "$RUGIX_ROOT_DIR/etc/token"\n'
        )
        script.chmod(0o755)
    output = build_rule(env_project)
    assert (output / "rootfs/etc/order").read_text() == "alphabetagamma"
    token = (output / "rootfs/etc/token").read_bytes()
    for _ in range(5):
        assert (build_rule(env_project) / "rootfs/etc/token").read_bytes() == token


def test_unidentified_layer_runtime(env_project: BakeryBuilder) -> None:
    """Layer commands execute each time when Bakery cannot identify its runtime."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    (project / "layers/device.toml").write_text('root = true\nrecipes = ["base"]\n')
    script = project / "recipes/base/steps/00-run.sh"
    script.write_text(
        script.read_text() + 'head -c 32 /dev/urandom > "$RUGIX_ROOT_DIR/etc/token"\n'
    )
    launcher = project / "run-bakery"
    launcher.write_text(
        launcher.read_text().replace(
            '-e "RUGIX_BAKERY_IMAGE=${RUGIX_BAKERY_IMAGE}"', ""
        )
    )
    token = (build_rule(env_project) / "rootfs/etc/token").read_bytes()
    assert (build_rule(env_project) / "rootfs/etc/token").read_bytes() != token
    env_project.run(["bake", "layer", "device", "--arch", "amd64"])
    archive = next((project / ".rugix/layers").glob("*/system.tar"))
    with tarfile.open(archive) as layer:
        entry = layer.extractfile("./roots/system/etc/token")
        assert entry is not None and len(entry.read()) == 32


def test_explicit_layer_source_date(env_project: BakeryBuilder) -> None:
    """Explicit source dates select separate layers and permit reuse when restored."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    (project / "layers/device.toml").write_text('root = true\nrecipes = ["base"]\n')
    script = project / "recipes/base/steps/00-run.sh"
    script.write_text(
        script.read_text()
        + 'printf %s "$SOURCE_DATE_EPOCH" > "$RUGIX_ROOT_DIR/etc/application"\n'
        + 'head -c 32 /dev/urandom > "$RUGIX_ROOT_DIR/etc/token"\n'
    )

    def bake(date: str) -> tuple[str, bytes]:
        env_project.run(["bake", "image", "device", "--source-date", date])
        output = project / "build/device"
        return (output / "application").read_text(), (output / "token").read_bytes()

    value, token = bake("2024-01-01T00:00:00Z")
    assert value == "1704067200"
    assert bake("2024-01-01T00:00:00Z")[1] == token
    assert bake("2025-01-01T00:00:00Z")[0] == "1735689600"
    assert bake("2024-01-01T00:00:00Z")[1] == token


def test_install_step_directory_inputs(env_project: BakeryBuilder) -> None:
    """Directory artifacts are read-only inside chroot install steps and remain outside outputs."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    (project / "layers/device.toml").write_text(
        'root = true\nrecipes = ["bootstrap", "configure"]\n'
        '[inputs.configure]\nsettings = { path = "settings" }\n'
    )
    settings = project / "settings"
    settings.mkdir()
    (settings / "value").write_text("configured\n")
    for name in ("bootstrap", "configure"):
        (project / "recipes" / name / "steps").mkdir(parents=True)
    (project / "recipes/bootstrap/recipe.toml").write_text("priority = 10\n")
    bootstrap = project / "recipes/bootstrap/steps/00-run.sh"
    bootstrap.write_text(
        """#!/bin/sh
set -eu
mkdir -p "$RUGIX_ROOT_DIR/bin" "$RUGIX_ROOT_DIR/etc" "$RUGIX_ROOT_DIR/dev" "$RUGIX_ROOT_DIR/sys" "$RUGIX_ROOT_DIR/proc"
cp --dereference /bin/sh "$RUGIX_ROOT_DIR/bin/sh"
for library in $(ldd /bin/sh | awk '$2 == "=>" {print $3} $1 ~ /^\\// {print $1}'); do
    cp --parents --dereference "$library" "$RUGIX_ROOT_DIR"
done
"""
    )
    bootstrap.chmod(0o755)
    (project / "recipes/configure/recipe.toml").write_text(
        '[inputs]\nsettings = "directory"\n'
    )
    configure = project / "recipes/configure/steps/00-install.sh"
    configure.write_text(
        """#!/bin/sh
set -eu
if printf forbidden >> "$RECIPE_INPUTS_DIR/settings/value"; then exit 43; fi
IFS= read -r value < "$RECIPE_INPUTS_DIR/settings/value"
printf %s "$value" > /etc/configured
"""
    )
    configure.chmod(0o755)
    output = build_rule(env_project)
    assert (output / "rootfs/etc/configured").read_text() == "configured"
    assert not (output / "rootfs/run/rugix/bakery/recipe-inputs").exists()
    assert (settings / "value").read_text() == "configured\n"


def test_uncached_layer_outputs_remain_immutable(env_project: BakeryBuilder) -> None:
    """Repeated layer nodes retain their own artifacts throughout one graph execution."""
    project = env_project.project_dir
    prepare(project, use_env=False)
    (project / "layers/device.toml").write_text('root = true\nrecipes = ["base"]\n')
    recipe = project / "recipes/base/recipe.toml"
    recipe.write_text('[inputs]\nsource = "file"\n')
    script = project / "recipes/base/steps/00-run.sh"
    script.write_text(
        script.read_text()
        + 'cp "$RECIPE_INPUTS_DIR/source" "$RUGIX_ROOT_DIR/etc/source"\n'
        + 'head -c 32 /dev/urandom > "$RUGIX_ROOT_DIR/etc/token"\n'
    )
    (project / "rules/device/rule.toml").write_text(
        '[inputs]\nsource = "file"\nwait = "file"\n'
        '[execution]\ntype = "layer"\nlayer = "device"\narchitecture = "amd64"\n'
    )
    snapshot = project / "rules/snapshot"
    snapshot.mkdir()
    (snapshot / "rule.toml").write_text(
        """[inputs]
archive = "file"
[execution]
type = "run"
command = ["/bin/cp", "/run/rugix/bakery/work/inputs/archive", "/run/rugix/bakery/work/outputs/archive"]
[execution.outputs.archive]
kind = "file"
path = "archive"
"""
    )
    graph = project / "rules/pair"
    graph.mkdir()
    (graph / "rule.toml").write_text(
        """[execution]
type = "graph"
[execution.rules.first]
uses = "device"
inputs.source = { path = "source" }
inputs.wait = { path = "source" }
[execution.rules.snapshot]
uses = "snapshot"
inputs.archive = { rule = { node = "first", output = "archive" } }
[execution.rules.second]
uses = "device"
inputs.source = { path = "source" }
inputs.wait = { rule = { node = "snapshot", output = "archive" } }
[execution.outputs]
first = { rule = { node = "first", output = "archive" } }
second = { rule = { node = "second", output = "archive" } }
snapshot = { rule = { node = "snapshot", output = "archive" } }
"""
    )
    (project / "invocation.toml").write_text('uses = "pair"\n')
    launcher = project / "run-bakery"
    launcher.write_text(
        launcher.read_text().replace(
            '-e "RUGIX_BAKERY_IMAGE=${RUGIX_BAKERY_IMAGE}"', ""
        )
    )
    output = build_rule(env_project)
    assert (output / "first").read_bytes() == (output / "snapshot").read_bytes()
    assert (output / "first").read_bytes() != (output / "second").read_bytes()
