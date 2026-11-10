#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

RUGIX_GIT_VERSION="$(git describe --tags --always)"
export RUGIX_GIT_VERSION

binaries=(rugix-bakery rugix-isolate rugix-bundler rugix-ctrl rugix-util)
bin_args=()
for bin in "${binaries[@]}"; do
    bin_args+=(--bin "${bin}")
done

targets=("$@")
if [ "${#targets[@]}" -eq 0 ]; then
    targets=(aarch64-unknown-linux-musl x86_64-unknown-linux-musl)
fi
for target in "${targets[@]}"; do
    echo ">> building binaries for ${target}"
    cargo build --locked --release --features xz2/static --target "${target}" "${bin_args[@]}"

    cargo cyclonedx -f json --features xz2/static --target "${target}"

    out_dir="build/binaries/${target}"
    rm -rf "${out_dir}"
    mkdir -p "${out_dir}"
    src_dir="${CARGO_TARGET_DIR:-target}/${target}/release"
    for bin in "${binaries[@]}"; do
        cp "${src_dir}/${bin}" "${out_dir}/${bin}"
        if [ -f "crates/apps/${bin}/${bin}.cdx.json" ]; then
            sbom="crates/apps/${bin}/${bin}.cdx.json"
        else
            sbom="repos/rugix/crates/apps/${bin}/${bin}.cdx.json"
        fi
        cp "${sbom}" "${out_dir}/${bin}.cdx.json"
    done
    tar -cf "build/binaries/${target}.tar" -C "${out_dir}" .
done
