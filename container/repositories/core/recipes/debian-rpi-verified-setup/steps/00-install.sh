#!/bin/sh
set -eu

install -D -m 644 "$RECIPE_INPUTS_DIR/system-config" /etc/rugix/system.toml

case "$RECIPE_PARAM_SYSTEM_SIZE" in
    "" | *[!0-9]*)
        echo "system_size must be a nonnegative byte count" >&2
        exit 1
        ;;
esac
sed "s/@SYSTEM_SIZE@/$RECIPE_PARAM_SYSTEM_SIZE/g" \
    "$RECIPE_INPUTS_DIR/bootstrap-config" > /etc/rugix/bootstrapping.toml
