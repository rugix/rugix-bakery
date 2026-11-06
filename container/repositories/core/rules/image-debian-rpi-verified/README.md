## Behavior

Build a Raspberry Pi 4/CM4 or Pi 5 A/B release with a verified root from a
prepared Debian ARM64 root. By default, this wrapper selects one installed
kernel whose version ends in `rpi-v8`, generates its initrd with
`core/initrd-debian`, and passes those artifacts to
[`core/image-rpi-verified`](../image-rpi-verified/README.md).

Raspberry Pi's `rpi-v8` kernel supports both Pi 4 and Pi 5 with 4 KiB pages.
The firmware tree must contain device trees for the target board.
See the [Raspberry Pi kernel documentation](https://www.raspberrypi.com/documentation/computers/linux_kernel.html).

Prepare the layer with
[`core/debian-rpi-verified-setup`](../../recipes/debian-rpi-verified-setup/README.md)
and configure trusted update signers in `/etc/rugix/ctrl.toml` through a
project recipe. On first boot, Rugix Ctrl completes the factory layout through
native bootstrap and uses its built-in `rpi-tryboot` flow. The wrapper consumes
the root and firmware tree without modifying them.

Set `kernel-version` when multiple matching kernels exist; its empty default
requires a unique matching kernel. `kernel-flavor` selects the required
installed kernel version suffix and defaults to `rpi-v8`. The selected kernel
and firmware tree must support the target board. Automatic capacity defaults,
optional size overrides, boot settings, remaining release inputs, signing
policy, and outputs match the generic Raspberry Pi composition.

## Example

For example, select the wrapper in a system definition:

```toml
[systems.rpi]
layer = "rpi"
architecture = "arm64"

[systems.rpi.rules.release]
uses = "core/image-debian-rpi-verified"
parameters.cmdline-extra = "console=serial0,115200 console=tty1"
inputs.boot-public-key = { path = "keys/rpi.pub" }
inputs.bundle-certificate = { path = "keys/bundle.crt" }
secrets.boot-key = { path = "keys/rpi.key" }
secrets.bundle-key = { path = "keys/bundle.key" }

[systems.rpi.outputs]
"system.img" = { rule = { node = "release", output = "image" } }
"system.rugixb" = { rule = { node = "release", output = "bundle" } }
"system.rugixb-hash" = { rule = { node = "release", output = "bundle-hash" } }
```

Bakery supplies the built layer's `rootfs`, `boot`, and `config` trees. Run
`./run-bakery bake system rpi --release-version v1`.


The `bundle-hash` output contains the bundle identity used to verify updates.
