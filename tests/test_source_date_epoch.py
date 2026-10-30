"""Source epochs propagate through graphs and participate in caching by default."""

from __future__ import annotations

import json
import subprocess

import pytest
from rugix_testkit import BakeryBuilder


@pytest.mark.parametrize("use_env", [False, True])
def test_source_epoch_context(
    env_project: BakeryBuilder, monkeypatch: pytest.MonkeyPatch, use_env: bool
) -> None:
    """Nested graphs inherit the epoch; opt-outs remove image defaults and retain cached results."""
    project = env_project.project_dir
    monkeypatch.delenv("SOURCE_DATE_EPOCH", raising=False)
    dockerfile = project / "envs/tools/Dockerfile"
    dockerfile.write_text(dockerfile.read_text() + "ENV SOURCE_DATE_EPOCH=123\n")
    command = [
        "/bin/sh",
        "-ec",
        (
            'printf %s "${SOURCE_DATE_EPOCH-unset}" > /run/rugix/bakery/work/outputs/epoch; '
            "head -c 32 /dev/urandom > /run/rugix/bakery/work/outputs/token"
        ),
    ]
    for name, settings in [
        ("consume", ""),
        ("ignore", "uses-source-date-epoch = false\n"),
    ]:
        rule = project / "rules" / name
        rule.mkdir(parents=True)
        (rule / "rule.toml").write_text(
            '[execution]\ntype = "run"\n'
            + ('env = "tools"\n' if use_env else "")
            + settings
            + "command = "
            + json.dumps(command)
            + '\n[execution.outputs.epoch]\nkind = "file"\npath = "epoch"\n'
            + '[execution.outputs.token]\nkind = "file"\npath = "token"\n'
        )
    graph = project / "rules/pair"
    graph.mkdir()
    (graph / "rule.toml").write_text(
        '[execution]\ntype = "graph"\n'
        '[execution.rules.consume]\nuses = "consume"\n'
        '[execution.rules.ignore]\nuses = "ignore"\n'
        "[execution.outputs]\n"
        'epoch = { rule = { node = "consume", output = "epoch" } }\n'
        'token = { rule = { node = "consume", output = "token" } }\n'
        'ignored = { rule = { node = "ignore", output = "epoch" } }\n'
        'stable = { rule = { node = "ignore", output = "token" } }\n'
    )
    outer = project / "rules/outer"
    outer.mkdir()
    (outer / "rule.toml").write_text(
        '[execution]\ntype = "graph"\n'
        '[execution.rules.pair]\nuses = "pair"\n'
        "[execution.outputs]\n"
        + "".join(
            f'{name} = {{ rule = {{ node = "pair", output = "{name}" }} }}\n'
            for name in ("epoch", "token", "ignored", "stable")
        )
    )
    (project / "invocation.toml").write_text('uses = "outer"\n')

    def bake(*options: str) -> dict[str, bytes]:
        env_project.run(["bake", "rule", "invocation.toml", "build/result", *options])
        return {
            path.name: path.read_bytes()
            for path in (project / "build/result").iterdir()
        }

    initial = bake()
    assert initial["epoch"] == b"0"
    assert initial["ignored"] == b"unset"
    assert bake() == initial
    monkeypatch.setenv("SOURCE_DATE_EPOCH", "1704067200")
    changed = bake()
    assert changed["epoch"] == b"1704067200"
    assert changed["token"] != initial["token"]
    assert changed["stable"] == initial["stable"]
    assert changed["ignored"] == b"unset"
    monkeypatch.setenv("SOURCE_DATE_EPOCH", "1735689600")
    assert bake("--source-date", "2024-01-01T00:00:00Z") == changed
    monkeypatch.setenv("SOURCE_DATE_EPOCH", "invalid")
    assert bake("--source-date", "2024-01-01T00:00:00Z") == changed
    for value in ("invalid", "-1", "9223372036854775808"):
        monkeypatch.setenv("SOURCE_DATE_EPOCH", value)
        with pytest.raises(subprocess.CalledProcessError):
            bake()
        assert (project / "build/result/token").read_bytes() == changed["token"]
    with pytest.raises(subprocess.CalledProcessError):
        bake("--source-date", "1969-12-31T23:59:59Z")
    monkeypatch.delenv("SOURCE_DATE_EPOCH")
    assert bake() == initial
