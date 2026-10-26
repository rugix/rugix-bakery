#!/usr/bin/env bash
# Run the storage matrix in fresh, unprivileged rootless Podman containers.
set -euo pipefail

repo_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)
run_id=$(date -u +%Y%m%dT%H%M%SZ)-$$
results_dir="${POC_RESULTS_DIR:-${repo_dir}/build/inner-container-poc/${run_id}}"
image="${POC_IMAGE:-localhost/rugix-bakery-inner-poc:dev}"
volume="${POC_VOLUME:-rugix-bakery-inner-poc-${run_id}}"
modes=("$@")
if [[ ${#modes[@]} -eq 0 ]]; then
    modes=(native fuse vfs)
fi
for mode in "${modes[@]}"; do
    case "${mode}" in
        native|fuse|vfs) ;;
        *) echo "unknown storage mode: ${mode}; use native, fuse, or vfs" >&2; exit 2 ;;
    esac
done

if [[ "$(podman info --format '{{.Host.Security.Rootless}}')" != true ]]; then
    echo "Run this PoC with rootless Podman." >&2
    exit 1
fi
mkdir -p "${results_dir}"
results_dir=$(cd -- "${results_dir}" && pwd)

report() {
    printf '%s\n' "$*" | tee -a "${results_dir}/summary.txt"
}

report "Results: ${results_dir}"
report "Image: ${image}"
report "Cache volume: ${volume}"
podman version > "${results_dir}/podman-version.txt"
podman info > "${results_dir}/podman-info.txt"

if ! podman build \
    --build-arg "BAKERY_IMAGE=${BAKERY_IMAGE:-ghcr.io/rugix/rugix-bakery:dev}" \
    --tag "${image}" --file "${repo_dir}/xtask/inner-container/Dockerfile" "${repo_dir}" \
    > "${results_dir}/build.log" 2>&1; then
    cat "${results_dir}/build.log" >&2
    exit 1
fi
podman image inspect "${image}" > "${results_dir}/image-inspect.json"
if ! podman volume exists "${volume}"; then
    podman volume create "${volume}" > /dev/null
fi

failed=0
for mode in "${modes[@]}"; do
    device_args=()
    if [[ "${mode}" == fuse ]]; then
        device_args=(--device /dev/fuse)
    fi
    for pass in 1 2; do
        log="${results_dir}/${mode}-${pass}.log"
        report "Running ${mode}, outer-container pass ${pass}..."
        if podman run --rm \
            "${device_args[@]}" \
            --volume "${volume}:/run/rugix/bakery/cache" \
            "${image}" "${mode}" "${pass}" > "${log}" 2>&1; then
            report "PASS ${mode} ${pass}; log: ${log}"
        else
            report "FAIL ${mode} ${pass}; log: ${log}"
            cat "${log}" >&2
            failed=1
            break
        fi
    done
done

report "The dedicated cache volume is retained for inspection."
report "Remove it with: podman volume rm ${volume}"
exit "${failed}"
