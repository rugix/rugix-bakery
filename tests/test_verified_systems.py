"""Verified systems boot only authenticated boot artifacts and roots."""

from __future__ import annotations

import os
import shutil
import struct
import subprocess
from collections.abc import Callable
from pathlib import Path

import pytest
from rugix_testkit import (
    BakeryBuilder,
    Drive,
    Pflash,
    QemuVM,
    RugixCtrl,
    VMConfig,
    VMHandle,
)
from rugix_testkit.locks import exclusive_lock

SYSTEM = "efi-verified"
EFI_GLOBAL = "8be4df61-93ca-11d2-aa0d-00e098032b8c"


@pytest.fixture(scope="module")
def verified_keys(rugix_project: Path) -> Path:
    """Boot and bundle signing keys referenced by the project's verified system."""
    keys = rugix_project / ".rugix-testkit" / "verified-keys"
    with exclusive_lock(keys.parent / "verified-keys.lock"):
        if not (keys / "bundle.crt").exists():
            _create_keys(keys)
    return keys


def _create_keys(keys: Path) -> None:
    """Create the boot certificate and a bundle signer issued by a bundle root."""
    keys.mkdir(parents=True, exist_ok=True)
    _certificate(keys / "boot.key", keys / "boot.crt")
    _certificate(keys / "bundle-root.key", keys / "bundle-root.crt")
    _openssl("genrsa", "-out", str(keys / "bundle.key"), "2048")
    _openssl(
        "req",
        "-new",
        "-key",
        str(keys / "bundle.key"),
        "-subj",
        "/CN=Rugix test signer/",
        "-out",
        str(keys / "bundle.csr"),
    )
    (keys / "bundle.ext").write_text(
        "basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\n"
    )
    _openssl(
        "x509",
        "-req",
        "-in",
        str(keys / "bundle.csr"),
        "-CA",
        str(keys / "bundle-root.crt"),
        "-CAkey",
        str(keys / "bundle-root.key"),
        "-CAcreateserial",
        "-days",
        "3650",
        "-sha256",
        "-extfile",
        str(keys / "bundle.ext"),
        "-out",
        str(keys / "bundle.crt"),
    )


@pytest.fixture(scope="module")
def secure_boot_firmware(
    verified_keys: Path, tmp_path_factory: pytest.TempPathFactory
) -> tuple[Path, Path]:
    """OVMF code with Secure Boot and variables enforcing the boot certificate."""
    code, template = _ovmf_secure_boot()
    variables = tmp_path_factory.mktemp("secure-boot") / "vars.fd"
    certificate = str(verified_keys / "boot.crt")
    owner = "c1d62147-383a-4c03-833a-34e9c2db4df7"
    subprocess.run(
        [
            "virt-fw-vars",
            "--input",
            str(template),
            "--output",
            str(variables),
            "--set-pk",
            owner,
            certificate,
            "--add-kek",
            owner,
            certificate,
            "--add-db",
            owner,
            certificate,
            "--secure-boot",
        ],
        check=True,
        capture_output=True,
    )
    return code, variables


def test_efi_verified_inventory_selection(
    bakery: BakeryBuilder, verified_keys: Path
) -> None:
    """A verified system can publish its independent SBOM without an image or bundle."""
    bakery.run(["bake", "system", SYSTEM, "--artifact", "sbom.spdx.json"])
    output = bakery.project_dir / "build" / SYSTEM
    assert {path.name for path in output.iterdir()} == {
        "sbom.spdx.json",
        "build-context.json",
    }


def test_efi_verified_boot_and_update(
    bakery: BakeryBuilder,
    start_vm: Callable[[VMConfig], VMHandle],
    bundle_url: Callable[[Path], str],
    verified_keys: Path,
    secure_boot_firmware: tuple[Path, Path],
) -> None:
    """The system boots with Secure Boot and updates, rolls back, and commits slot B."""
    bakery.run(["bake", "system", SYSTEM])
    output = bakery.project_dir / "build" / SYSTEM
    image = output / "system.img"
    assert (output / "sbom.spdx.json").is_file()
    root_hash = (output / "root-hash").read_text().strip()
    vm = start_vm(_config(image, secure_boot_firmware))
    rugix = RugixCtrl(vm)
    assert rugix.assert_boot(default="a", active="a").boot_flow == "systemd-boot"
    _assert_verified(vm, root_hash, partition=2)

    url = bundle_url(output / "system.rugixb")
    _install(vm, rugix, url, verified_keys)
    rugix.assert_boot(default="a", active="b")
    _assert_verified(vm, root_hash, partition=3)
    vm.reboot()
    rugix.assert_boot(default="a", active="a")

    _install(vm, rugix, url, verified_keys)
    rugix.system_commit()
    vm.reboot()
    rugix.assert_boot(default="b", active="b")
    _assert_verified(vm, root_hash, partition=3)


@pytest.mark.parametrize(
    ("change", "message"),
    [
        ("uki", "Error loading \\EFI\\Linux\\rugix-a.efi"),
        ("root", "data block 0 is corrupted"),
        ("second-disk", "Ambiguous verified root partition system-a"),
    ],
)
def test_efi_verified_boot_rejects(
    bakery: BakeryBuilder,
    start_qemu: Callable[[VMConfig], QemuVM],
    secure_boot_firmware: tuple[Path, Path],
    tmp_path: Path,
    change: str,
    message: str,
) -> None:
    """Firmware rejects a modified UKI; the initramfs rejects a modified or ambiguous root."""
    image = bakery.bake_image(SYSTEM)
    if change == "second-disk":
        # A copy of the device's own image carries a genuine root A.
        config = _config(image, secure_boot_firmware, disks=2)
    else:
        modified = tmp_path / "modified.img"
        subprocess.run(
            ["cp", "--sparse=always", "--reflink=auto", str(image), str(modified)],
            check=True,
        )
        partitions = _partitions(modified)
        with modified.open("r+b") as disk:
            if change == "uki":
                start, size = partitions["config"]
                disk.seek(start)
                marker = b"rugix.verity-root-hash="
                position = start + disk.read(size).index(marker) + len(marker)
            else:
                position = partitions["system-a"][0] + 1024
            disk.seek(position)
            original = disk.read(1)[0]
            disk.seek(position)
            disk.write(bytes([original ^ 1]))
        config = _config(modified, secure_boot_firmware)
    start_qemu(config).wait_for_serial(message, timeout=180)


def test_rpi_verified_release(env_project: BakeryBuilder) -> None:
    """The Raspberry Pi release signs its boot image and authenticates the root hash."""
    project = env_project.project_dir
    _certificate(project / "bundle.key", project / "bundle.crt")
    _openssl("genrsa", "-out", str(project / "rpi.key"), "2048")
    _openssl(
        "rsa",
        "-in",
        str(project / "rpi.key"),
        "-pubout",
        "-out",
        str(project / "rpi.pub"),
    )
    firmware = project / "firmware"
    (firmware / "overlays").mkdir(parents=True)
    for name in ("start4.elf", "fixup4.dat", "board.dtb", "overlays/board.dtbo"):
        (firmware / name).write_bytes(name.encode())
    (project / "rootfs/etc").mkdir(parents=True)
    (project / "rootfs/etc/hostname").write_text("rugix\n")
    (project / "kernel").write_bytes(b"kernel")
    (project / "initrd").write_bytes(b"initrd")
    (project / "invocation.toml").write_text(
        'uses = "core/image-rpi-verified"\n'
        'inputs.rootfs = { path = "rootfs" }\n'
        'inputs.kernel = { path = "kernel" }\n'
        'inputs.initrd = { path = "initrd" }\n'
        'inputs.boot = { path = "firmware" }\n'
        "inputs.config = { tree = {} }\n"
        'inputs.boot-public-key = { path = "rpi.pub" }\n'
        'inputs.bundle-certificate = { path = "bundle.crt" }\n'
        'secrets.boot-key = { path = "rpi.key" }\n'
        'secrets.bundle-key = { path = "bundle.key" }\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/release"])
    output = project / "build/release"
    image = output / "image"
    partitions = _partitions(image)
    assert list(partitions) == ["config", "boot-a", "boot-b", "system-a"]

    extracted = project / "extracted"
    extracted.mkdir()
    boot = f"{image}@@{partitions['boot-a'][0]}"
    config = f"{image}@@{partitions['config'][0]}"
    for source, name in [(boot, "boot.img"), (boot, "boot.sig"), (boot, "config.txt")]:
        _mcopy(source, name, extracted / name)
    _mcopy(config, "autoboot.txt", extracted / "autoboot.txt")
    _mcopy(str(extracted / "boot.img"), "cmdline.txt", extracted / "cmdline.txt")
    assert (extracted / "boot.img").read_bytes() == (output / "boot-image").read_bytes()
    assert "boot_ramdisk=1" in (extracted / "config.txt").read_text()
    assert "tryboot_a_b=1" in (extracted / "autoboot.txt").read_text()

    signature = next(
        line.removeprefix("rsa2048: ")
        for line in (extracted / "boot.sig").read_text().splitlines()
        if line.startswith("rsa2048: ")
    )
    (extracted / "signature").write_bytes(bytes.fromhex(signature))
    _openssl(
        "dgst",
        "-sha256",
        "-verify",
        str(project / "rpi.pub"),
        "-signature",
        str(extracted / "signature"),
        str(extracted / "boot.img"),
    )
    arguments = (extracted / "cmdline.txt").read_text().split()
    root_hash = (output / "root-hash").read_text().strip()
    assert f"rugix.verity-root-hash={root_hash}" in arguments
    assert "rugix.platform=rpi" in arguments


def _install(vm: VMHandle, rugix: RugixCtrl, url: str, keys: Path) -> None:
    """Install the signed bundle, verifying it against its root, and trial boot it."""
    vm.upload(keys / "bundle-root.crt", "/run/bundle-root.crt")
    rugix.install_and_reboot(url, insecure=False, root_cert="/run/bundle-root.crt")


def _assert_verified(vm: VMHandle, root_hash: str, *, partition: int) -> None:
    """Secure Boot is enforced and the root is the verity device on the selected partition."""
    secure_boot = vm.run(
        ["od", "-An", "-t", "u1", f"/sys/firmware/efi/efivars/SecureBoot-{EFI_GLOBAL}"],
        hide=True,
    )
    assert secure_boot.stdout.split()[-1] == "1"
    table = vm.run(["dmsetup", "table", "root"], hide=True).stdout.split()
    assert table[2:4] == ["verity", "1"], table
    assert table[11] == root_hash, table
    dependencies = vm.run(["dmsetup", "deps", "-o", "devname", "root"], hide=True)
    assert f"(vda{partition})" in dependencies.stdout


def _config(image: Path, firmware: tuple[Path, Path], *, disks: int = 1) -> VMConfig:
    """A Secure Boot machine booting copy-on-write overlays of *image*."""
    code, variables = firmware
    return VMConfig(
        arch="x86_64",
        machine="q35,smm=on",
        memory=2048,
        smp=2,
        drives=[
            Drive(
                file=image, format="raw", interface="virtio", overlay=True, size="16G"
            )
            for _ in range(disks)
        ],
        pflash=[
            Pflash(file=code, format="raw", readonly=True),
            Pflash(file=variables, format="raw", readonly=False),
        ],
        extra_args=[
            "-global",
            "driver=cfi.pflash01,property=secure,value=on",
            "-device",
            "virtio-rng-pci",
        ],
    )


def _ovmf_secure_boot() -> tuple[Path, Path]:
    """Locate OVMF code with Secure Boot support and a matching variable template."""
    code = os.environ.get("RUGIX_OVMF_CODE_SECBOOT_AMD64")
    variables = os.environ.get("RUGIX_OVMF_VARS_AMD64")
    if code and variables:
        return Path(code), Path(variables)
    candidates = [
        (
            Path("/usr/share/OVMF/OVMF_CODE_4M.secboot.fd"),
            Path("/usr/share/OVMF/OVMF_VARS_4M.fd"),
        )
    ]
    qemu = shutil.which("qemu-system-x86_64")
    if qemu is not None:
        share = Path(qemu).resolve().parent.parent / "share" / "qemu"
        candidates.insert(
            0, (share / "edk2-x86_64-secure-code.fd", share / "edk2-i386-vars.fd")
        )
    for code_path, variables_path in candidates:
        if code_path.exists() and variables_path.exists():
            return code_path, variables_path
    pytest.fail(
        "Could not locate OVMF firmware with Secure Boot. Set "
        "RUGIX_OVMF_CODE_SECBOOT_AMD64 and RUGIX_OVMF_VARS_AMD64."
    )


def _partitions(image: Path) -> dict[str, tuple[int, int]]:
    """GPT partition names mapped to their byte offset and size."""
    with image.open("rb") as disk:
        disk.seek(512)
        header = disk.read(92)
        assert header[:8] == b"EFI PART"
        entries, count, entry_size = struct.unpack_from("<QII", header, 72)
        disk.seek(entries * 512)
        table = disk.read(count * entry_size)
    partitions: dict[str, tuple[int, int]] = {}
    for index in range(count):
        entry = table[index * entry_size : (index + 1) * entry_size]
        if entry[:16] == bytes(16):
            continue
        first, last = struct.unpack_from("<QQ", entry, 32)
        name = entry[56:128].decode("utf-16-le").rstrip("\0")
        partitions[name] = (first * 512, (last - first + 1) * 512)
    return partitions


def _certificate(key: Path, certificate: Path) -> None:
    _openssl(
        "req",
        "-x509",
        "-newkey",
        "rsa:2048",
        "-nodes",
        "-sha256",
        "-subj",
        "/CN=Rugix test/",
        "-days",
        "3650",
        "-keyout",
        str(key),
        "-out",
        str(certificate),
    )


def _openssl(*arguments: str) -> None:
    subprocess.run(["openssl", *arguments], check=True, capture_output=True)


def _mcopy(source: str, name: str, destination: Path) -> None:
    subprocess.run(["mcopy", "-i", source, f"::{name}", str(destination)], check=True)
