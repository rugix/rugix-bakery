#!/bin/bash
set -euo pipefail

BSP_ARCHIVE="${RECIPE_DIR}/files/nxp-imx91-frdm.bsp.tar.gz"
BSP_DIR="$(mktemp -d)"
trap 'rm -rf "${BSP_DIR}"' EXIT

tar xzf "${BSP_ARCHIVE}" -C "${BSP_DIR}" --strip-components=1

CONFIG_DIR="${RUGIX_LAYER_DIR}/roots/config"
ARTIFACTS_DIR="${RUGIX_LAYER_DIR}/artifacts"

# --- Config partition (boot.scr, bootstrap marker) ---

mkdir -p "${CONFIG_DIR}"
install -m 0644 "${BSP_DIR}/boot/boot.scr" "${CONFIG_DIR}/boot.scr"

mkdir -p "${CONFIG_DIR}/.rugix"
touch "${CONFIG_DIR}/.rugix/bootstrap"

# --- System root (kernel, DTBs, modules, rugix-ctrl config) ---

mkdir -p "${RUGIX_ROOT_DIR}/boot"
install -m 0644 "${BSP_DIR}/boot/Image" "${RUGIX_ROOT_DIR}/boot/Image"
cp -a "${BSP_DIR}/boot/dtbs" "${RUGIX_ROOT_DIR}/boot/dtbs"

mkdir -p "${RUGIX_ROOT_DIR}/lib/modules"
cp -a "${BSP_DIR}/modules/lib/modules/"* "${RUGIX_ROOT_DIR}/lib/modules/"
for kver_dir in "${RUGIX_ROOT_DIR}/lib/modules/"*/; do
    kver="$(basename "${kver_dir}")"
    depmod -a -b "${RUGIX_ROOT_DIR}" "${kver}" || true
done

mkdir -p "${RUGIX_ROOT_DIR}/etc/rugix"
install -m 0644 "${BSP_DIR}/system.toml" "${RUGIX_ROOT_DIR}/etc/rugix/system.toml"
install -m 0644 "${BSP_DIR}/bootstrapping.toml" "${RUGIX_ROOT_DIR}/etc/rugix/bootstrapping.toml"

install -m 0644 "${RECIPE_DIR}/files/fw_env.config" "${RUGIX_ROOT_DIR}/etc/fw_env.config"

# --- Artifacts (raw firmware blobs for image assembly) ---

mkdir -p "${ARTIFACTS_DIR}"
cp -a "${BSP_DIR}/blobs/"* "${ARTIFACTS_DIR}/"
