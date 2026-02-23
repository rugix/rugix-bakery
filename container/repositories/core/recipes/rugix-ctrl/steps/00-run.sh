#!/bin/bash

set -euo pipefail

GITHUB_REPO="rugix/rugix"

# Determine the Rust target triple based on architecture and libc.
if [ "${RECIPE_PARAM_USE_MUSL}" = "true" ]; then
    LIBC="musl"
    case "${RUGIX_ARCH}" in
        "amd64")
            TARGET="x86_64-unknown-linux-musl"
            ;;
        "arm64")
            TARGET="aarch64-unknown-linux-musl"
            ;;
        "armv7")
            TARGET="armv7-unknown-linux-musleabihf"
            ;;
        "armhf")
            TARGET="arm-unknown-linux-musleabihf"
            ;;
        "arm")
            TARGET="arm-unknown-linux-musleabi"
            ;;
        *)
            echo "Unsupported architecture '${RUGIX_ARCH}' (MUSL)."
            exit 1
    esac
else
    LIBC="gnu"
    case "${RUGIX_ARCH}" in
        "amd64")
            TARGET="x86_64-unknown-linux-gnu"
            ;;
        "arm64")
            TARGET="aarch64-unknown-linux-gnu"
            ;;
        "armv7")
            TARGET="armv7-unknown-linux-gnueabihf"
            ;;
        "armhf")
            TARGET="arm-unknown-linux-gnueabihf"
            ;;
        "arm")
            TARGET="arm-unknown-linux-gnueabi"
            ;;
        *)
            echo "Unsupported architecture '${RUGIX_ARCH}' (GNU)."
            exit 1
    esac
fi

install_from_container() {
    cp "/usr/share/rugix/binaries/${TARGET}/rugix-ctrl" "${RUGIX_ROOT_DIR}/usr/bin"
}

install_from_tar() {
    local version="$1"
    local url="https://github.com/${GITHUB_REPO}/releases/download/${version}/binaries-${TARGET}.tar"
    echo "Downloading ${url}..."
    local tmpdir
    tmpdir=$(mktemp -d)
    curl -fSL -o "${tmpdir}/binaries.tar" "${url}"
    tar -xf "${tmpdir}/binaries.tar" -C "${tmpdir}"
    cp "${tmpdir}/rugix-ctrl" "${RUGIX_ROOT_DIR}/usr/bin"
    rm -rf "${tmpdir}"
}

install_from_deb() {
    local version="$1"
    # Convert version tag to deb version: strip 'v' prefix, replace first '-' with '+'.
    local deb_version
    deb_version=$(echo "${version}" | sed 's/^v//; s/-/+/')
    # Map architecture to deb architecture.
    local deb_arch
    case "${RUGIX_ARCH}" in
        "amd64") deb_arch="amd64" ;;
        "arm64") deb_arch="arm64" ;;
        "armv7") deb_arch="armhf" ;;
        "armhf") deb_arch="armhf" ;;
        *)
            echo "No .deb package available for architecture '${RUGIX_ARCH}', falling back to tar."
            install_from_tar "${version}"
            return
            ;;
    esac
    mkdir -p "${RUGIX_ROOT_DIR}/tmp"
    local base_url="https://github.com/${GITHUB_REPO}/releases/download/${version}"
    local ctrl_deb="rugix-ctrl-${LIBC}_${deb_version}_${deb_arch}.deb"
    echo "Downloading ${base_url}/${ctrl_deb}..."
    curl -fSL -o "${RUGIX_ROOT_DIR}/tmp/${ctrl_deb}" "${base_url}/${ctrl_deb}"
}

resolve_version() {
    local source="$1"
    # If source matches a major version prefix (e.g., "v1", "v2"), resolve via GitHub API.
    if echo "${source}" | grep -qE '^v[0-9]+$'; then
        echo "Resolving latest release for ${source}..." >&2
        local resolved
        resolved=$(curl -fSs "https://api.github.com/repos/${GITHUB_REPO}/releases?per_page=100" \
            | jq -r --arg prefix "${source}." \
                '[.[] | select(.tag_name | startswith($prefix))]
                 | map(.tag_name | ltrimstr("v"))
                 | sort_by([(split("-")[0] | split(".") | map(tonumber))[], (if test("-") then 0 else 1 end)])
                 | last
                 | if . then "v" + . else null end')
        if [ -z "${resolved}" ] || [ "${resolved}" = "null" ]; then
            echo "No release found matching '${source}.*'." >&2
            exit 1
        fi
        echo "Resolved to ${resolved}." >&2
        echo "${resolved}"
    else
        echo "${source}"
    fi
}

if [ "${RECIPE_PARAM_SOURCE}" = "container" ]; then
    install_from_container
else
    VERSION=$(resolve_version "${RECIPE_PARAM_SOURCE}")
    if [ -x "${RUGIX_ROOT_DIR}/usr/bin/dpkg" ]; then
        install_from_deb "${VERSION}"
    else
        install_from_tar "${VERSION}"
    fi
fi
