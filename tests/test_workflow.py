"""``rugix-bakery test`` runs workflows against systems booted in a VM.

The workflows in ``tests/`` use the testkit's SSH key, which the parameter
overrides passed with ``--param-file`` authorize on the system.
"""

from __future__ import annotations

import subprocess

import pytest
from rugix_testkit import BakeryBuilder


def test_workflow_passes(bakery: BakeryBuilder) -> None:
    """Steps run over SSH, read stdin from a file, tolerate allowed failures
    and disconnects, and reconnect after a reboot."""
    bakery.test("boot")


def test_workflow_fails_with_failing_step(
    bakery: BakeryBuilder, capfd: pytest.CaptureFixture[str]
) -> None:
    """A workflow fails when a step exits with a non-zero code."""
    with pytest.raises(subprocess.CalledProcessError):
        bakery.test("failing-step")
    assert "non-zero return code 42" in capfd.readouterr().err
