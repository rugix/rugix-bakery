# Debian with Secure Boot on Raspberry Pi

This template builds Debian images for Raspberry Pi 4, Compute Module 4, and
Pi 5, with signed boot images and Rugix Ctrl updates. It provides a disk image
for initial installation and signed update bundles for later releases.
The system includes Ethernet networking and SSH access.

Rugix Bakery signs the images using keys you supply. To make the device reject
software you have not signed, you also need to configure its boot firmware to
trust your key. Building the image and configuring firmware trust are separate
steps, described below.

## Build Your First Image

You need Docker or Podman to run Rugix Bakery, plus OpenSSL and OpenSSH's
`ssh-keygen` to generate development keys. The template builds an ARM64 system;
an AMD64 build host also needs ARM64 emulation. Use a Bakery version that
includes this template.

In a new project directory containing the `run-bakery` launcher, run:

```sh
./run-bakery init rpi-debian-secure
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

Build the image:

```sh
./run-bakery bake image pi --release-version v1
```

The two files you need are in `build/pi/`:

| File | Use |
| --- | --- |
| `system.img` | Install the system on a new device. |
| `system.rugixb` | Update a device that is already running the system. |

Keep `keys/` for subsequent builds. Git ignores this directory, and the private
signing keys stay on the build host. The device image contains the public
certificates and SSH public key it needs to check updates and allow login.

## Set Up Signed Boot on the Device

The Raspberry Pi bootloader is stored in EEPROM, a small flash memory on the
board. For signed boot, this bootloader needs to trust your boot signing key
and require a valid signature before starting the system. With ordinary
firmware settings, it can load the same image without checking the signature.

Use Raspberry Pi's provisioning instructions for your board:

- [Raspberry Pi 4 and Compute Module 4](https://github.com/raspberrypi/usbboot/tree/master/secure-boot-recovery)
- [Raspberry Pi 5](https://github.com/raspberrypi/usbboot/tree/master/secure-boot-recovery5)

The template's boot key pair is `keys/boot.key` and `keys/boot.pub`.
Provisioning tools use these keys to prepare and sign the bootloader. Keep
the private key on the provisioning host.

Follow the development and recovery steps for your board before making
permanent changes. Programming one-time programmable (OTP) memory can lock a
device to its signing key, so losing that key can prevent future updates.
The template itself leaves EEPROM and OTP unchanged. Check signature rejection
and recovery on your target hardware before deploying it.

## Boot and Connect to the Device

Write `build/pi/system.img` to the device's target storage and boot from it.
Use at least 16 GiB of storage. On first boot, Rugix Ctrl finishes the disk
layout. It makes room for two copies of the system, called A and B, with at
least 4 GiB each, and uses the remaining space for persistent data.

The device obtains an Ethernet address through DHCP. Look up that address on
your DHCP server and connect with the generated SSH key. For example:

```sh
ssh -i keys/ssh root@192.0.2.10
```

Replace the example address with the device's address. Login uses the SSH key;
password login is disabled.

## How Boot Verification Works

The boot files are packaged in `boot.img`, a filesystem image containing the
Linux kernel, the files needed to start it, firmware files, and boot settings.
The Raspberry Pi bootloader checks its signature and loads it into memory.

Those signed boot settings contain a trusted hash of the root filesystem.
Linux uses that hash with dm-verity to check the filesystem's contents as they
are read. This connects the boot signature to the Debian system and
application files: modifying the root image without signing matching boot
settings causes verification to fail. This protection depends on firmware
that enforces signed boot.

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
./run-bakery bake image pi --release-version v2
```

Use the resulting `build/pi/system.rugixb` with
[Rugix Ctrl's system update workflow](https://rugix.org/docs/ctrl/updates/system-updates/).
Rugix Ctrl installs a matching pair of boot and root images in the copy of the
system that is not running, allowing a trial boot before you commit the update. Keep using
signing keys and certificates that the deployed devices trust.

## Adapt the Template

The project separates filesystem customization from image assembly.
`layers/customized.toml` selects the Debian base and recipes, while
`recipes/secure-setup/` configures networking, login, and update trust.
`rugix-bakery.toml` selects the image build rule and supplies its signing keys
and certificates.

For example, to enable serial console output, add this entry inside the
existing `[systems.pi.build]` table in `rugix-bakery.toml`:

```toml
parameters.boot-config = """
[all]
arm_64bit=1
kernel=kernel8.img
initramfs initrd followkernel
enable_uart=1
"""
```

This replaces the full boot configuration, so it includes the kernel and
startup file selections as well as the UART setting. The default build rule
selects the installed `rpi-v8` kernel, which supports both Pi 4 and Pi 5.
Test booting and rolling back an update on the hardware you intend to use.

The initial size of each system is controlled by `system_size` on
`core/debian-rpi-verified-setup` in `layers/customized.toml`.
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

The [Raspberry Pi build rule reference](https://github.com/rugix/rugix-bakery/tree/main/container/repositories/core/rules/image-rpi-verified)
covers signing inputs, certificate requirements, and image sizes.
