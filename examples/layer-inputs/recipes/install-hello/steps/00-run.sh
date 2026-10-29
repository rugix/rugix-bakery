#!/bin/sh
set -eu
install -D -m 0755 "$RECIPE_INPUTS_DIR/hello" "$RUGIX_ROOT_DIR/usr/bin/hello"
