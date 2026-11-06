# Debian with Secure Boot on EFI

This template builds Debian images for EFI devices with Secure Boot and
Rugix Ctrl updates. It provides a disk image for initial installation and
signed update bundles for later releases. The default system runs on AMD64
devices and includes Ethernet networking and SSH access.

There are two parts to setting up Secure Boot: signing the software and telling
the device which signatures to trust. Rugix Bakery builds and signs the images.
You add the signing certificate to the device's firmware before relying on it
to reject unauthorized software.

## Build Your First Image

You need Docker or Podman to run Rugix Bakery, plus OpenSSL and OpenSSH's
`ssh-keygen` to generate development keys. Use a Bakery version that includes
this template. In a new project directory containing the `run-bakery` launcher,
run:

```sh
./run-bakery init debian-secure-efi
./prepare.sh
```

The preparation script creates a `keys/` directory with signing keys,
certificates, and an SSH key for logging in to the device. Run it once for the
project. Replacing these keys can make future images and updates fail
verification on devices set up with the original keys, so the script refuses
to overwrite them.

Boot signing and update signing use separate keys. The boot key authenticates
files loaded at startup. A separate key signs update bundles, and its
certificate is issued by a root certificate that Rugix Ctrl trusts.

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
signing keys stay on the build host. The device image contains the public
certificates and SSH public key it needs to check updates and allow login.

## Boot and Connect to the Device

First, add `keys/boot.cer` to the UEFI firmware's allowed signature database,
usually called `db`. This certificate tells the firmware to trust boot files
signed with `keys/boot.key`. The enrollment procedure depends on your firmware;
it may require Setup Mode or authorization using an existing key. Add the
certificate alongside any existing trust entries you need to retain, and
check that Secure Boot is enabled.

For a QEMU virtual machine, use OVMF, its UEFI firmware, with Secure Boot
support. Enroll the same certificate in persistent EFI variables so the
firmware remembers it across VM starts.

Write `build/efi/system.img` to the device's target drive, then boot from that
drive. Use a drive of at least 16 GiB. On first boot, Rugix Ctrl finishes the
partition layout. It makes room for two copies of the system, called A and B,
with at least 4 GiB each, and uses the remaining space for persistent data.

The device obtains an Ethernet address through DHCP. Look up that address on
your DHCP server and connect with the generated SSH key. For example:

```sh
ssh -i keys/ssh root@192.0.2.10
```

Replace the example address with the device's address. Login uses the SSH key;
password login is disabled.

## How Boot Verification Works

The firmware checks the signature on systemd-boot, the bootloader used by this
template. systemd-boot then checks and starts a signed Unified Kernel Image
(UKI), a file containing the Linux kernel and the files and settings needed to
start it.

The UKI also contains a trusted hash of the root filesystem. Linux uses that
hash with dm-verity to check the filesystem's contents as they are read. This
connects the firmware's signature check to the Debian system and application
files, so modifying the root image without signing a matching UKI causes
verification to fail.

The firmware must enforce signature checks for this chain to protect the
system. As part of testing your device, check that it rejects a boot image
signed with an untrusted key.

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

For a new release, change your project and build it again with a new version:

```sh
./run-bakery bake image efi --release-version v2
```

Use the resulting `build/efi/system.rugixb` with
[Rugix Ctrl's system update workflow](https://rugix.org/docs/ctrl/updates/system-updates/).
Rugix Ctrl installs the root filesystem and UKI in the copy of the system
that is not running, allowing a trial boot before you commit the update.
Keep using signing keys
and certificates that the deployed devices trust.

## Adapt the Template

The project separates filesystem customization from image assembly.
`layers/customized.toml` selects the Debian base and recipes, while
`recipes/secure-setup/` configures networking, login, and update trust.
`rugix-bakery.toml` selects the image build rule and supplies its signing keys
and certificates.

The template also reserves a second configuration partition. The first is the
EFI System Partition (ESP), which holds systemd-boot and the UKIs. The second
has the same capacity and starts empty, leaving room to stage a replacement
ESP for a bootloader update. Writing and activating that replacement require
a separate operation. The image's `secondary-config` option and the layer's
`secondary_config` option must agree.

To build for ARM64 EFI, change both architecture values in `rugix-bakery.toml`
to `arm64`. An AMD64 build host then needs ARM64 emulation. Adapt the console
settings and the startup modules in `recipes/secure-setup/steps/00-install.sh`
for your hardware; the supplied modules support virtio devices in QEMU.

The initial size of each system is controlled by `system_size` on
`core/debian-efi-verified-setup` in `layers/customized.toml`.
Future updates must fit the partitions created during installation; changing
a deployed device's partition layout requires a separate migration.

## Use Your Own Signing Keys

The generated keys have no passphrases and are intended for development.
To use your own keys, replace the signing inputs and secrets in
`rugix-bakery.toml`, and set the trusted update root certificate in
`layers/customized.toml`. The update signer certificate must be issued directly
by the root certificate the device trusts. It needs `CA:FALSE` and
digital-signature usage: it signs updates, but cannot issue certificates.
Changing the trusted root or the firmware boot key also requires updating
trust on the devices that use them.

The [EFI build rule reference](https://github.com/rugix/rugix-bakery/tree/main/container/repositories/core/rules/image-efi-verified)
covers signing inputs, certificate requirements, and image sizes.
