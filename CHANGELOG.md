# Changelog

## Version 0.9.4

- Update the bundled Rugix Core repository.
- Add an optional core recipe for the privileged Rugix Ctrl daemon.
- Fix handling of the `bootable` flag for image partitions.
- Resolve Rugix Ctrl major-version channels by release publication time.
- Use the published `anyver`, `byte-calc`, and `reportify` crates instead of
  vendored copies.
- Refresh dependencies to resolve advisories in `cmov`, `quinn-proto`, `russh`,
  and `tar`.

## Version 0.9.3

- Generate CycloneDX SBOMs alongside SPDX SBOMs.
- Add experimental system mixins for conditional layer fragments.
- Add an experimental generic BSP target.
- Allow custom SquashFS options.
- Make BSP metadata optional.
- Fix Debian architecture selection for `armhf`.
- Encode partition UUIDs using lowercase hexadecimal.

## Version 0.9.2

- Add recipe parameter overrides.
- Support KVM hardware acceleration when running tests.
- Include Skopeo and the latest Rugix Bundler for creating application bundles.
- Preserve the required permissions of bundled application files.

## Version 0.9.1

- Store the bundle hash in a separate file.
- Allow Debian mirror components to be configured.

## Version 0.9.0

- Split Rugix Bakery from Rugix Ctrl and begin versioning it independently.
- Make rootless Podman the default and support unprivileged builds through user
  namespaces.
- Allow Bakery to run from arbitrary working directories.
- Extract filesystems without elevated privileges.
- Add custom package mirror support and a Raspberry Pi OS Trixie layer.
- Rename CLI parameters for consistency, including `source` to `version`.
- Stop installing Rugix Admin from the `rugix-ctrl` recipe.

## Version 0.8.17

- Re-release of v0.8.16 due to immutable release preventing CI from publishing assets.

## Version 0.8.16

- Use `/usr/bin/env` instead of hard-coded paths.
- Improved progress reporting for delta updates.

**Note:** This release migrates to the Rugix GitHub organization.

## Version 0.8.15

- Parallel compression of Rugix update bundles.
- Project templates now default to Debian Trixie.

## Version 0.8.14

- Cryptographic integrity verification through embedded signatures.
- Compatibility with Mender and RAUC.
- State resets with backups of the old state.
- Data partition mount scripts.

## Version 0.8.13

- Fix build issues caused by layout changes in Raspberry Pi's firmware repository.

## Version 0.8.12

- Static delta updates using Xdelta.
- Update simulator as part of Rugix Bundler.

## Version 0.8.11

- Fix `fsck` invocation on data partition.

## Version 0.8.10

- Automated, built-in generation of SPDX SBOMs through Syft.

## Version 0.8.7

- Fix issue determining block device size on 32-bit platforms.

## Version 0.8.6

New features:

- Support GPT-based partition layouts on Raspberry Pi.

Bug fixes:

- Fix spurious boot errors after `fsck` repaired the data partition.
- Fix incompatibility issues with Raspberry Pi OS's initial ramdisk.

## Version 0.8.5

New features:

- Allow `auto_initramfs=1` on Raspberry Pi (required for SquashFS).
- Add new `update-install/progress` hook to report installation progress.

Bug fixes:

- Allow the default image path to be passed to `bake image`.
- Resolve compatibility issues when updating from older Rugix (Rugpi) versions.

## Version 0.8.4

- Fix broken reading of `system-build-info.json` on builds with a hot cache.

## Version 0.8.3

New features:

- Write release information to `/etc/rugix/system-build-info.json`.
- Support for SquashFS root filesystems (#6).

Bug fixes:

- Persist `machine-id` from state after updating rootfs.
- Check whether stdout is piped instead of stderr (#51).
- Change slot db directory to `/var/lib/rugix` (was `/var/rugix` before).
- Only copy image when output path differs from system image path (#53).

## Version 0.8.2

- Prevent error during update installation when using multiple block indices.

## Version 0.8.1

- Fix caching issue where cache is always cleared regardless of whether Docker image changed.

## Version 0.8.0

Rename to Rugix.

Rugix Ctrl:

- New format for update bundles.
- Adaptive delta updates with HTTP range queries.
- Support for any update scenario, including non-A/B updates and incremental updates.
- Support for any bootloader and boot process through custom boot flows.
- New JSON-based system information format.

Rugix Bakery:

- Ability to run VMs.
- Integrated system testing framework.

## Version 0.7.5

- Fixes off-by-one error in partition table sanity check affecting GPT layouts.

## Version 0.7.4

- Add support for verifying the hash of updates via `--check-hash`.

## Version 0.7.3

- Fixes issues with incompatible partition layouts when upgrading from v0.6 (see #29).

**Additional Notes:** Flashing a device with a v0.7.3 image and then installing an update based on an older 0.7 version will fail for the `rpi-` targets.

## Version 0.7.2

- Fixes bootstrapping of foreign architectures with `binfmt_misc`.

## Version 0.7.1

- Add `unknown` target.
- Limit size of MBR partitions (fix).

## Version 0.7.0

New features:

- Official support for Alpine Linux and Debian.
- Support for EFI systems and integration with Grub.
- Configurable image layouts.

Breaking changes to the image building pipeline:

- The `boot_flow` option has been superseded by `target`.
- The `include_firmware` option has been removed. To include a firmware update for Raspberry Pi, use the `core/rpi-include-firmware` recipe.
- The following recipes have been renamed:
  - `core/raspberrypi` => `core/rpi-raspios-setup`
  - `core/pi-cleanup` => `core/rpi-raspios-cleanup`
  - `core/apt-cleanup` => `core/pkg-cleanup` (also supports `apk` now)
  - `core/apt-update` => `core/pkg-cleanup` (also supports `apk` now)
  - `core/apt-upgrade` => `core/pkg-upgrade` (also supports `apk` now)
- The following recipes have been removed:
  - `core/disable-swap` (now part of `rpi-raspios-cleanup` via parameter)

## Version 0.6.6

- Allow for deferred reboots into the spare partition set.
- Make streaming updates the default.

## Version 0.6.5

- Allow booting from external USB devices.
- Fix issues with Docker due to the usage of `chroot`.

## Version 0.6.4

- Allow `gz` compressed tarballs as base layer.
- Check root filesystem size when building an image.
- Ignore any files in the `layers` directory not ending with `.toml`.

## Version 0.6.3

- Allow local `.tar` files to be used as a layer.
- Patch `/etc/fstab` instead of overwriting it.

## Version 0.6.2

- Create directories when baking images.
- Ignore `.DS_Store` directories/files.

## Version 0.6.1

- Transparent decompression of XZ-compressed images.
- Switch to streaming updates in Rugpi Admin.

## Version 0.6.0

- Introduction of layers.
- Introduction of repositories.
- Backwards-incompatible changes to image building pipeline:
  - Layers instead of recipes in `rugpi-bakery.toml`.
  - Removal of default recipes. Recipes must be explicitly enabled.
  - Separate `images` sections in `rugpi-bakery.toml`.

## Version 0.5.0

- Support for all models of Raspberry Pi via U-Boot.
- Support for persisting the overlay by default.
- Experimental support for streaming updates.

## Pre-Releases (0.1 to 0.4)

- Initial experimental version.
