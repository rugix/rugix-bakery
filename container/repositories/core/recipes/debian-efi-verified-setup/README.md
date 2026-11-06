## Behavior

Prepare a Debian root for signed EFI A/B releases using Rugix Ctrl's built-in
`systemd-boot` flow. The recipe installs an AMD64 or ARM64 kernel
and systemd-boot. Its `core/debian-verified-root` dependency installs
the verity initramfs hooks.

Rugix Ctrl boot groups pair GPT system partitions `system-a` and `system-b`
with UKI files `EFI/Linux/rugix-a.efi` and `rugix-b.efi` on the EFI System
Partition (ESP). Each system partition contains SquashFS and appended verity data.
The systemd-boot flow selects and commits these entries.

The ESP is partition `config`, Rugix Ctrl's configuration partition, mounted
at `/run/rugix/mounts/config`. Boot file slots use paths under that mount.
Native bootstrap preserves the ESP, expands A's partition capacity, creates an
empty B system partition, and creates data in the remaining drive space. Rugix
Ctrl then mounts data and manages its root overlay and persistent state.

The custom `/etc/rugix/system.toml` maps system partitions and UKI file slots
to systemd-boot entries. Slots use `immutable = true`: their installed bytes
remain unchanged, so Rugix Ctrl can retain payload hashes for slot verification.

Keep `/etc/fstab` consistent with the Rugix-managed mounts. Use
`core/image-debian-efi-verified` for the matching factory image and update
bundle. Firmware certificate enrollment remains a separate provisioning step.

Use Rugix Ctrl 1.4.0 or later. It resumes bootstrap after an interrupted
first boot. Pin an exact release with `core/rugix-ctrl`'s `version`
parameter, as shown in the example below.

## Configuration

The `system-config` and `bootstrap-config` inputs default to this recipe's
standard verified A/B configuration. Supply your own files to customize boot
slots, the data partition, or partition layout. For example:

```toml
[inputs."core/debian-efi-verified-setup"]
system-config = { path = "config/system.toml" }
bootstrap-config = { path = "config/bootstrap.toml" }
```

The supplied files must match the image layout and verified-root boot contract.
Both are installed at their usual paths under `/etc/rugix/`.
The bootstrap file can contain `@SYSTEM_SIZE@` placeholders, replaced by the
recipe's `system_size` parameter. This parameter defaults to 4 GiB per system
slot. Existing factory capacities of the ESP are preserved; data receives the
remaining drive space.

Configure update-bundle trust and other Rugix Ctrl policy separately in
`/etc/rugix/ctrl.toml`.

## Example

For example, define an EFI layer:

```toml
parent = "core/debian-trixie"
recipes = ["core/debian-efi-verified-setup"]

[parameters."core/rugix-ctrl"]
version = "v1.4.0"
```

Set the system architecture to `amd64` or `arm64`, then pass the same value
as the EFI build rule's required `architecture` parameter. The
[Debian EFI rule](../../rules/image-debian-efi-verified/README.md) shows
the matching build configuration.
