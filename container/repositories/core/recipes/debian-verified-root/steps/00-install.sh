#!/bin/sh
set -eu

apt-get install -y --no-install-recommends initramfs-tools cryptsetup-bin busybox
install -D -m 755 "$RECIPE_DIR/files/verity-hook" /etc/initramfs-tools/hooks/rugix-verity
install -D -m 755 "$RECIPE_DIR/files/verity-root" /etc/initramfs-tools/scripts/local-top/rugix-verity
