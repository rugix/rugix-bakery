#!/bin/bash
set -euo pipefail
mkdir -p "${RUGIX_ROOT_DIR}/etc" "${RUGIX_LAYER_DIR}/roots/boot"
echo "console=serial0,115200" >"${RUGIX_LAYER_DIR}/roots/boot/cmdline.txt"
