## Behavior

Prepare an EFI System Partition directory tree for the core A/B boot policy.
Install a signed systemd-boot executable at `EFI/BOOT/BOOTX64.EFI` for
`amd64`, or `EFI/BOOT/BOOTAA64.EFI` for `arm64`. Install the initial signed
UKI at `EFI/Linux/rugix-a.efi`.

The generated loader configuration selects A by default, uses the `timeout`
parameter as the menu timeout in seconds (zero by default), and disables the
command-line editor. Rugix Ctrl later installs B as `EFI/Linux/rugix-b.efi`.
The filesystem holding this tree needs capacity for both UKIs.

If the optional `secure-boot-keys` directory is not empty, it must contain
exactly `PK.auth`, `KEK.auth`, and `db.auth`. The rule installs them at
`loader/keys/auto/` and sets `secure-boot-enroll force`, so systemd-boot
enrolls them when the firmware is in Setup Mode.
[`core/image-efi-verified`](../image-efi-verified/README.md#enroll-secure-boot-keys)
explains the enrollment.

The tree includes `.rugix/bootstrap`, which starts Rugix Ctrl's native
first-boot expansion using the root filesystem's bootstrap plan. The ESP is
Rugix Ctrl's configuration partition, mounted at `/run/rugix/mounts/config`.

The rule packages its inputs and does not sign them. Supply matching signed
artifacts and pass the exported `tree` to `core/image-fat`.

## Example

For example, prepare an AMD64 ESP:

```toml
uses = "core/prepare-efi-esp"
inputs.uki = { path = "signed-uki.efi" }
inputs.bootloader = { path = "signed-systemd-boot.efi" }
parameters.architecture = "amd64"
```

Save the invocation as `build-rule.toml` and run
`./run-bakery bake rule build-rule.toml build/result`.
