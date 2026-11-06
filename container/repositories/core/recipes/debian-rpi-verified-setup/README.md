## Behavior

Prepare a Debian ARM64 root for Raspberry Pi 4/CM4 or Pi 5 signed boot using
Rugix Ctrl's built-in `rpi-tryboot` flow. The `core/rpi-debian-setup`
dependency supplies kernel and firmware packages. `core/debian-verified-root`
installs verity initramfs hooks.

Rugix Ctrl boot groups pair `boot-a` and `system-a` with corresponding B
partitions. Each system partition contains SquashFS and appended verity data.
The built-in flow manages trial selection and commit through `autoboot.txt`.
When `boot.img` is present, it preserves the authenticated command line inside
that signed image instead of patching it.

Partition `config` is Rugix Ctrl's configuration partition, mounted at
`/run/rugix/mounts/config`. Native bootstrap preserves the configuration and
boot capacities, expands A's partition capacity, creates an empty B system
partition, and allocates the remaining drive space to data partition 6. Rugix
Ctrl then mounts data and manages its root overlay and persistent state.

The custom `/etc/rugix/system.toml` marks signed boot slots `immutable = true`.
Their installed bytes remain unchanged, so Rugix Ctrl can retain payload hashes
for slot verification. Defining slots replaces the default slot map, so the
configuration also lists both system slots.

Keep `/etc/fstab` consistent with the Rugix-managed mounts. Use
`core/image-debian-rpi-verified` for the matching factory image and update
bundle.

Use Rugix Ctrl 1.4.0 or later. Earlier versions modify the boot partition
after installing it, and they cannot resume bootstrap after an interrupted
first boot. Pin an exact release with `core/rugix-ctrl`'s `version`
parameter, as shown in the example below.

## Configuration

The `system-config` and `bootstrap-config` inputs default to this recipe's
standard verified A/B configuration. Supply your own files to customize boot
slots, the data partition, or partition layout. For example:

```toml
[inputs."core/debian-rpi-verified-setup"]
system-config = { path = "config/system.toml" }
bootstrap-config = { path = "config/bootstrap.toml" }
```

The supplied files must match the image layout and verified-root boot contract.
Both are installed at their usual paths under `/etc/rugix/`.
The bootstrap file can contain `@SYSTEM_SIZE@` placeholders, replaced by the
recipe's `system_size` parameter. This parameter defaults to 4 GiB per system
slot. Existing factory capacities of the configuration and boot partitions are
preserved; data receives the remaining drive space.

Configure update-bundle trust and other Rugix Ctrl policy separately in
`/etc/rugix/ctrl.toml`.

## Example

For example, define a Raspberry Pi layer:

```toml
parent = "core/debian-trixie"
recipes = ["core/debian-rpi-verified-setup"]

[parameters."core/rugix-ctrl"]
version = "v1.4.0"
```

Use system architecture `arm64`. The
[Debian Raspberry Pi rule](../../rules/image-debian-rpi-verified/README.md)
shows the matching build configuration. Optional firmware packages are
configured through `core/rpi-debian-setup`.

EEPROM key provisioning and OTP programming are separate steps.
