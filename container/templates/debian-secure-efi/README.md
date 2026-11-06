# Debian Secure EFI

Build a Debian system with signed systemd-boot, signed Unified Kernel Images
(UKIs), and a SquashFS root protected by dm-verity. Firmware verifies the UKI;
its signed command line authenticates the root filesystem. Rugix Ctrl manages
A/B updates and rollback.

## Build a Development Image

Use a Rugix Bakery version that includes this template, with Docker or Podman
available. From a project directory containing the `run-bakery` launcher and
no `rugix-bakery.toml`, initialize the project:

```sh
./run-bakery init debian-secure-efi
./prepare.sh
./run-bakery bake image efi --release-version v1
```

The host needs OpenSSL and OpenSSH's `ssh-keygen` for `prepare.sh`. The script
creates a boot signing certificate and key, a bundle root CA and signer, and an
SSH key in ignored `keys/`. It refuses to replace an existing `keys/` directory.
Keep these keys to build updates for the same devices. Only the public bundle
root and SSH public key enter the root filesystem; signing keys are rule secrets.

Outputs in `build/efi/` include `system.img` (factory disk),
`system.rugixb` (signed update bundle), `uki`, `bootloader`, and `root-hash`.
Write the factory image to a dedicated target drive. Use a drive of at least
16 GiB: first boot reserves two system slots of at least 4 GiB and assigns the
remaining space to data.

## Enable Firmware Verification

Before booting with Secure Boot enforced, enroll `keys/boot.cer` in the
firmware's allowed signature database (`db`). The file is the DER version of
`keys/boot.crt`. Certificate enrollment depends on the firmware; it may require
Setup Mode or an authorized update signed by an existing key. Keep the
firmware's existing trust entries when adding this certificate.

The template signs its artifacts but does not enroll firmware keys. Confirm
that Secure Boot is enabled on the target and rejects an untrusted boot image.
For QEMU, use OVMF with Secure Boot support and persistent EFI variables
provisioned with this certificate.

## Access and Persistent State

Ethernet uses DHCP. Find the device's address through your DHCP server, then
connect using the generated SSH key, for example:

```sh
ssh -i keys/ssh root@192.0.2.10
```

Replace the example address with the device's address. Password login is
disabled. The writable root overlay is in memory and resets on reboot,
including changes to `/root`. Store application data under `/data`, which
points to Rugix Ctrl's data mount. SSH host keys persist through the core SSH
recipe.

The data partition and persistent SSH host keys are unencrypted and are outside
dm-verity verification. This template does not configure TPM-backed encryption.
Choose a data protection policy before deploying it.

## Customize the Project

`rugix-bakery.toml` selects `core/image-debian-efi-verified`. Its
`secondary-config = true` parameter matches `secondary_config = true` in
`layers/customized.toml`. The factory layout is `config` (ESP),
`config-secondary` (empty), and `system-a`. Bootstrap adds `system-b` and
`data`. The secondary config partition reserves room for a replacement ESP;
writing it and switching the partition table require a separate operation.

To target ARM64 EFI, change both the system architecture and the build rule's
`parameters.architecture` to `arm64`, and adapt the console arguments and
initramfs modules to the target hardware. The initial configuration includes
virtio modules for QEMU. An AMD64 builder needs ARM64 emulation when building
the ARM64 Debian root and initrd.

Set `system_size` on `core/debian-efi-verified-setup` to change bootstrap
slot capacity. Existing installations require a separate layout migration
before changing their partition scheme. Update bundles replace the inactive
root and UKI; they do not repartition deployed devices.

Development keys are stored without passphrases. For production, provide your
own protected signing keys and public certificates through the configured
inputs and secrets. Issue the bundle signer directly from the trusted bundle
root, with `CA:FALSE` and digital-signature usage. Preserve the boot trust and
bundle trust needed by already deployed devices when rotating keys.

See the
[EFI composition reference](https://github.com/rugix/rugix-bakery/tree/main/container/repositories/core/rules/image-efi-verified)
for sizing, signed boot arguments, and output contracts.
