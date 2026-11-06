## Behavior

Build an EFI A/B release with a verified root from a prepared Debian root
filesystem. This wrapper selects one installed kernel with
`core/kernel-debian`, generates its initrd with `core/initrd-debian`, and
passes both to [`core/image-efi-verified`](../image-efi-verified/README.md).

Prepare the root filesystem with
[`core/debian-efi-verified-setup`](../../recipes/debian-efi-verified-setup/README.md)
and configure trusted update signers in `/etc/rugix/ctrl.toml` through a
project recipe. On first boot, Rugix Ctrl completes the factory layout through
native bootstrap. The wrapper consumes the prepared root without modifying it.

The required `architecture` must match the system's `architecture` setting. Set
`kernel-version` when several installed kernels exist; its empty default
requires a unique installed kernel. Automatic capacity defaults, optional size
overrides, remaining release inputs, signing policy, and outputs match the
generic EFI composition.

## Customize Boot Settings

Set `cmdline-extra` to supply kernel arguments, such as consoles and log levels.
It defaults to an empty string. The `panic-timeout` parameter defaults to 10
seconds; zero disables automatic reboot after a panic. Verified-root arguments
remain supplied by the composition.

Set `boot-timeout` to control the systemd-boot menu timeout in seconds.
For example:

```toml
parameters.cmdline-extra = "console=ttyS0,115200n8 loglevel=4"
parameters.panic-timeout = 30
parameters.boot-timeout = 5
```

## Example

For example, select the wrapper in a system definition:

```toml
[systems.efi]
layer = "efi"
architecture = "amd64"

[systems.efi.build]
uses = "core/image-debian-efi-verified"
parameters.architecture = "amd64"
inputs.boot-certificate = { path = "keys/boot.crt" }
inputs.bundle-certificate = { path = "keys/bundle.crt" }
secrets.boot-key = { path = "keys/boot.key" }
secrets.bundle-key = { path = "keys/bundle.key" }
```

Bakery supplies the built layer as `rootfs`. Run
`./run-bakery bake image efi --release-version v1`.

## Secondary Configuration Partition

To reserve space for a replacement ESP, including systemd-boot, set
`parameters.secondary-config = true`. This adds an empty `config-secondary`
partition with the same capacity as `config`. Both config partitions precede
the system partitions, so `system-a`, `system-b`, and `data` use partition
numbers 3, 4, and 5 after bootstrap.

Enable the matching recipe option in the root filesystem's layer:

```toml
[parameters."core/debian-efi-verified-setup"]
secondary_config = true
```

Use this option when provisioning a matching factory layout. Existing
installations need a separate partition-layout migration before receiving the
changed root configuration.

Custom roots must provide equivalent system and bootstrap configurations.
The [EFI layout rule](../layout-efi-ab/README.md) describes the reserved
partition. The update bundle continues to update the root and UKI slots;
updating and activating a replacement ESP requires a separate operation.
