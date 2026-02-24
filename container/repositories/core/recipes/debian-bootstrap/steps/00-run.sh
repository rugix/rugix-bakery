#!/bin/bash

set -euo pipefail

case "${RUGIX_ARCH}" in
    "amd64")
        DEBIAN_ARCH="amd64"
        ;;
    "arm64")
        DEBIAN_ARCH="arm64"
        ;;
    "armv7")
        DEBIAN_ARCH="armhf"
        ;;
    "arm")
        DEBIAN_ARCH="armel"
        ;;
    *)
        echo "Unsupported architecture '${RUGIX_ARCH}'."
        exit 1
esac

OPTS=(
    "--skip=check/qemu"
    "--architectures=${DEBIAN_ARCH}"
)
TARGET_MIRROR=""

if [ -n "${RECIPE_PARAM_SNAPSHOT}" ]; then
    TARGET_MIRROR="https://snapshot.debian.org/archive/debian/${RECIPE_PARAM_SNAPSHOT}/"
    OPTS+=("--aptopt=Acquire::Check-Valid-Until=false")
    OPTS+=("--aptopt=Apt::Key::gpgvcommand=/usr/libexec/mmdebstrap/gpgvnoexpkeysig")
    OPTS+=("--include=ca-certificates,mmdebstrap")
elif [ -n "${RECIPE_PARAM_MIRROR}" ]; then
    TARGET_MIRROR="deb [trusted=yes] ${RECIPE_PARAM_MIRROR} ${RECIPE_PARAM_SUITE} main"
fi

mmdebstrap \
    "${OPTS[@]}" \
    "${RECIPE_PARAM_SUITE}" \
    "${RUGIX_ROOT_DIR}" \
    ${TARGET_MIRROR:+"$TARGET_MIRROR"}
