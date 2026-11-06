## Behavior

Build an EFI A/B release with a verified root from an existing root filesystem,
kernel, and initrd. The composition builds a SquashFS root
filesystem, appends dm-verity authentication data, embeds the root hash and image geometry in a Unified Kernel Image (UKI),
signs the UKI and systemd-boot, and assembles a factory GPT disk and signed
Rugix update bundle.

The required `architecture` is `amd64` or `arm64` and must match the kernel.
The root filesystem supplies `etc/os-release`. The initrd must implement the
[core verified-root boot contract](../../recipes/debian-verified-root/README.md);
custom distributions can implement the same contract independently.

The factory disk contains the EFI System Partition (ESP) with the initial
`rugix-a.efi` UKI and the A system partition with the combined root image. The
ESP also carries the bootstrap marker and becomes Rugix Ctrl's configuration
partition at `/run/rugix/mounts/config`. On first boot, Rugix Ctrl preserves
the ESP, expands the system capacity, creates an empty B system partition, and
creates data in the remaining drive space according to the root filesystem's
bootstrap plan. The platform setup recipe defaults both system slots to at
least 4 GiB. [The EFI layout rule](../layout-efi-ab/README.md) describes the
factory and completed layouts.

Factory sizing requires no size settings:

- Root: SquashFS sized to its contents, followed by verity data.
- ESP: reserve three copies of the current boot files, use at least 256 MiB,
  and round up to 64 MiB.
- Disk: fit the factory partition layout automatically.

`root-compression` selects the SquashFS compression of the root filesystem and
defaults to `none`, which helps delta updates find unchanged data. Select a
compressor such as `zstd` or `xz` for smaller images.

`config-size` and `disk-size` are optional factory capacity overrides. The bootstrap plan controls deployed system capacities and data
allocation. Once bootstrapped, future payloads must fit the deployed slots.
`signing-time` is Unix seconds encoded as a string; empty uses the build
source epoch.

The bundle delivers combined `system.img` and signed `boot.efi` payloads to
Rugix Ctrl slots named `system` and `boot`. The consuming root filesystem
must configure those slots as corresponding A/B boot groups.

| Output | Contents |
| --- | --- |
| `image` | Factory disk |
| `bundle` | Signed update bundle |
| `uki` | Signed UKI |
| `bootloader` | Signed systemd-boot executable |
| `root-hash` | Root hash embedded in the UKI |
| `hash-offset` | Byte offset of the appended hash area, as decimal text |
| `data-size` | SquashFS length in bytes before padding and hashes, as decimal text |

Boot certificate enrollment and firmware trust configuration happen separately.

Supply one PEM-encoded RSA certificate as `bundle-certificate`. The bundle
embeds only this signer certificate, without intermediates. Issue it directly
from the root that Rugix Ctrl trusts. The signer must not be a CA certificate.

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

For example, assemble prepared artifacts:

```toml
uses = "core/image-efi-verified"
inputs.rootfs = { path = "prepared-root" }
inputs.kernel = { path = "kernel" }
inputs.initrd = { path = "initrd" }
inputs.boot-certificate = { path = "keys/boot.crt" }
inputs.bundle-certificate = { path = "keys/bundle.crt" }
secrets.boot-key = { path = "keys/boot.key" }
secrets.bundle-key = { path = "keys/bundle.key" }
parameters.architecture = "amd64"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
For Debian roots, [the Debian wrapper](../image-debian-efi-verified/README.md)
also selects a kernel and generates its initrd.
