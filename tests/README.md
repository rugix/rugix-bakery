# System Tests

The system tests check that Rugix Bakery builds working systems. They bake
images and bundles with a Bakery container image and then check the results,
either by booting a system in a QEMU virtual machine (VM) or by inspecting the
partitions of an image. The tests use
[Rugix Testkit](https://github.com/rugix/rugix-testkit), which documents the
available fixtures.

The tests cover:

- Booting and updating `generic-grub-efi` systems with ext4 and SquashFS system
  partitions, including file metadata such as capabilities, ACLs, and extended
  attributes, and the generated SBOMs.
- The boot configuration of images for the Raspberry Pi tryboot and U-Boot
  targets. These systems use a minimal layer without binaries, so baking them
  works on any host without emulating the target architecture.
- `rugix-bakery test` workflows, which live in `tests/` as this directory is
  also a Bakery project.
- Independent tooling environments: repository discovery, Dockerfile caching,
  context exclusions, fresh shell containers, image defaults, and command failures.
  These tests use temporary env-only projects and do not need QEMU.
- Repository build rules: composition, owner-relative environment resolution,
  direct Bakery execution, private scratch and artifact mounts, read-only inputs,
  cached artifacts, sensitive input rotation and reuse, and failed-build recovery.
- Runtime project dependencies in rules and layer recipes: content changes with
  preserved timestamps, directory membership, absent paths, registration changes,
  contained symlink targets, read-only project mounts, and failed-build recovery.
  These tests also run in temporary projects without QEMU.
- Artifact selection and tree assembly: metadata preservation, cache invalidation,
  path validation, and generic ext4, FAT32, and SquashFS inputs.
- Rule-to-layer artifact dependencies: parent filesystem inputs, recipe input
  isolation, content-based layer caching, mixins, cycle and interface validation,
  release metadata isolation, and image assembly in the same executor.
- Recipe-owned build rules and default inputs: repository ownership, typed recipe
  parameters, layer overrides, lazy dependencies, isolated scopes, and cycle detection.
- Predefined system pipelines: layer-provided BSP layouts, firmware placement,
  filesystem and file-based bundle payloads, clean rebuilds, cache reuse, source-date
  and layout changes, failed-build publication, FAT hardlink capacity, and nested
  initialized boot roots.
- Core artifact reproducibility: independent caches and trees with different
  insertion orders, inode numbers, access times, and build times produce identical
  ext4, FAT32, and SquashFS artifacts. Bundle tests compare independently built
  outputs and extracted payloads.
- Installing released Rugix Ctrl Debian packages (extended).
- Reproducible builds of a pinned Debian snapshot (extended).

## Running the Tests

Prerequisites:

- A container runtime (Podman or Docker).
- `qemu-system-x86_64` with OVMF firmware. VMs use KVM when `/dev/kvm` is
  accessible.
- mtools for inspecting the FAT partitions of Raspberry Pi images.

The tests use the Bakery image named by `RUGIX_BAKERY_IMAGE`, which defaults
to the `dev` image that `mise run build` creates. From the repository root, run
all tests on four parallel workers:

```sh
mise run build
mise run test:system -n 4
```

Tests marked as `extended` only run when `RUGIX_TESTS_EXTENDED=1` is set. They
download from GitHub and the Debian snapshot archive, and the reproducibility
test bakes a system twice from scratch. For the focused artifact checks, run
`mise run test:system test_core_reproducibility.py test_bundle_rules.py`.
