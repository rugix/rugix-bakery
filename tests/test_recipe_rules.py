"""Recipe-owned artifact defaults compose with layers, overrides, and caching."""

from __future__ import annotations

import json
import shutil
import subprocess
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder

from test_layer_rules import build_rule, prepare


def prepare_recipe_rules(project: Path, *, use_env: bool = False) -> None:
    """Give an imported recipe its own build rule and default input graph."""
    prepare(project, use_env=use_env)
    (project / "vendor/source").write_text("vendor-source")
    (project / "layers/device.toml").write_text(
        'parent = "base"\nrecipes = ["vendor/install-app"]\n'
        '[parameters."vendor/install-app"]\nmode = "layer"\ncount = 11\n'
        '[rules.application]\nuses = "unused-layer-rule"\n'
    )
    recipe = project / "vendor/recipes/install-app"
    (recipe / "recipe.toml").write_text(
        """[inputs]
application = { kind = "file" }
token = { kind = "file" }
settings = { kind = "file" }
source = { kind = "file" }
[parameters.mode]
default = "recipe"
[parameters.enabled]
default = true
[parameters.count]
default = 7
[parameters.ratio]
default = 0.5
[rules.application]
uses = "app"
inputs.parent = { input = "rootfs" }
parameters.architecture = { parameter = "architecture" }
parameters.mode = { parameter = "mode" }
parameters.enabled = { parameter = "enabled" }
parameters.count = { parameter = "count" }
parameters.ratio = { parameter = "ratio" }
[input-defaults]
application = { rule = { node = "application", output = "application" } }
token = { rule = { node = "application", output = "token" } }
settings = { rule = { node = "application", output = "settings" } }
source = { path = "source" }
"""
    )
    script = recipe / "steps/00-run.sh"
    script.write_text(
        script.read_text()
        + 'cp "$RECIPE_INPUTS_DIR/settings" "$RUGIX_ROOT_DIR/etc/settings"\n'
        + 'printf %s "$RECIPE_PARAM_MODE" > "$RUGIX_ROOT_DIR/etc/recipe-mode"\n'
    )
    rule = project / "vendor/rules/app"
    with (rule / "rule.toml").open("a") as config:
        config.write(
            """
[parameters.mode]
default = "unused"
[parameters.enabled]
default = false
[parameters.count]
default = 0
[parameters.ratio]
default = 0.0
[execution.outputs.settings]
kind = "file"
path = "settings"
"""
        )
    with (rule / "run.sh").open("a") as stream:
        stream.write(
            "cp /run/rugix/bakery/work/parameters.json /run/rugix/bakery/work/outputs/settings\n"
        )
    with (project / "rules/image/rule.toml").open("a") as config:
        config.write(
            'settings = { select = { source = { input = "rootfs" }, '
            'path = "etc/settings", kind = "file" } }\n'
        )
    # A project namesake must not replace the recipe repository's implementation.
    shadow = project / "rules/app"
    shadow.mkdir()
    (shadow / "rule.toml").write_text("invalid TOML")


@pytest.mark.parametrize("use_env", [False, True])
def test_recipe_rules_and_default_inputs(
    env_project: BakeryBuilder, use_env: bool
) -> None:
    """Defaults use their owner's rules/files and typed effective recipe parameters."""
    project = env_project.project_dir
    prepare_recipe_rules(project, use_env=use_env)
    output = build_rule(env_project)
    assert (output / "rootfs/etc/application").read_text() == "basevendor-source"
    expected = {
        "architecture": "amd64",
        "mode": "layer",
        "enabled": True,
        "count": 11,
        "ratio": 0.5,
    }
    assert json.loads((output / "rootfs/etc/settings").read_text()) == expected
    assert (output / "rootfs/etc/recipe-mode").read_text() == "layer"
    token = (output / "rootfs/etc/token").read_bytes()
    assert (build_rule(env_project) / "rootfs/etc/token").read_bytes() == token
    (project / "source").write_text("project-source")
    assert (build_rule(env_project) / "rootfs/etc/token").read_bytes() == token
    (project / "vendor/source").write_text("changed")
    output = build_rule(env_project)
    assert (output / "rootfs/etc/application").read_text() == "basechanged"
    assert (output / "rootfs/etc/token").read_bytes() != token

    with (project / "vendor/recipes/install-app/recipe.toml").open("a") as config:
        config.write('[parameters.architecture]\ndefault = "recipe-target"\n')
    expected["architecture"] = "recipe-target"
    output = build_rule(env_project)
    assert json.loads((output / "rootfs/etc/settings").read_text()) == expected

    parameters = project / "parameters.toml"
    parameters.write_text(
        '["vendor/install-app"]\nmode = "cli"\nenabled = false\nratio = 1.25\n'
    )
    env_project.run(["bake", "image", "device", "--param-file", "parameters.toml"])
    assert json.loads((project / "build/device/settings").read_text()) == {
        **expected,
        "mode": "cli",
        "enabled": False,
        "ratio": 1.25,
    }


def test_layer_overrides_recipe_defaults(env_project: BakeryBuilder) -> None:
    """Layer overrides feed local builds and eliminate unreachable default rules."""
    project = env_project.project_dir
    prepare_recipe_rules(project)
    layer = project / "layers/device.toml"
    with layer.open("a") as config:
        config.write('[inputs."vendor/install-app"]\nsource = { path = "source" }\n')
    assert (build_rule(env_project) / "rootfs/etc/application").read_text() == (
        "baseapplication"
    )
    (project / "prebuilt").write_text("prebuilt")
    with layer.open("a") as config:
        for name in ("application", "token", "settings"):
            config.write(f'{name} = {{ path = "prebuilt" }}\n')
    (project / "vendor/rules/app/rule.toml").unlink()
    output = build_rule(env_project)
    assert (output / "rootfs/etc/application").read_text() == "prebuilt"
    assert (output / "rootfs/etc/settings").read_text() == "prebuilt"


def test_forwarded_inputs_override_recipe_defaults(env_project: BakeryBuilder) -> None:
    """Same-name layer rule inputs override defaults and feed recipe-local rules."""
    project = env_project.project_dir
    prepare_recipe_rules(project)
    wrapper = project / "rules/device/rule.toml"
    wrapper.write_text('[inputs]\nsource = { kind = "file" }\n' + wrapper.read_text())
    (project / "invocation.toml").write_text(
        'uses = "device"\ninputs.source = { path = "source" }\n'
    )
    assert (build_rule(env_project) / "rootfs/etc/application").read_text() == (
        "baseapplication"
    )


def test_recipe_rule_scopes_and_mixins(env_project: BakeryBuilder) -> None:
    """Namesakes remain recipe-local and defaults also work in mixin applications."""
    project = env_project.project_dir
    prepare_recipe_rules(project)
    other = project / "vendor/recipes/other"
    shutil.copytree(project / "vendor/recipes/install-app", other)
    config = other / "recipe.toml"
    config.write_text(
        config.read_text().replace('default = "recipe"', 'default = "other"')
    )
    script = other / "steps/00-run.sh"
    script.write_text(
        script.read_text().replace(
            '"$RUGIX_ROOT_DIR/etc/', '"$RUGIX_ROOT_DIR/etc/other-'
        )
    )
    layer = project / "layers/device.toml"
    layer.write_text(
        layer.read_text().replace(
            'recipes = ["vendor/install-app"]',
            'recipes = ["vendor/install-app", "vendor/other"]',
        )
    )
    output = build_rule(env_project)
    assert json.loads((output / "rootfs/etc/settings").read_text())["mode"] == "layer"
    assert (
        json.loads((output / "rootfs/etc/other-settings").read_text())["mode"]
        == "other"
    )
    assert (output / "rootfs/etc/token").read_bytes() != (
        output / "rootfs/etc/other-token"
    ).read_bytes()

    (project / "mixins").mkdir()
    (project / "mixins/addon.toml").write_text(
        'recipes = ["vendor/install-app"]\n'
        '[parameters."vendor/install-app"]\nmode = "mixin"\n'
    )
    env_project.run(["bake", "image", "device", "--enable-mixin", "addon"])
    assert (
        json.loads((project / "build/device/settings").read_text())["mode"] == "mixin"
    )


@pytest.mark.parametrize(
    "problem", ["unknown-default", "kind", "input-cycle", "rule-cycle", "layer-cycle"]
)
def test_recipe_default_validation(env_project: BakeryBuilder, problem: str) -> None:
    """Reject invalid defaults and mixed input/rule/layer cycles before execution."""
    project = env_project.project_dir
    prepare_recipe_rules(project)
    recipe = project / "vendor/recipes/install-app/recipe.toml"
    text = recipe.read_text()
    if problem == "unknown-default":
        text += 'typo = { path = "source" }\n'
    elif problem == "kind":
        text = text.replace(
            'application = { rule = { node = "application", output = "application" } }',
            'application = { path = "." }',
        )
    elif problem == "input-cycle":
        text = text.replace(
            'application = { rule = { node = "application", output = "application" } }',
            'application = { input = "token" }',
        ).replace(
            'token = { rule = { node = "application", output = "token" } }',
            'token = { input = "application" }',
        )
    elif problem == "rule-cycle":
        text = text.replace(
            'source = { path = "source" }',
            'source = { rule = { node = "application", output = "application" } }',
        )
    else:
        (project / "vendor/layers").mkdir()
        (project / "vendor/layers/cycle.toml").write_text(
            'root = true\nrecipes = ["install-app"]\n'
        )
        (project / "vendor/rules/app/rule.toml").write_text(
            '[execution]\ntype = "layer"\nlayer = "cycle"\narchitecture = "amd64"\n'
        )
        start = text.index("[rules.application]")
        end = text.index("[input-defaults]")
        text = text[:start] + '[rules.application]\nuses = "app"\n' + text[end:]
    recipe.write_text(text)
    with pytest.raises(subprocess.CalledProcessError):
        build_rule(env_project)
    assert not (project / ".rugix/rules").exists()
    assert not (project / "build/layer").exists()
