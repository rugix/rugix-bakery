# Debian with Secure Boot on EFI

This template builds Debian images for devices with UEFI firmware, using
Secure Boot, a verified root filesystem, and updates through Rugix Ctrl.
Rugix is an open-source toolkit for building and updating embedded Linux
systems, and Rugix Ctrl is its update tool on the device. The template
provides a disk image for initial installation and signed update bundles for
later releases. The default system runs on AMD64 devices and includes Ethernet
networking and SSH access.

Secure Boot is a UEFI firmware feature that starts only boot software signed
with a key the firmware trusts. The template signs the bootloader and the
kernel, and the signed kernel checks the root filesystem as it reads it. This
extends the firmware's trust to the root filesystem.

> [!WARNING]
> Signing images provides no boot authenticity on its own. The device's UEFI
> firmware must trust your boot signing certificate, and Secure Boot must be
> enabled so that the firmware enforces signatures. Without signature
> enforcement, a modified boot image can bypass root filesystem verification.
> [Set Up Secure Boot in the Firmware](#set-up-secure-boot-in-the-firmware)
> explains these steps.
>
> The data partition is neither encrypted nor authenticated. Configure these
> protections separately to protect stored data against disclosure and tampering.

## Build Your First Image

You need Docker or Podman to run Rugix Bakery, the build tool for Rugix
images, plus OpenSSL and OpenSSH's `ssh-keygen` to generate development keys.
Use a Rugix Bakery version that includes this template. The
[Rugix Bakery documentation](https://rugix.org/docs/bakery/) explains how to
get the `run-bakery` launcher. In a new project directory containing the
launcher, run:

```sh
./run-bakery init debian-secure-efi
./prepare.sh
```

The preparation script creates a `keys/` directory with development keys and
certificates:

| Files | Purpose |
| --- | --- |
| `boot.key`, `boot.crt`, `boot.cer` | Sign systemd-boot and the kernel. The firmware trusts `boot.cer`. |
| `bundle-root.key`, `bundle-root.crt` | Update root. Devices trust `bundle-root.crt`; `bundle-root.key` issues signer certificates. |
| `bundle.key`, `bundle.crt` | Sign update bundles. `bundle-root.key` issued `bundle.crt`. |
| `ssh`, `ssh.pub` | Log in to the device. |

Boot signing and update signing use separate keys. The firmware trusts the
boot certificate directly. Rugix Ctrl trusts the update root certificate and
accepts bundles signed by a certificate that the root issues directly. You can
therefore replace the update signing key without changing what devices trust.

Run the script once for the project. Replacing these keys can make future
images and updates fail verification on devices set up with the original keys,
so the script refuses to overwrite them.

Now build the image:

```sh
./run-bakery bake image efi --release-version v1
```

The two files you need are in `build/efi/`:

| File | Use |
| --- | --- |
| `system.img` | Install the system on a new device. |
| `system.rugixb` | Update a device that is already running the system. |

Keep `keys/` for subsequent builds. Git ignores this directory, and the private
signing keys stay on the build host. The device image contains only the public
certificates and SSH public key it needs to check updates and allow login.

## Set Up Secure Boot in the Firmware

Rugix Bakery signs the images, but you configure each device's firmware to
trust them. Add `keys/boot.cer` to the UEFI firmware's allowed signature
database, usually called `db`. This certificate tells the firmware to trust
boot files signed with `keys/boot.key`.

The enrollment procedure depends on your firmware. Some setup menus can enroll
a certificate file directly. Others accept changes only in Setup Mode, a state
in which the firmware lets you change its key databases without authorization.
Outside Setup Mode, firmware can require a signed update authorized by an
existing key.

Every certificate in `db` can authorize boot software. Keep existing entries
that your device needs, for example for option ROMs, the firmware on expansion
cards such as graphics or network cards. Each retained entry also lets the
firmware start software signed by its owner, such as Microsoft on many PCs.
After enrollment, check that Secure Boot is enabled.

For a QEMU virtual machine, use OVMF, the UEFI firmware for QEMU, in a build
with Secure Boot support. Enroll the same certificate in persistent EFI
variables, for example through OVMF's setup menu, so the firmware remembers it
across VM starts.

## Boot and Connect to the Device

Write `build/efi/system.img` to the device's target drive, then boot from that
drive. Use a drive of at least 16 GiB. On first boot, Rugix Ctrl finishes the
partition layout. It makes room for two copies of the system, called A and B,
with at least 4 GiB each, and uses the remaining space for persistent data.

The device obtains an Ethernet address through DHCP. Look up that address on
your DHCP server, for example in your router's lease list, and connect with the
generated SSH key:

```sh
ssh -i keys/ssh root@192.0.2.10
```

Replace the example address with the device's address. Login uses the SSH key;
password login is disabled.

## How Boot Verification Works

The firmware checks the signature on systemd-boot, the bootloader used by this
template. systemd-boot then starts a signed Unified Kernel Image (UKI), a
single file containing the Linux kernel, its initial RAM disk, and its command
line. The firmware checks the UKI's signature as well.

The UKI's command line contains the root hash of the root filesystem. Linux
uses it with dm-verity, a kernel feature that checks each block of the
filesystem against a hash tree as the block is read. The hash tree is stored
after the filesystem, and the root hash authenticates the whole tree. This
connects the firmware's signature check to the Debian system and application
files: modifying the root image without signing a matching UKI causes
verification to fail.

To confirm that the firmware enforces this chain, check that your device
refuses to start a boot image signed with an untrusted key.

## Keep Data Across Reboots

The system image is read-only. To let programs write files, Rugix Ctrl places a
writable layer in memory over it. Those writes disappear on reboot, including
changes under `/root`. Put files that must survive reboots and updates in
`/data`. SSH host keys are also saved persistently, so the device keeps its SSH
identity across reboots.

Persistent data is unencrypted and is outside root filesystem verification.
For example, dm-verity detects a changed application in the system image, but
does not check a configuration file that your application reads from `/data`.

## Build an Update

For a new release, change your project and build it again with a new version,
using the same `keys/` directory:

```sh
./run-bakery bake image efi --release-version v2
```

Use the resulting `build/efi/system.rugixb` with
[Rugix Ctrl's system update workflow](https://rugix.org/docs/ctrl/updates/system-updates/).
Rugix Ctrl installs the root filesystem and UKI in the copy of the system that
is not running and then boots it once as a trial. If the trial boot fails or
you do not commit the update, the device returns to the previous copy on the
next reboot.

## Adapt the Template

Rugix Bakery builds the root filesystem from a layer, which applies recipes,
sets of build steps and files, to a base system. A build rule then assembles
the root filesystem into the disk image and update bundle.
`layers/customized.toml` selects the Debian base and recipes, while
`recipes/secure-setup/` configures networking, login, and update trust.
`rugix-bakery.toml` selects the image build rule and supplies its signing keys
and certificates.

### Change the System Size

To change the minimum size of each system copy, set `system_size` in bytes in
the existing `[parameters."core/debian-efi-verified-setup"]` table in
`layers/customized.toml`. For example, for 8 GiB:

```toml
system_size = 8589934592
```

Both copies share the drive with the boot partitions and `/data`, so larger
copies need a larger drive. Rugix Ctrl creates the partitions on first boot,
and future updates must fit them. Changing the partition layout of a deployed
device requires a separate migration.

### Build for ARM64

To build for ARM64 EFI, change both architecture values in
`rugix-bakery.toml` to `arm64`. An AMD64 build host then needs ARM64 emulation,
as described in the Rugix Bakery documentation on
[CPU architectures](https://rugix.org/docs/bakery/#cpu-architectures).

Adapt the kernel console arguments in `cmdline-extra` in `rugix-bakery.toml`
and the initial RAM disk modules in `recipes/secure-setup/steps/00-install.sh`
for your hardware. The supplied modules support virtio devices in QEMU.

### Space for Bootloader Updates

The firmware loads systemd-boot and the UKIs from the EFI System Partition
(ESP), which Rugix calls the configuration partition. The template reserves a
second, empty partition with the same capacity. This leaves room to stage a
replacement ESP for a bootloader update. Writing and activating that
replacement requires a separate operation; update bundles change only the root
filesystem and UKI.

The image's `secondary-config` option in `rugix-bakery.toml` and the layer's
`secondary_config` option in `layers/customized.toml` must agree. To drop the
reserved partition, set both to `false` before you install devices. The option
changes the partition layout, so existing installations need a separate
migration.

## Use Your Own Signing Keys

The generated keys have no passphrases and are intended for development.
To use your own keys, replace the signing inputs and secrets in
`rugix-bakery.toml`, and set the trusted update root certificate in
`layers/customized.toml`. Both certificates in `rugix-bakery.toml` must be
PEM-encoded RSA certificates. The update signer certificate must be issued
directly by the root certificate the device trusts. It needs `CA:FALSE` and
digital-signature usage: it signs updates, but cannot issue certificates.

Deployed devices keep trusting the original keys. To change the boot key,
enroll the new certificate in each device's firmware before shipping images
signed with it. To change the update
root, ship an update, signed with the current keys, that installs the new root
certificate.

The [EFI build rule reference](https://github.com/rugix/rugix-bakery/tree/main/container/repositories/core/rules/image-efi-verified)
covers signing inputs, certificate requirements, and image sizes.
