"""``rugix-bakery test`` runs workflows against systems booted in a VM."""

from __future__ import annotations

import subprocess

import pytest
from rugix_testkit import BakeryBuilder


@pytest.mark.usefixtures("ssh_key")
def test_workflow_passes(bakery: BakeryBuilder) -> None:
    """A workflow whose steps succeed passes."""
    bakery.run(["test", "boot"])


@pytest.mark.usefixtures("ssh_key")
def test_workflow_fails_with_failing_step(
    bakery: BakeryBuilder, capfd: pytest.CaptureFixture[str]
) -> None:
    """A workflow with a failing step fails because of that step."""
    with pytest.raises(subprocess.CalledProcessError):
        bakery.run(["test", "failing-step"])
    assert "script execution failed" in capfd.readouterr().err
