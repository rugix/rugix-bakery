# Raspberry Pi Debian Secure Boot

Build Debian ARM64 releases for Raspberry Pi 4/CM4 and Pi 5 using signed FAT
boot images and a SquashFS root protected by dm-verity. A signed boot image
contains the kernel, initrd, firmware, and command line that authenticates the
root filesystem. Rugix Ctrl pairs boot and root partitions in A/B boot groups.

## Build a Development Image

Use a Rugix Bakery version containing this template, with Docker or Podman
available. An AMD64 builder also needs ARM64 emulation for Debian package
installation and initrd generation. From a project directory containing the
`run-bakery` launcher and no `rugix-bakery.toml`, run:

```sh
./run-bakery init rpi-debian-secure
./prepare.sh
./run-bakery bake image pi --release-version v1
```

The host needs OpenSSL and OpenSSH's `ssh-keygen`. The preparation script creates
an RSA-2048 boot key and public key, a bundle root CA and signer, and an SSH key
in ignored `keys/`. It refuses to replace an existing `keys/` directory. Keep
these keys for subsequent releases. Only the public bundle root and SSH public
key enter the root filesystem; signing keys are build-rule secrets.

Outputs in `build/pi/` include `system.img` (factory disk), `system.rugixb`
(signed update bundle), `boot-image`, `boot-signature`, and `root-hash`.
The `boot-image` output is the inner signed RAM disk. The bundle's boot payload
is the outer FAT partition containing that RAM disk and its signature.

Write the factory image to a dedicated target drive of at least 16 GiB.
The initial layout is `config`, `boot-a`, an empty `boot-b`, and `system-a`.
First boot preserves the configuration and boot capacities, expands system A,
creates system B, and assigns the remaining space to data. Each system slot
reserves at least 4 GiB by default.

## Provision Firmware Trust

Building signed images does not enable firmware signature enforcement. With
ordinary EEPROM settings, the image can boot without authenticating its root
hash. Provision the matching boot key and enable signed boot before relying
on dm-verity as protection against unauthorized images.

Follow Raspberry Pi's current instructions for the target board:

- [Pi 4 and CM4 secure boot provisioning](https://github.com/raspberrypi/usbboot/tree/master/secure-boot-recovery)
- [Pi 5 secure boot provisioning](https://github.com/raspberrypi/usbboot/tree/master/secure-boot-recovery5)

The corresponding key pair is `keys/boot.key` and `keys/boot.pub`. EEPROM
provisioning tools may need the private key to sign the bootloader as well.
Keep it on the provisioning host. Validate signature rejection and recovery
on the target hardware before deploying it. OTP programming can permanently
bind a device to a key. The template does not update EEPROM or program OTP.

## Access and Persistent State

Ethernet uses DHCP. Find the device's address through the DHCP server, then
connect using the generated SSH key, for example:

```sh
ssh -i keys/ssh root@192.0.2.10
```

Replace the example address with the device's address. Password login is
disabled. Root writes, including changes under `/root`, use an in-memory
overlay and disappear on reboot. Store application data under `/data`, a link
to Rugix Ctrl's persistent data mount. The core SSH recipe persists host keys.
The data filesystem and persisted SSH host keys are unencrypted and outside
root verification; choose a data protection policy before deploying it.

## Customize the Project

`rugix-bakery.toml` uses `core/image-debian-rpi-verified` and the
`core/debian-rpi-verified-setup` recipe through `secure-setup`.
The wrapper selects the installed `rpi-v8` kernel and generates its initrd.
The kernel and firmware must support the target board; validate boot and
tryboot rollback on your hardware.

For example, add this entry within the existing `[systems.pi.build]` table
to enable UART logging with a complete signed boot configuration:

```toml
parameters.boot-config = """
[all]
arm_64bit=1
kernel=kernel8.img
initramfs initrd followkernel
enable_uart=1
"""
```

Keep the kernel and initrd selection when replacing `boot-config`.
Set `system_size` on `core/debian-rpi-verified-setup` in the layer to adjust
bootstrap slot capacity. Future boot and root payloads must fit the deployed
slots; changing their layout requires a separate migration.

Development keys are stored without passphrases. For production, replace the
configured inputs and secrets with your own protected keys and certificates.
Issue the bundle signer directly from the trusted bundle root with `CA:FALSE`
and digital-signature usage. Preserve the trust needed by deployed devices
when rotating boot or bundle keys.

See the
[Raspberry Pi composition reference](https://github.com/rugix/rugix-bakery/tree/main/container/repositories/core/rules/image-rpi-verified)
for boot artifact contracts and sizing.
