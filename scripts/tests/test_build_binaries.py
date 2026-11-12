"""Check release archive contents using a fixture Cargo command."""

import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import unittest

BUILD_SCRIPT = Path(__file__).resolve().parents[1] / "build-binaries.sh"
BINARIES = ["rugix-bakery", "rugix-isolate", "rugix-bundler", "rugix-ctrl", "rugix-util"]


class BuildBinariesTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="bakery build ")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        (self.root / "scripts").mkdir()
        shutil.copy2(BUILD_SCRIPT, self.root / "scripts/build-binaries.sh")
        self.bin = self.root / "bin"
        self.bin.mkdir()
        cargo = self.bin / "cargo"
        cargo.write_text('''#!/usr/bin/env python3
import os
from pathlib import Path
import sys

target = sys.argv[sys.argv.index("--target") + 1]
names = ["rugix-bakery", "rugix-isolate", "rugix-bundler", "rugix-ctrl", "rugix-util"]
if sys.argv[1] == "build":
    assert "--locked" in sys.argv
    output = Path(os.environ.get("CARGO_TARGET_DIR", "target")) / target / "release"
    output.mkdir(parents=True, exist_ok=True)
    for name in names:
        (output / name).write_text(target)
else:
    assert sys.argv[1] == "cyclonedx"
    for name in names:
        if name == os.environ.get("MISSING_SBOM"):
            continue
        base = Path(".") if name in ["rugix-bakery", "rugix-isolate"] else Path("repos/rugix")
        output = base / "crates/apps" / name
        output.mkdir(parents=True, exist_ok=True)
        (output / (name + ".cdx.json")).write_text('{"bomFormat":"CycloneDX"}')
''')
        cargo.chmod(0o755)
        git = self.bin / "git"
        git.write_text("#!/bin/sh\nprintf '%s\\n' test-version\n")
        git.chmod(0o755)
        self.environment = {
            **os.environ,
            "PATH": str(self.bin) + os.pathsep + os.environ["PATH"],
            "CARGO_TARGET_DIR": str(self.root / "target with spaces"),
        }

    def build(self, *targets: str, **environment: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["bash", str(self.root / "scripts/build-binaries.sh"), *targets],
            cwd=self.root.parent,
            env={**self.environment, **environment},
            capture_output=True,
            text=True,
        )

    def assert_archive(self, target: str) -> None:
        archive = self.root / "build/binaries" / f"{target}.tar"
        with tarfile.open(archive) as contents:
            files = {member.name.removeprefix("./") for member in contents if member.isfile()}
            self.assertEqual(files, set(BINARIES + [name + ".cdx.json" for name in BINARIES]))
        for name in BINARIES:
            self.assertEqual((archive.parent / target / name).read_text(), target)

    def test_default_targets(self) -> None:
        """Build both container architectures with complete binary and SBOM archives."""
        result = self.build()
        self.assertEqual(result.returncode, 0, result.stderr)
        for target in ["aarch64-unknown-linux-musl", "x86_64-unknown-linux-musl"]:
            self.assert_archive(target)

    def test_explicit_target(self) -> None:
        """Honor an explicit target and a Cargo output directory containing spaces."""
        result = self.build("aarch64-unknown-linux-musl")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assert_archive("aarch64-unknown-linux-musl")
        self.assertFalse((self.root / "build/binaries/x86_64-unknown-linux-musl.tar").exists())

    def test_missing_sbom_fails(self) -> None:
        """Reject incomplete release artifacts before creating the target archive."""
        result = self.build("x86_64-unknown-linux-musl", MISSING_SBOM="rugix-ctrl")
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse((self.root / "build/binaries/x86_64-unknown-linux-musl.tar").exists())


if __name__ == "__main__":
    unittest.main()
