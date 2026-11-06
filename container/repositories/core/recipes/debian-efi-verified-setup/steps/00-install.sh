#!/bin/sh
set -eu

case "$RUGIX_ARCH" in
    amd64)
        kernel=linux-image-amd64
        ;;
    arm64)
        kernel=linux-image-arm64
        ;;
    *)
        echo "EFI requires amd64 or arm64" >&2
        exit 1
        ;;
esac

apt-get install -y --no-install-recommends \
    "$kernel" systemd-boot cryptsetup-bin kmod

case "$RECIPE_PARAM_SECONDARY_CONFIG" in
    true)
        system_a=3
        system_b=4
        data=5
        secondary='[[layout.partitions]]\nname = "config-secondary"\nsize = 0\n'
        ;;
    false)
        system_a=2
        system_b=3
        data=4
        secondary=''
        ;;
esac

mkdir -p /etc/rugix
sed -e "s/@SYSTEM_A_PARTITION@/$system_a/g" \
    -e "s/@SYSTEM_B_PARTITION@/$system_b/g" \
    -e "s/@DATA_PARTITION@/$data/g" \
    "$RECIPE_INPUTS_DIR/system-config" > /etc/rugix/system.toml

case "$RECIPE_PARAM_SYSTEM_SIZE" in
    "" | *[!0-9]*)
        echo "system_size must be a nonnegative byte count" >&2
        exit 1
        ;;
esac
sed -e "s/@SYSTEM_SIZE@/$RECIPE_PARAM_SYSTEM_SIZE/g" -e "s/@SECONDARY_CONFIG@/$secondary/g" \
    "$RECIPE_INPUTS_DIR/bootstrap-config" > /etc/rugix/bootstrapping.toml
