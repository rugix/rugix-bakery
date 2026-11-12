#!/bin/sh
set -eu

apt-get install -y --no-install-recommends systemd-resolved iproute2 curl xdelta3

install -D -m 600 "$RECIPE_INPUTS_DIR/ssh-key" /root/.ssh/authorized_keys
chmod 700 /root/.ssh
passwd -l root
install -D -m 644 "$RECIPE_DIR/files/sshd.conf" /etc/ssh/sshd_config.d/10-key-only.conf

install -D -m 644 "$RECIPE_DIR/files/ethernet.network" /etc/systemd/network/20-ethernet.network
systemctl enable systemd-networkd systemd-resolved
ln -sf /run/systemd/resolve/stub-resolv.conf /etc/resolv.conf
: > /etc/machine-id
rm -f /etc/ssh/ssh_host_*

install -D -m 644 "$RECIPE_INPUTS_DIR/bundle-root" /etc/rugix/bundle-root.crt
install -D -m 644 "$RECIPE_DIR/files/ctrl.toml" /etc/rugix/ctrl.toml
install -D -m 644 "$RECIPE_DIR/files/state.toml" /etc/rugix/state.toml

cat "$RECIPE_DIR/files/initramfs-modules" >> /etc/initramfs-tools/modules
