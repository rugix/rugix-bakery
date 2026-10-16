"""Systems for the ``generic-grub-efi`` target boot and update from baked bundles."""

from __future__ import annotations

import json
from collections.abc import Callable
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder, RugixCtrl, VMHandle

SPECIAL_FILES = "/usr/share/rugix-test"


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
    """Boot an image, install the bundle of the same system, and commit it.
    Both systems keep the file metadata that the layer recipes set up."""
    filesystem = "squashfs" if boot_system == "efi-squashfs" else "ext4"
    info = rugix.assert_boot(default="a", active="a")
    assert info.boot_flow == "grub"
    assert _system_filesystem(vm) == filesystem
    vm.run(["test", "-s", "/etc/rugix/system-build-info.json"], hide=True)
    _assert_special_files(vm, filesystem)

    rugix.install_and_reboot(bundle_url(bakery.bake_bundle(boot_system)))
    rugix.assert_boot(default="a", active="b")
    assert _system_filesystem(vm) == filesystem
    _assert_special_files(vm, filesystem)

    rugix.system_commit()
    rugix.assert_boot(default="b", active="b")


def test_bake_writes_sboms(bakery: BakeryBuilder) -> None:
    """Baking a system writes SPDX and CycloneDX SBOMs of its packages."""
    output = bakery.bake_image("efi").parent
    for sbom in ("sbom.spdx.json", "sbom.cdx.json"):
        content = (output / sbom).read_text()
        json.loads(content)
        assert "openssh-server" in content, sbom


def _system_filesystem(vm: VMHandle) -> str:
    result = vm.run(["findmnt", "-no", "FSTYPE", "/run/rugix/mounts/system"], hide=True)
    return result.stdout.strip()


def _assert_special_files(vm: VMHandle, filesystem: str) -> None:
    """Check the files that the `special-files` recipe creates."""
    names = ("setuid", "original", "fifo")
    stat = vm.run(
        ["stat", "-c", "%n %F %A %h", *(f"{SPECIAL_FILES}/{name}" for name in names)],
        hide=True,
    )
    assert stat.stdout.splitlines() == [
        f"{SPECIAL_FILES}/setuid regular file -rwsr-xr-x 1",
        f"{SPECIAL_FILES}/original regular file -rw-r--r-- 2",
        f"{SPECIAL_FILES}/fifo fifo prw-r--r-- 1",
    ]
    original = f"{SPECIAL_FILES}/original"
    xattr = vm.run(
        ["getfattr", "-n", "user.rugix", "--only-values", original], hide=True
    )
    assert xattr.stdout == "test"
    capability = vm.run(["getcap", f"{SPECIAL_FILES}/capability"], hide=True)
    assert "cap_net_bind_service=ep" in capability.stdout
    # SquashFS does not support POSIX ACLs.
    acl = vm.run(["getfacl", "--omit-header", original], hide=True)
    assert ("user:nobody:r--" in acl.stdout) == (filesystem == "ext4")
