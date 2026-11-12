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

The firmware must trust `boot-certificate` and enforce Secure Boot. Configure
the firmware separately, or let the factory image enroll your keys as described
in [Enroll Secure Boot Keys](#enroll-secure-boot-keys).

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

## Enroll Secure Boot Keys

UEFI firmware without a platform key (PK) runs in Setup Mode. It then starts
unsigned boot software and accepts changes to its key databases without
authorization. A factory image can use this state to enroll your keys on its
first boot. Supply the keys as the `secure-boot-keys` directory:

```toml
inputs.secure-boot-keys = { path = "keys/secure-boot" }
```

The directory must contain exactly `PK.auth`, `KEK.auth`, and `db.auth`, signed
updates of the UEFI variables holding the platform key, the key exchange keys
(KEK), and the allowed signature database (db). The db must include
`boot-certificate`, otherwise the firmware rejects the release's boot software
once it enforces Secure Boot. The files hold only certificates and signatures,
so the PK and KEK private keys stay outside the build.

The composition places the files in the ESP at `loader/keys/auto/` and enables
automatic enrollment in systemd-boot's loader configuration. When the firmware
is in Setup Mode, systemd-boot writes db, KEK, and PK, then restarts the device.
Writing PK ends Setup Mode, so the firmware verifies signatures from the next
boot on. The operating system therefore starts only after enrollment. TPM
policies bound to PCR 7, the register in which the firmware records its Secure
Boot state, see the final keys from the first operating system boot on.
Outside Setup Mode, systemd-boot leaves the keys untouched. Updates do not
remove them from the ESP, so a device enrolls them again whenever its firmware
returns to Setup Mode, for example after a reset of its keys.

In a virtual machine, systemd-boot enrolls the keys immediately. On other
machines, it first counts down 15 seconds; pressing a key cancels the
enrollment. Once Secure Boot is enforced, the firmware starts only software
that your db authorizes. This includes option ROMs, the firmware of expansion
cards such as graphics or network cards. Include the certificates of the option
ROMs a device needs, or the device can fail to start them.

For example, create the files with OpenSSL and sbsigntool from the
certificates and private keys `keys/PK.crt`, `keys/PK.key`, `keys/KEK.crt`,
`keys/KEK.key`, and the boot certificate `keys/boot.crt`. PK signs the updates
of PK and KEK, and KEK signs the update of db:

```sh
mkdir keys/secure-boot
owner=$(uuidgen)
attributes=NON_VOLATILE,RUNTIME_ACCESS,BOOTSERVICE_ACCESS,TIME_BASED_AUTHENTICATED_WRITE_ACCESS
for name in PK KEK boot; do
    openssl x509 -in "keys/$name.crt" -outform DER -out "keys/$name.cer"
    sbsiglist --owner "$owner" --type x509 --output "keys/$name.esl" "keys/$name.cer"
done
sbvarsign --attr "$attributes" --key keys/PK.key --cert keys/PK.crt \
    --output keys/secure-boot/PK.auth PK keys/PK.esl
sbvarsign --attr "$attributes" --key keys/PK.key --cert keys/PK.crt \
    --output keys/secure-boot/KEK.auth KEK keys/KEK.esl
sbvarsign --attr "$attributes" --key keys/KEK.key --cert keys/KEK.crt \
    --output keys/secure-boot/db.auth db keys/boot.esl
```

The owner GUID records who added each entry. To keep further certificates in
db, such as the vendor certificate of an option ROM, concatenate their lists
with `keys/boot.esl` before signing.

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
