"""Inspect repository definitions without loading or executing their build dependencies."""

from __future__ import annotations

import json
import subprocess

import pytest
from rugix_testkit import BakeryBuilder

from test_layer_rules import build_rule
from test_recipe_rules import prepare_recipe_rules


@pytest.mark.parametrize("kind", ["rules", "recipes", "layers"])
def test_repository_definition_catalog(
    env_project: BakeryBuilder, capfd: pytest.CaptureFixture[str], kind: str
) -> None:
    """Export the selected repository's definitions and notes without resolving dependencies."""
    project = env_project.project_dir
    directory = project / "vendor" / kind
    directory.mkdir()
    if kind == "layers":
        (directory / "demo.toml").write_text(
            'description = "Demo layer"\nparent = "unavailable"\n'
        )
        (directory / "demo.md").write_text("## Usage\n\nLayer notes.\n")
    else:
        definition = directory / "demo"
        definition.mkdir()
        text = (
            'description = "Demo definition"\ncategory = "Examples"\n'
            '[inputs.asset]\nkind = "file"\ndescription = "Source asset"\n'
            '[parameters.enabled]\ndefault = false\ndescription = "Enable feature"\n'
        )
        if kind == "rules":
            text += (
                '[execution]\ntype = "run"\nenv = "unavailable"\n'
                'command = ["false"]\n'
                '[execution.outputs.result]\nkind = "file"\npath = "result"\n'
                'description = "Produced asset"\n'
            )
        filename = "rule.toml" if kind == "rules" else "recipe.toml"
        (definition / filename).write_text(text)
        (definition / "README.md").write_text("## Usage\n\nDefinition notes.\n")
        hidden = directory / ".hidden"
        hidden.mkdir()
        (hidden / filename).write_text("invalid TOML")
        (directory / "unfinished").mkdir()
    env_project.run([kind, "list", "--repository", "vendor"])
    assert capfd.readouterr().out.splitlines() == ["vendor/demo"]
    env_project.run([kind, "list", "--repository", "vendor", "--json"])
    catalog = json.loads(capfd.readouterr().out)
    assert (
        catalog["kind"]
        == {"rules": "rule", "recipes": "recipe", "layers": "layer"}[kind]
    )
    assert [item["name"] for item in catalog["definitions"]] == ["vendor/demo"]
    expected = catalog["definitions"][0]
    env_project.run([kind, "inspect", "vendor/demo", "--json"])
    assert json.loads(capfd.readouterr().out) == {"kind": catalog["kind"], **expected}
    assert "notes." in expected["documentation"]
    if kind != "layers":
        inspected = expected["definition"]
        assert inspected["inputs"]["asset"] == {
            "kind": "file",
            "description": "Source asset",
        }
        assert inspected["parameters"]["enabled"]["default"] is False
    env_project.run([kind, "inspect", "vendor/demo"])
    assert "notes." in capfd.readouterr().out
    for name in ("vendor/../demo", "vendor/demo/child", "missing/demo"):
        with pytest.raises(subprocess.CalledProcessError):
            env_project.run([kind, "inspect", name, "--json"])
        capfd.readouterr()
    with pytest.raises(subprocess.CalledProcessError):
        env_project.run([kind, "list", "--repository", "missing", "--json"])
    capfd.readouterr()


def test_layer_variants_are_complete_alternatives(
    env_project: BakeryBuilder, capfd: pytest.CaptureFixture[str]
) -> None:
    """Inspection retains the default and each architecture override without merging them."""
    directory = env_project.project_dir / "vendor/layers"
    directory.mkdir()
    (directory / "device.toml").write_text(
        'parent = "default-base"\nrecipes = ["default-recipe"]\n'
    )
    (directory / "device.arm64.toml").write_text('parent = "arm-base"\n')
    env_project.run(["layers", "list", "--repository", "vendor"])
    assert capfd.readouterr().out.splitlines() == ["vendor/device"]
    env_project.run(["layers", "inspect", "vendor/device", "--json"])
    variants = json.loads(capfd.readouterr().out)["definition"]["variants"]
    assert len(variants) == 2
    assert variants[0].get("architecture") is None
    assert variants[0]["config"]["recipes"] == ["default-recipe"]
    assert variants[1]["architecture"] == "arm64"
    assert variants[1]["config"]["parent"] == "arm-base"
    assert "recipes" not in variants[1]["config"]


@pytest.mark.parametrize("kind", ["rules", "recipes", "layers"])
def test_core_catalog_is_documented(
    env_project: BakeryBuilder, capfd: pytest.CaptureFixture[str], kind: str
) -> None:
    """Every core building block has a description, reference notes, and documented parameters."""
    env_project.run([kind, "list", "--repository", "core", "--json"])
    catalog = json.loads(capfd.readouterr().out)
    assert catalog["definitions"]
    names = [item["name"] for item in catalog["definitions"]]
    assert names == sorted(names)
    for item in catalog["definitions"]:
        assert item["name"].startswith("core/")
        assert item["documentation"].strip()
        configurations = (
            [variant["config"] for variant in item["definition"]["variants"]]
            if kind == "layers"
            else [item["definition"]]
        )
        for definition in configurations:
            assert definition["description"]
            for parameter in definition.get("parameters", {}).values():
                if kind != "layers":
                    assert parameter["description"]


def test_documented_recipe_inputs_execute(env_project: BakeryBuilder) -> None:
    """Documented artifact inputs preserve recipe-owned defaults and graph execution."""
    project = env_project.project_dir
    prepare_recipe_rules(project)
    recipe = project / "vendor/recipes/install-app/recipe.toml"
    recipe.write_text(
        recipe.read_text().replace(
            'application = { kind = "file" }',
            'application = { kind = "file", description = "Compiled application" }',
        )
    )
    output = build_rule(env_project)
    assert (output / "rootfs/etc/application").read_text() == "basevendor-source"
