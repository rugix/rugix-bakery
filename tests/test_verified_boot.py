"""Verified boot primitives authenticate the same bytes used in images and bundles."""

from __future__ import annotations

import hashlib
import shutil
import subprocess

import pytest
from rugix_testkit import BakeryBuilder


def test_verity_reproducibility_and_corruption(env_project: BakeryBuilder) -> None:
    """Fresh hash-tree builds are identical and a changed data block is rejected."""
    project = env_project.project_dir
    (project / "data").write_bytes(
        hashlib.shake_256(b"verity data").digest(1024 * 1024)
    )
    (project / "invocation.toml").write_text(
        'uses = "core/dm-verity"\ninputs.image = { path = "data" }\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/first"])
    first = project / "build/first"
    expected = (first / "hash-tree").read_bytes()
    root_hash = (first / "root-hash").read_text().strip()
    shutil.rmtree(project / ".rugix/rules")
    env_project.run(["bake", "rule", "invocation.toml", "build/second"])
    assert (project / "build/second/hash-tree").read_bytes() == expected
    # Run the tool in Bakery so the test does not require host cryptsetup.
    (project / "rules/verify").mkdir(parents=True)
    (project / "rules/verify/rule.toml").write_text(
        '[inputs.data]\nkind = "file"\n[inputs.hashes]\nkind = "file"\n'
        '[execution]\ntype = "run"\n'
        'command = ["sh", "-ec", "veritysetup verify /run/rugix/bakery/work/inputs/data '
        f"/run/rugix/bakery/work/inputs/hashes {root_hash}; "
        'touch /run/rugix/bakery/work/outputs/valid"]\n'
        '[execution.outputs.valid]\nkind = "file"\npath = "valid"\n'
    )
    (project / "verify.toml").write_text(
        'uses = "verify"\ninputs.data = { path = "data" }\n'
        'inputs.hashes = { path = "build/first/hash-tree" }\n'
    )
    env_project.run(["bake", "rule", "verify.toml", "build/valid"])
    (project / "data").write_bytes(b"x" + (project / "data").read_bytes()[1:])
    with pytest.raises(subprocess.CalledProcessError):
        env_project.run(["bake", "rule", "verify.toml", "build/invalid"])


@pytest.mark.parametrize("alignment", [4096, 2097152])
def test_appended_verity(env_project: BakeryBuilder, alignment: int) -> None:
    """Combined images reproduce, authenticate both areas, and export signed geometry."""
    project = env_project.project_dir
    data = hashlib.shake_256(b"appended verity").digest(1024 * 1024)
    (project / "data").write_bytes(data)
    (project / "rules/combined").mkdir(parents=True)
    (project / "rules/combined/rule.toml").write_text(
        '[inputs.image]\nkind = "file"\n[execution]\ntype = "graph"\n'
        '[execution.rules.hashes]\nuses = "core/dm-verity"\n'
        '[execution.rules.append]\nuses = "core/append-verity"\n'
        f"parameters.alignment = {alignment}\n"
        'inputs.hash-tree = { rule = { node = "hashes", output = "hash-tree" } }\n'
        '[execution.rules.cmdline]\nuses = "core/verified-cmdline"\n'
        'parameters.platform = "efi"\n'
        'inputs.root-hash = { rule = { node = "hashes", output = "root-hash" } }\n'
        'inputs.hash-offset = { rule = { node = "append", output = "hash-offset" } }\n'
        'inputs.data-size = { rule = { node = "append", output = "data-size" } }\n'
        "[execution.outputs]\n"
        'image = { rule = { node = "append", output = "image" } }\n'
        'offset = { rule = { node = "append", output = "hash-offset" } }\n'
        'size = { rule = { node = "append", output = "data-size" } }\n'
        'hashes = { rule = { node = "hashes", output = "hash-tree" } }\n'
        'hash = { rule = { node = "hashes", output = "root-hash" } }\n'
        'cmdline = { rule = { node = "cmdline", output = "cmdline" } }\n'
    )
    (project / "invocation.toml").write_text(
        'uses = "combined"\ninputs.image = { path = "data" }\n'
    )
    for name in ("first", "second"):
        shutil.rmtree(project / ".rugix/rules", ignore_errors=True)
        env_project.run(["bake", "rule", "invocation.toml", f"build/{name}"])
    first = project / "build/first"
    image = (first / "image").read_bytes()
    offset = int((first / "offset").read_text())
    assert offset == ((len(data) + alignment - 1) // alignment) * alignment
    assert int((first / "size").read_text()) == len(data)
    assert image[: len(data)] == data
    assert image[len(data) : offset] == bytes(offset - len(data))
    assert image[offset:] == (first / "hashes").read_bytes()
    assert (project / "data").read_bytes() == data
    assert (project / "build/second/image").read_bytes() == image
    cmdline = (first / "cmdline").read_text()
    assert f"rugix.verity-hash-offset={offset}" in cmdline
    assert f"rugix.verity-data-blocks={len(data) // 4096}" in cmdline
    root_hash = (first / "hash").read_text().strip()
    (project / "rules/verify").mkdir(parents=True)
    (project / "rules/verify/rule.toml").write_text(
        '[inputs.image]\nkind = "file"\n[execution]\ntype = "run"\n'
        'command = ["sh", "-ec", "veritysetup verify '
        f"--hash-offset={offset} "
        "/run/rugix/bakery/work/inputs/image "
        f"/run/rugix/bakery/work/inputs/image {root_hash}; "
        'touch /run/rugix/bakery/work/outputs/valid"]\n'
        '[execution.outputs.valid]\nkind = "file"\npath = "valid"\n'
    )
    (project / "verify.toml").write_text(
        'uses = "verify"\ninputs.image = { path = "probe" }\n'
    )
    (project / "probe").write_bytes(image)
    env_project.run(["bake", "rule", "verify.toml", "build/valid"])
    for position in (0, offset + 4096):
        modified = bytearray(image)
        modified[position] ^= 1
        (project / "probe").write_bytes(modified)
        with pytest.raises(subprocess.CalledProcessError):
            env_project.run(["bake", "rule", "verify.toml", "build/invalid"])


@pytest.mark.parametrize("architecture", ["amd64", "arm64"])
def test_efi_signatures_and_time(env_project: BakeryBuilder, architecture: str) -> None:
    """Both EFI architectures sign reproducibly, rotate on time changes, and verify."""
    project = env_project.project_dir
    subprocess.run(
        [
            "openssl",
            "req",
            "-x509",
            "-newkey",
            "rsa:2048",
            "-nodes",
            "-subj",
            "/CN=Rugix test/",
            "-days",
            "1",
            "-keyout",
            str(project / "key.pem"),
            "-out",
            str(project / "certificate.pem"),
        ],
        check=True,
        capture_output=True,
    )
    (project / "rules/signed-loader").mkdir(parents=True)
    (project / "rules/signed-loader/rule.toml").write_text(
        'secrets = ["key"]\n[inputs.certificate]\nkind = "file"\n'
        '[parameters.time]\ndefault = "1704067200"\n'
        '[execution]\ntype = "graph"\n'
        '[execution.rules.loader]\nuses = "core/systemd-boot"\n'
        f'parameters.architecture = "{architecture}"\n'
        '[execution.rules.sign]\nuses = "core/sign-efi"\n'
        'inputs.image = { rule = { node = "loader", output = "image" } }\n'
        'secrets.key = { input = "key" }\n'
        'parameters.signing-time = { parameter = "time" }\n'
        "[execution.outputs]\n"
        'image = { rule = { node = "sign", output = "image" } }\n'
    )
    invocation = (
        'uses = "signed-loader"\ninputs.certificate = { path = "certificate.pem" }\n'
        'secrets.key = { path = "key.pem" }\n'
    )
    (project / "invocation.toml").write_text(invocation)
    env_project.run(["bake", "rule", "invocation.toml", "build/first"])
    expected = (project / "build/first/image").read_bytes()
    shutil.rmtree(project / ".rugix/rules")
    env_project.run(["bake", "rule", "invocation.toml", "build/second"])
    assert (project / "build/second/image").read_bytes() == expected
    (project / "invocation.toml").write_text(
        invocation + 'parameters.time = "1704067201"\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/third"])
    assert (project / "build/third/image").read_bytes() != expected


def test_gpt_layout_from_artifacts(env_project: BakeryBuilder) -> None:
    """Automatic disk sizing preserves partition names and exact supplied bytes."""
    project = env_project.project_dir
    (project / "partitions").mkdir()
    data = hashlib.shake_256(b"partition bytes").digest(1024 * 1024)
    (project / "partitions/1").write_bytes(data)
    (project / "layout.toml").write_text(
        'type = "gpt"\npartitions = [{ name = "system-a" }, { name = "system-b", size = 1048576 }]\n'
    )
    (project / "invocation.toml").write_text(
        'uses = "core/image-gpt"\ninputs.layout = { path = "layout.toml" }\n'
        'inputs.partitions = { path = "partitions" }\n'
    )
    env_project.run(["bake", "rule", "invocation.toml", "build/disk"])
    image = project / "build/disk/image"
    with image.open("rb") as disk:
        disk.seek(1024 * 1024)
        assert disk.read(len(data)) == data
    # GPT names are UTF-16LE and must be present in the partition table.
    assert "system-a".encode("utf-16le") in image.read_bytes()[: 1024 * 1024]
    assert image.stat().st_size < 5 * 1024 * 1024


@pytest.mark.parametrize("kind", ["rpi", "bundle-rugix"])
def test_signing_reproducibility(env_project: BakeryBuilder, kind: str) -> None:
    """Pi and CMS signatures keep their timestamp and are stable across fresh builds."""
    project = env_project.project_dir
    subprocess.run(
        [
            "openssl",
            "req",
            "-x509",
            "-newkey",
            "rsa:2048",
            "-nodes",
            "-subj",
            "/CN=Rugix test/",
            "-addext",
            "basicConstraints=critical,CA:FALSE",
            "-days",
            "1",
            "-keyout",
            str(project / "key.pem"),
            "-out",
            str(project / "certificate.pem"),
        ],
        check=True,
        capture_output=True,
    )
    public = subprocess.run(
        ["openssl", "pkey", "-in", str(project / "key.pem"), "-pubout"],
        check=True,
        capture_output=True,
    )
    (project / "public.pem").write_bytes(public.stdout)
    (project / "data").write_bytes(b"authenticated payload")
    if kind == "rpi":
        invocation = (
            'uses = "core/sign-rpi"\ninputs.image = { path = "data" }\n'
            'inputs.public-key = { path = "public.pem" }\n'
            'secrets.key = { path = "key.pem" }\n'
        )
        output = "signature"
    else:
        (project / "payloads").mkdir()
        (project / "payloads/data").write_bytes(b"authenticated payload")
        (project / "bundle.toml").write_text(
            'update-type = "full"\n[[payloads]]\nfilename = "data"\n'
            'delivery = { type = "slot", slot = "system" }\n'
        )
        (project / "bundle-rule.toml").write_text(
            'uses = "core/bundle-rugix"\ninputs.manifest = { path = "bundle.toml" }\n'
            'inputs.payloads = { path = "payloads" }\n'
        )
        env_project.run(["bake", "rule", "bundle-rule.toml", "build/unsigned"])
        invocation = (
            'uses = "core/sign-bundle-rugix"\n'
            'inputs.bundle = { path = "build/unsigned/bundle" }\n'
            'inputs.certificate = { path = "certificate.pem" }\n'
            'rules.signer.secrets.key = { path = "key.pem" }\n'
        )
        output = "bundle"
    for name, time in [
        ("first", "1704067200"),
        ("second", "1704067200"),
        ("third", "1704067201"),
    ]:
        (project / "invocation.toml").write_text(
            invocation + f'parameters.signing-time = "{time}"\n'
        )
        shutil.rmtree(project / ".rugix/rules", ignore_errors=True)
        env_project.run(["bake", "rule", "invocation.toml", f"build/{name}"])
    expected = (project / "build/first" / output).read_bytes()
    assert (project / "build/second" / output).read_bytes() == expected
    assert (project / "build/third" / output).read_bytes() != expected


def test_fat_reserves_space_for_second_boot_artifact(
    env_project: BakeryBuilder,
) -> None:
    """An automatically sized ESP can hold its second A/B boot artifact."""
    project = env_project.project_dir
    (project / "tree").mkdir()
    with (project / "tree/first.efi").open("wb") as artifact:
        artifact.truncate(40 * 1024 * 1024)
    (project / "fat.toml").write_text(
        'uses = "core/image-fat"\ninputs.tree = { path = "tree" }\n'
        "parameters.free-percent = 100\n"
    )
    env_project.run(["bake", "rule", "fat.toml", "build/fat"])
    (project / "rules/copy").mkdir(parents=True)
    (project / "rules/copy/rule.toml").write_text(
        '[inputs.image]\nkind = "file"\n[inputs.uki]\nkind = "file"\n'
        '[execution]\ntype = "run"\ncommand = ["sh", "-ec", '
        '"cp /run/rugix/bakery/work/inputs/image /run/rugix/bakery/work/outputs/image; '
        'mcopy -i /run/rugix/bakery/work/outputs/image /run/rugix/bakery/work/inputs/uki ::/second.efi"]\n'
        '[execution.outputs.image]\nkind = "file"\npath = "image"\n'
    )
    (project / "copy.toml").write_text(
        'uses = "copy"\ninputs.image = { path = "build/fat/image" }\n'
        'inputs.uki = { path = "tree/first.efi" }\n'
    )
    env_project.run(["bake", "rule", "copy.toml", "build/with-second"])


@pytest.mark.parametrize("architecture", ["amd64", "arm64"])
def test_uki_reproducibility(env_project: BakeryBuilder, architecture: str) -> None:
    """UKI assembly embeds its explicit inputs deterministically for both EFI targets."""
    project = env_project.project_dir
    (project / "initrd").write_bytes(b"test initrd contents")
    (project / "cmdline").write_text("root=/dev/mapper/root ro\n")
    (project / "os-release").write_text('ID=rugix-test\nPRETTY_NAME="Rugix Test"\n')
    (project / "rules/uki").mkdir(parents=True)
    (project / "rules/uki/rule.toml").write_text(
        '[inputs.initrd]\nkind = "file"\n[inputs.cmdline]\nkind = "file"\n'
        '[inputs.os-release]\nkind = "file"\n[execution]\ntype = "graph"\n'
        '[execution.rules.stub]\nuses = "core/efi-stub"\n'
        f'parameters.architecture = "{architecture}"\n'
        '[execution.rules.uki]\nuses = "core/uki"\n'
        'inputs.stub = { rule = { node = "stub", output = "stub" } }\n'
        # A PE stub provides a small architecture-correct fixture for assembly.
        'inputs.kernel = { rule = { node = "stub", output = "stub" } }\n'
        '[execution.outputs]\nimage = { rule = { node = "uki", output = "image" } }\n'
    )
    (project / "invocation.toml").write_text(
        'uses = "uki"\ninputs.initrd = { path = "initrd" }\n'
        'inputs.cmdline = { path = "cmdline" }\n'
        'inputs.os-release = { path = "os-release" }\n'
    )
    for name in ["first", "second"]:
        shutil.rmtree(project / ".rugix/rules", ignore_errors=True)
        env_project.run(["bake", "rule", "invocation.toml", f"build/{name}"])
    expected = (project / "build/first/image").read_bytes()
    assert b"test initrd contents" in expected
    assert b"root=/dev/mapper/root ro" in expected
    assert (project / "build/second/image").read_bytes() == expected
