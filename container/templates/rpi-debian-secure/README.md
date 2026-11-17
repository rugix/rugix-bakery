# Debian with Secure Boot on Raspberry Pi

This template builds Debian images for Raspberry Pi 4, Compute Module 4, and
Pi 5, with signed boot images, a verified root filesystem, and updates through
Rugix Ctrl. Rugix is an open-source toolkit for building and updating embedded
Linux systems, and Rugix Ctrl is its update tool on the device. The template
provides a disk image for initial installation and signed update bundles for
later releases. The system includes Ethernet networking and SSH access.

These boards support signed boot: once provisioned, the Raspberry Pi bootloader
starts only a boot image signed with your key. The template signs the boot
image, which contains the kernel, and the kernel checks the root filesystem as
it reads it. This extends the bootloader's trust to the root filesystem.

> [!WARNING]
> Signing images provides no boot authenticity on its own. The Raspberry Pi
> bootloader must be provisioned with your boot public key and must enforce
> signature checks. Without signature enforcement, a modified boot image can
> bypass root filesystem verification. This template does not
> change the device's firmware settings.
> [Set Up Signed Boot on the Device](#set-up-signed-boot-on-the-device)
> explains these steps.
>
> The data partition is neither encrypted nor authenticated. Configure
> encryption and authentication separately to protect stored data against
> disclosure and tampering.

## Build Your First Image

You need Docker or Podman to run Rugix Bakery, the build tool for Rugix
images, plus OpenSSL and OpenSSH's `ssh-keygen` to generate development keys.
Use a Rugix Bakery version that includes this template. The
[Rugix Bakery documentation](https://rugix.org/docs/bakery/) explains how to
get the `run-bakery` launcher.

The template builds an ARM64 system, so an AMD64 build host also needs ARM64
emulation, as described in the Rugix Bakery documentation on
[CPU architectures](https://rugix.org/docs/bakery/#cpu-architectures).

In a new project directory containing the launcher, run:

```sh
./run-bakery init rpi-debian-secure
./prepare.sh
```

The preparation script creates a `keys/` directory with development keys and
certificates:

| Files | Purpose |
| --- | --- |
| `boot.key`, `boot.pub` | Sign boot images. The bootloader trusts `boot.pub`. |
| `bundle-root.key`, `bundle-root.crt` | Update root. Devices trust `bundle-root.crt`; `bundle-root.key` issues signer certificates. |
| `bundle.key`, `bundle.crt` | Sign update bundles. `bundle-root.key` issued `bundle.crt`. |
| `ssh`, `ssh.pub` | Log in to the device. |

Boot signing and update signing use separate keys. The bootloader trusts the
boot public key directly. Rugix Ctrl trusts the update root certificate and
accepts bundles signed by a certificate that the root issues directly. You can
therefore replace the update signing key without changing what devices trust.

Run the script once for the project. Replacing these keys can make future
images and updates fail verification on devices set up with the original keys,
so the script refuses to overwrite them.

Now build the image:

```sh
./run-bakery bake system pi --release-version v1
```

The two files you need are in `build/pi/`:

| File | Use |
| --- | --- |
| `system.img` | Install the system on a new device. |
| `system.rugixb` | Update a device that is already running the system. |

Keep `keys/` for subsequent builds. Git ignores this directory, and the
private signing keys never enter the device image. The image contains only the
public keys and certificates it needs to check updates and allow login.

## Set Up Signed Boot on the Device

Rugix Bakery signs the images, but you configure each device's bootloader to
require the signatures. The Raspberry Pi bootloader is stored in EEPROM, a
small flash memory on the board. For signed boot, this bootloader needs to
trust your boot public key and require a valid signature before starting the
system. With ordinary firmware settings, it loads the same image without
checking the signature.

Use Raspberry Pi's provisioning instructions for your board:

- [Raspberry Pi 4 and Compute Module 4](https://github.com/raspberrypi/usbboot/tree/master/secure-boot-recovery)
- [Raspberry Pi 5](https://github.com/raspberrypi/usbboot/tree/master/secure-boot-recovery5)

Pass `keys/boot.key` to the provisioning script. It signs the bootloader
configuration and embeds the matching public key in the bootloader. Like the
build, provisioning needs the private key, so keep it on hosts you control and
never copy it to a device.

> [!WARNING]
> Some provisioning steps write to the processor's one-time programmable (OTP)
> memory. These changes cannot be undone and can permanently bind the device to
> your boot key, so losing the key can prevent future updates.

The guides above explain which steps are permanent and how to test signed boot
and recovery on your board before taking them. Before deploying, confirm on
your target hardware that a modified boot image does not start.

## Boot and Connect to the Device

Write `build/pi/system.img` to the device's target storage and boot from it.
Use at least 16 GiB of storage. On first boot, Rugix Ctrl finishes the disk
layout. It makes room for two copies of the system, called A and B, with at
least 4 GiB each, and uses the remaining space for persistent data.

The device obtains an Ethernet address through DHCP. Look up that address on
your DHCP server, for example in your router's lease list, and connect with the
generated SSH key:

```sh
ssh -i keys/ssh root@192.0.2.10
```

Replace the example address with the device's address. Login uses the SSH key;
password login is disabled.

## How Boot Verification Works

The boot files are packaged in `boot.img`, a filesystem image containing the
Linux kernel, its initial RAM disk, Raspberry Pi firmware files, the kernel
command line, and boot settings. The boot partition holds `boot.img`, its
signature `boot.sig`, and a short `config.txt` that tells the bootloader to
load `boot.img`. This outer file cannot turn off signature checks. With signed
boot enabled, the bootloader checks the signature, loads `boot.img` into
memory, and takes all other settings from inside it.

The kernel command line inside the signed image contains the root hash of the
root filesystem. Linux uses it with dm-verity, a kernel feature that checks
each block of the filesystem against a hash tree as the block is read. The
hash tree is stored after the filesystem, and the root hash authenticates the
whole tree. This connects the boot signature to the Debian system and
application files: modifying the root image without signing a matching boot
image causes verification to fail.

## Keep Data Across Reboots

See [Rugix Ctrl's state management documentation](https://rugix.org/docs/ctrl/state-management/)
for persistence options; this template keeps root filesystem writes in memory.

## Build an Update

For a new release, change your project and build it again with a new version,
using the same `keys/` directory:

```sh
./run-bakery bake system pi --release-version v2
```

Use the resulting `build/pi/system.rugixb` with
[Rugix Ctrl's system update workflow](https://rugix.org/docs/ctrl/updates/system-updates/).
Rugix Ctrl installs a matching pair of boot and root images in the copy of the
system that is not running and then boots it once as a trial. If the trial
boot fails or you do not commit the update, the device returns to the previous
copy on the next reboot.

## Adapt the Template

Rugix Bakery builds the root filesystem from a layer, which applies recipes,
sets of build steps and files, to a base system. A build rule then assembles
the root filesystem into the disk image and update bundle.
`layers/customized.toml` selects the Debian base and recipes, while
`recipes/secure-setup/files/` contains the static configuration for networking,
login, update trust, and writable state.
`rugix-bakery.toml` selects the image build rule and supplies its signing keys
and certificates.

### Change Boot Settings

Because the bootloader reads its settings from inside the signed boot image,
you change them in the build configuration. Kernel arguments go in
`cmdline-extra` in `rugix-bakery.toml`, which already sets serial and screen
consoles. Boot settings are the contents of `config.txt` inside the signed
image. For example, to enable the serial port for that console, add this entry
inside the existing `[systems.pi.rules.release]` table in `rugix-bakery.toml`:

```toml
parameters.boot-config = """
[all]
arm_64bit=1
kernel=kernel8.img
initramfs initrd followkernel
enable_uart=1
"""
```

This replaces the full boot configuration, so it repeats the default kernel
and initial RAM disk settings along with the UART setting. `kernel8.img` is
the kernel selected by the build rule, by default Debian's installed `rpi-v8`
kernel, which supports both Pi 4 and Pi 5. A faulty boot configuration can
stop the new system copy from booting, so test updates with changed settings
on your target hardware, including the return to the previous copy.

### Change the System Size

To change the minimum size of each system copy, add a `system_size` value in
bytes to `layers/customized.toml`. For example, for 8 GiB:

```toml
[parameters."core/debian-rpi-verified-setup"]
system_size = 8589934592
```

Both copies share the storage with the boot partitions and the data partition,
so larger copies need larger storage. Rugix Ctrl creates the partitions on
first boot, and future updates must fit them. Changing the partition layout of
a deployed device requires a separate migration.

## Use Your Own Signing Keys

The generated keys have no passphrases and are intended for development.
To use your own keys, replace the certificate and public key inputs and the
signer key bindings in `rugix-bakery.toml`, and set the trusted update root
certificate in `layers/customized.toml`. The boot key must be a 2048-bit RSA
key with a PEM-encoded public key. The default update signer requires a
PEM-encoded RSA certificate. The update signer certificate must be issued
directly by the
root certificate the device trusts. It needs `CA:FALSE` and digital-signature
usage: it signs updates, but cannot issue certificates.

Deployed devices keep trusting the original keys. Choose the boot key before
making OTP changes, because they can bind a device to that key permanently.
To change the update root, ship an update, signed with the current keys, that
installs the new root certificate.

To keep the private keys in a hardware security module (HSM) or a key
management service (KMS), replace the default signers with your own rules, as
described in [Signing with an HSM or KMS](https://rugix.org/docs/bakery/next/advanced/hsm-signing).

The [Raspberry Pi build rule reference](https://github.com/rugix/rugix-bakery/tree/main/container/repositories/core/rules/image-rpi-verified)
covers signing inputs, signers, certificate requirements, and image sizes.
