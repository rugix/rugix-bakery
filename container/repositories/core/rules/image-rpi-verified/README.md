## Behavior

Build a Raspberry Pi 4/CM4 or Pi 5 A/B release with a verified root from an
existing root filesystem, ARM64 kernel, initrd, and firmware tree. The
composition builds a SquashFS root filesystem, appends dm-verity authentication
data, embeds the root hash and image geometry in boot arguments, and packages the kernel, initrd,
firmware, and arguments in a signed FAT `boot.img`.

A firmware-visible FAT boot partition contains `boot.img`, `boot.sig`, and an
outer `config.txt` with `boot_ramdisk=1`. With signed boot enabled in the
EEPROM, the bootloader verifies the RAM disk using its embedded public key and
loads boot settings from inside it. The outer file cannot disable signature
enforcement. With normal EEPROM settings, the same file selects the RAM disk
without verifying its signature; the root hash is then unauthenticated.

Signed boot requires EEPROM provisioning with the matching public key. See
Raspberry Pi's [boot security guide](https://pip-assets.raspberrypi.com/categories/1260-security/documents/RP-003466-WP/Boot-Security-Howto)
and [RAM-disk documentation](https://www.raspberrypi.com/documentation/computers/config_txt.html#boot_ramdisk).
For Pi 5, supply a Pi 5-compatible kernel and device trees.

The factory disk contains the configuration partition, boot A, an empty boot
B reservation, and system A with the combined root image. On first boot, Rugix
Ctrl's native bootstrap preserves configuration and boot capacities, expands
the system capacity, creates an empty B system partition, and allocates the
remaining drive space to data. The platform setup recipe defaults both system
slots to at least 4 GiB. [The Raspberry Pi layout rule](../layout-rpi-ab/README.md)
describes both layouts.

The initrd must implement the
[core verified-root boot contract](../../recipes/debian-verified-root/README.md).
The root must contain Rugix Ctrl and a matching bootstrap plan.
The `boot` input must contain `start4.elf`, `fixup4.dat`, device trees, and
an `overlays` directory. The core packaging policy uses a 64-bit kernel and
accepts caller-supplied firmware settings. Supply different packaging rules
for other board policies.

Factory sizing requires no size settings:

- Root: SquashFS sized to its contents, followed by verity data.
- Signed boot image: use compact automatic FAT sizing.
- Boot partition: reserve 256 MiB, enough for any supported boot image and its
  signature, including growth in later releases.
- Configuration partition: size automatically with a 64 MiB minimum.
- Disk: fit the factory partition layout automatically.

`root-compression` selects the SquashFS compression of the root filesystem and
defaults to `none`, which helps delta updates find unchanged data. Select a
compressor such as `zstd` or `xz` for smaller images.

`config-size`, `boot-size`, `boot-image-size`, and `disk-size` are optional
factory capacity overrides. The signed boot image is loaded into
RAM and must not exceed 180,000,000 bytes. `core/verify-rpi` enforces this
conservative limit for the firmware's 180 MB RAM disk budget. The bootstrap
plan controls deployed system capacities and data allocation. Once
bootstrapped, future payloads must fit the deployed slots. `signing-time` is
Unix seconds encoded as a string; empty uses the build source epoch.

The bundle delivers combined `system.img` and the boot partition image
`boot.img` to Rugix Ctrl slots named `system` and `boot`. The consuming root
filesystem must configure corresponding A/B boot groups.

| Output | Contents |
| --- | --- |
| `image` | Factory disk |
| `bundle` | Signed update bundle |
| `boot-image` | Signed boot image loaded into RAM by the firmware |
| `boot-signature` | Signature of the boot image |
| `root-hash` | Root hash embedded in the boot image |
| `hash-offset` | Byte offset of the appended hash area, as decimal text |
| `data-size` | SquashFS length in bytes before padding and hashes, as decimal text |

Supply one PEM-encoded certificate as `bundle-certificate`; the default
bundle signer requires RSA. The bundle embeds only this signer certificate, without intermediates. Issue it directly
from the root that Rugix Ctrl trusts. The signer must not be a CA certificate.

## Signers

Two rule parameters sign the release:

- `rpi-signer` signs the boot image. It defaults to
  [`core/sign-rpi`](../sign-rpi/README.md), which uses a private key file.
- `bundle-signer` signs the update bundle's metadata. It defaults to
  [`core/sign-cms`](../sign-cms/README.md), which uses a private key file.

Bind the private keys of the default signers through `rules`:

```toml
rules.rpi-signer.secrets.key = { path = "keys/rpi.key" }
rules.bundle-signer.secrets.key = { path = "keys/bundle.key" }
```

To keep private keys in an HSM or a KMS, bind your own rules instead. A
replacement receives the same inputs and parameters as the default, listed
under Rule Parameters, and may require additional inputs, parameters, and
secrets, which its binding supplies. Bakery verifies every signature
independently of the signer: [`core/verify-rpi`](../verify-rpi/README.md)
checks the boot signature against `boot-public-key`, and
[`core/attach-bundle-signature`](../attach-bundle-signature/README.md)
checks the bundle signature against `bundle-certificate`.

## Customize Boot Settings

Set `cmdline-extra` to supply kernel arguments such as consoles and log levels.
It defaults to an empty string. Set `panic-timeout` to control automatic reboot
after a panic; its default is 10 seconds, and zero disables that reboot.
Verified-root arguments are supplied by the composition and cannot be overridden
through `cmdline-extra`.

`boot-config` supplies the complete `config.txt` inside the signed boot image.
Its default selects 64-bit `kernel8.img` and loads `initrd`. For example:

```toml
parameters.cmdline-extra = "console=serial0,115200 console=tty1 loglevel=4"
parameters.panic-timeout = 30
parameters.boot-config = """
[all]
arm_64bit=1
kernel=kernel8.img
initramfs initrd followkernel
enable_uart=1
dtoverlay=my-board
"""
```

## Example

For example, assemble prepared Raspberry Pi artifacts:

```toml
uses = "core/image-rpi-verified"
inputs.rootfs = { path = "prepared-root" }
inputs.kernel = { path = "kernel" }
inputs.initrd = { path = "initrd" }
inputs.boot = { path = "firmware" }
inputs.config = { tree = {} }
inputs.boot-public-key = { path = "keys/rpi.pub" }
inputs.bundle-certificate = { path = "keys/bundle.crt" }
rules.rpi-signer.secrets.key = { path = "keys/rpi.key" }
rules.bundle-signer.secrets.key = { path = "keys/bundle.key" }
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
For Debian roots, [the Debian wrapper](../image-debian-rpi-verified/README.md)
also selects a kernel and generates its initrd.
