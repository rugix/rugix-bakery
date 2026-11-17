#!/bin/bash

set -euo pipefail

export DEBIAN_FRONTEND=noninteractive

# Hardware autodetection cannot determine the target root device in the build chroot.
mkdir -p /etc/initramfs-tools/conf.d
printf 'MODULES=most\n' > /etc/initramfs-tools/conf.d/rugix

systemctl disable userconfig
systemctl disable resize2fs_once

# Only exists on older (Debian Bullseye) releases.
systemctl disable unattended-upgrades.service || true

# Only exists on newer (Debian Bookworm) releases.
systemctl disable sshswitch.service || true

apt-get purge -y userconf-pi
rm -f /etc/ssh/sshd_config.d/rename_user.conf

if [ "${RECIPE_PARAM_DISABLE_SWAPFILE}" = "true" ]; then
    apt-get purge -y dphys-swapfile
fi