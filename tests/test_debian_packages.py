"""``core/rugix-ctrl`` installs released Debian packages of Rugix Ctrl.

These tests download the latest Rugix Ctrl 1.x release from GitHub.
"""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path

import pytest
from rugix_testkit import BakeryBuilder, RugixCtrl, VMHandle

PACKAGES = {
    "debian-package-musl": "rugix-ctrl-musl",
    "debian-package-gnu": "rugix-ctrl-gnu",
}


@pytest.fixture(params=sorted(PACKAGES))
def boot_system(request: pytest.FixtureRequest) -> str:
    return str(request.param)


@pytest.mark.extended
def test_debian_package_updates_system(
    vm: VMHandle,
    rugix: RugixCtrl,
    bakery: BakeryBuilder,
    boot_system: str,
    bundle_url: Callable[[Path], str],
) -> None:
    """The packaged Rugix Ctrl installs and commits an update."""
    vm.run(["dpkg", "-s", PACKAGES[boot_system]], hide=True)
    rugix.assert_boot(default="a", active="a")

    rugix.install_and_reboot(bundle_url(bakery.bake_bundle(boot_system)))
    rugix.assert_boot(default="a", active="b")

    rugix.system_commit()
    rugix.assert_boot(default="b", active="b")
