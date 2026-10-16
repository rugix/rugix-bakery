"""Systems for the ``generic-grub-efi`` target boot and update from baked bundles."""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder, RugixCtrl, VMHandle


@pytest.fixture(params=["efi", "efi-squashfs"])
def boot_system(request: pytest.FixtureRequest) -> str:
    return str(request.param)


def test_efi_system_updates_from_baked_bundle(
    vm: VMHandle,
    rugix: RugixCtrl,
    bakery: BakeryBuilder,
    boot_system: str,
    bundle_url: Callable[[Path], str],
) -> None:
    """Boot an image, install the bundle of the same system, and commit it."""
    filesystem = "squashfs" if boot_system == "efi-squashfs" else "ext4"
    info = rugix.assert_boot(default="a", active="a")
    assert info.boot_flow == "grub"
    assert _system_filesystem(vm) == filesystem

    rugix.install_and_reboot(bundle_url(bakery.bake_bundle(boot_system)))
    rugix.assert_boot(default="a", active="b")
    assert _system_filesystem(vm) == filesystem

    rugix.system_commit()
    rugix.assert_boot(default="b", active="b")


def _system_filesystem(vm: VMHandle) -> str:
    result = vm.run(["findmnt", "-no", "FSTYPE", "/run/rugix/mounts/system"], hide=True)
    return result.stdout.strip()
