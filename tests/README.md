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
test bakes a system twice from scratch.
