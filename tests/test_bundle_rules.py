"""Package update bundles using Bakery's installed tooling."""

from __future__ import annotations

import hashlib
import shutil

from rugix_testkit import BakeryBuilder


def test_bundle_rule_uses_bakery_tooling(env_project: BakeryBuilder) -> None:
    """Build independent byte-identical bundles and extract their original payload."""
    project = env_project.project_dir
    payload = hashlib.shake_256(b"bundle payload").digest(128 * 1024)
    (project / "data.bin").write_bytes(payload)
    (project / "manifest.toml").write_text(
        'update-type = "full"\n[[payloads]]\nfilename = "data.bin"\n'
        'delivery = { type = "slot", slot = "system" }\n'
        'block-encoding = { chunker = "casync-64", deduplicate = true }\n'
    )
    (project / "invocation.toml").write_text(
        'uses = "core/bundle-rugix"\ninputs.manifest = { path = "manifest.toml" }\n'
        'inputs.payloads = { tree = { "data.bin" = { path = "data.bin" } } }\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/release"])
    bundle = project / "build/release/bundle"
    first = bundle.read_bytes()
    env_project.run(
        ["bundler", "extract", "build/release/bundle", "0", "extracted.bin"]
    )
    assert (project / "extracted.bin").read_bytes() == payload
    shutil.rmtree(project / ".rugix/rules")
    env_project.run(["bake", "rule", "invocation.toml", "build/release"])
    assert bundle.read_bytes() == first
