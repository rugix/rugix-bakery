## Behavior

Install Debian initramfs hooks that open a dm-verity root before handing off to
Rugix Ctrl. A signed boot artifact authenticates the root hash and image geometry.
Rugix Ctrl then performs its normal first-boot bootstrap, data mounting, root
overlay setup, and persistent-state handling.

The recipe installs initramfs-tools, verity tooling, and the verification hooks.
Hardware-specific modules belong in the layer's initramfs configuration, such
as `/etc/initramfs-tools/modules`.

Rugix Ctrl system configuration, the bootstrap layout, and update-bundle trust
are supplied separately. The platform recipes `core/debian-efi-verified-setup`
and `core/debian-rpi-verified-setup` provide standard A/B system and bootstrap
configuration. Configure trusted update signers in `/etc/rugix/ctrl.toml`.

Verification covers the root filesystem only. The data partition, persistent
paths, and Rugix Ctrl's default root overlay are not authenticated. By default,
the overlay is stored on the data partition and placed over the verified root.
Choose the overlay and data partition configuration according to your threat
model.

## Boot Contract

The signed command line selects Rugix Ctrl as init with
`init=/usr/bin/rugix-ctrl` and supplies the verification metadata:

| Argument | Value |
| --- | --- |
| `rugix.verity-root-hash` | SHA-256 root hash |
| `rugix.verity-hash-offset` | Byte offset of the appended verity superblock |
| `rugix.verity-data-blocks` | Authenticated SquashFS length in 4096-byte blocks |
| `rugix.platform` | `efi` or `rpi` |

EFI chooses A or B from systemd-boot's `LoaderEntrySelected` variable, using
entries `rugix-a.efi` and `rugix-b.efi`. Raspberry Pi chooses A for firmware
boot partition 2 and B for 3. The selected system partition is named
`system-a` or `system-b`; each contains SquashFS followed by its verity data.

Exactly one GPT partition across all disks may carry the selected name. Rugix
Ctrl takes its configuration and data partitions from the disk holding the
root. A second disk with the same partition name, such as an attached copy of
the device's image, could otherwise supply them. The initramfs script waits for
device probing to settle, then stops the boot when it finds more than one match.
An attached drive with a matching partition therefore prevents booting.
Disks that appear after probing settles are not considered.

The initramfs script checks the appended superblock against the signed geometry,
then opens `/dev/mapper/root` with the selected partition as both data and hash
device. The contract uses verity format 1, SHA-256, and 4096-byte data and hash
blocks. A signed UKI or Raspberry Pi boot image must include these hooks and the
authenticated command line. `core/verified-cmdline` produces the arguments.

## Example

For example, configure this dependency in an EFI layer:

```toml
parent = "core/debian-trixie"
recipes = ["core/debian-efi-verified-setup"]

[parameters."core/rugix-ctrl"]
version = "v1.4.0"
```

Configure update trust in a project recipe. Only public trust certificates
enter the layer; bind signing private keys as build-rule secrets.
