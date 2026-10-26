#!/usr/bin/env bash
# Exercise environment construction, caching, and artifact processing in Bakery's namespace.
# Variables in single-quoted command scripts must expand inside the working container.
# shellcheck disable=SC2016
set -euo pipefail

mode=${1:?expected native, fuse, or vfs}
pass=${2:?expected pass number}
store="${POC_STORE_BASE:-/run/rugix/bakery/cache/inner-container-poc}/${mode}"
mkdir -p "${store}"
work_dir=$(mktemp -d)
containers=()

buildah_args=(--root "${store}/storage" --runroot "/run/buildah-${mode}")
case "${mode}" in
    native) buildah_args+=(--storage-driver overlay --storage-opt overlay.mount_program=) ;;
    fuse) buildah_args+=(--storage-driver overlay --storage-opt overlay.mount_program=/usr/bin/fuse-overlayfs) ;;
    vfs) buildah_args+=(--storage-driver vfs) ;;
    *) echo "unknown storage mode: ${mode}" >&2; exit 2 ;;
esac

buildah() {
    command buildah "${buildah_args[@]}" "$@"
}

cleanup() {
    for container in "${containers[@]}"; do
        buildah rm "${container}" >/dev/null || true
    done
    rm -rf "${work_dir}"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

echo "Storage mode: ${mode}; outer-container pass: ${pass}"
id
uname -r
cat /proc/self/uid_map
cat /proc/self/gid_map
findmnt -n -o TARGET,FSTYPE -T /
findmnt -n -o TARGET,FSTYPE -T "${store}"
command buildah version
if [[ "${mode}" != fuse ]]; then
    test ! -e /dev/fuse
fi
buildah info | tee "${work_dir}/info.json"
if [[ "${mode}" == native ]]; then
    # An empty mount_program can still trigger automatic FUSE detection.
    if ! jq -e '.store.GraphStatus["Native Overlay Diff"] == "true"' "${work_dir}/info.json" >/dev/null; then
        echo "native overlay unavailable at ${store}; refusing automatic FUSE selection" >&2
        exit 1
    fi
fi

cp -a /opt/inner-container/environment "${work_dir}/context"

build_environment() {
    buildah build --isolation chroot --userns host --network host --layers --force-rm \
        --iidfile "${work_dir}/image.id" "${work_dir}/context"
}

# Identical builds must reuse the environment, including across outer-container restarts.
build_environment
image=$(cat "${work_dir}/image.id")
if [[ "${pass}" == 1 ]]; then
    printf '%s\n' "${image}" > "${store}/first-image.id"
else
    test "${image}" = "$(cat "${store}/first-image.id")"
    echo "PASS environment cache survived outer-container restart"
fi
build_environment
test "${image}" = "$(cat "${work_dir}/image.id")"
echo "PASS identical build reused image ${image}"

container=$(buildah from --isolation chroot --userns host "${image}")
containers+=("${container}")
mount_path=$(buildah mount "${container}")
mount_type=$(findmnt -n -o FSTYPE -T "${mount_path}")
echo "Working root filesystem: ${mount_type}"
case "${mode}" in
    native) test "${mount_type}" = overlay ;;
    fuse) test "${mount_type}" = fuse.fuse-overlayfs ;;
esac
buildah unmount "${container}"

mkdir -p "${work_dir}/input" "${work_dir}/output" "${store}/tool-cache"
if [[ "${pass}" == 2 ]]; then
    test "$(cat "${store}/tool-cache/marker")" = "persistent cache"
    echo "PASS explicit cache survived outer-container restart"
fi
printf 'example unsigned boot artifact\n' > "${work_dir}/input/artifact"
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 \
    -out "${work_dir}/signing-key.pem" 2>/dev/null
openssl pkey -in "${work_dir}/signing-key.pem" -pubout -out "${work_dir}/public-key.pem"

# Sign with an injected temporary key and verify input protection and image defaults.
buildah run --isolation chroot \
    --volume "${work_dir}/input:/input:ro" \
    --volume "${work_dir}/output:/output:rw" \
    --volume "${store}/tool-cache:/tool-cache:rw" \
    --volume "${work_dir}/signing-key.pem:/run/secrets/signing-key.pem:ro" \
    "${container}" -- sh -ec '
        test "$PWD" = /work
        test "$POC_TOOLCHAIN" = alpine-openssl
        if (printf modified > /input/artifact) 2>/dev/null; then
            echo "input mount was writable" >&2
            exit 1
        fi
        openssl dgst -sha256 -sign /run/secrets/signing-key.pem \
            -out /output/artifact.sig /input/artifact
        cp /environment-build-id /output/environment-build-id
        touch /invocation-only
        printf "persistent cache\n" > /tool-cache/marker
        touch /output/owned-file
        chown 1234:2345 /output/owned-file
    '
openssl dgst -sha256 -verify "${work_dir}/public-key.pem" \
    -signature "${work_dir}/output/artifact.sig" "${work_dir}/input/artifact"
test "$(stat -c '%u:%g' "${work_dir}/output/owned-file")" = 1234:2345
test "$(cat "${work_dir}/input/artifact")" = "example unsigned boot artifact"
echo "PASS signing, read-only input, output ownership, and image defaults"

# A fresh invocation must lose private state and key access, while keeping explicit caches.
fresh=$(buildah from --isolation chroot --userns host "${image}")
containers+=("${fresh}")
buildah run --isolation chroot --volume "${store}/tool-cache:/tool-cache:rw" \
    "${fresh}" -- sh -ec '
        test ! -e /invocation-only
        test ! -e /run/secrets/signing-key.pem
        test "$(cat /tool-cache/marker)" = "persistent cache"
    '
echo "PASS fresh invocation and explicit cache"

# A bounded rendezvous requires overlapping executions with separate root filesystems.
mkdir "${work_dir}/rendezvous"
run_concurrent() {
    buildah run --isolation chroot \
        --volume "${work_dir}/rendezvous:/rendezvous:rw" "$1" -- sh -ec '
            test ! -s /run/secrets/signing-key.pem
            if mountpoint -q /run/secrets/signing-key.pem; then
                echo "signing key is still mounted" >&2
                exit 1
            fi
            test ! -e /input/artifact
            test ! -e /output/artifact.sig
            touch "/$1" "/rendezvous/$1"
            attempt=0
            until test -e "/rendezvous/$2"; do
                attempt=$((attempt + 1))
                if test "$attempt" -ge 100; then
                    echo "concurrent peer did not start within 10 seconds" >&2
                    exit 1
                fi
                sleep 0.1
            done
            test ! -e "/$2"
        ' sh "$2" "$3"
}
run_concurrent "${container}" concurrent-a concurrent-b &
first_pid=$!
run_concurrent "${fresh}" concurrent-b concurrent-a &
second_pid=$!
first_status=0
second_status=0
wait "${first_pid}" || first_status=$?
wait "${second_pid}" || second_status=$?
test "${first_status}" -eq 0
test "${second_status}" -eq 0
echo "PASS concurrent invocations and temporary mount cleanup"

# A failing inner command must retain its exit status.
if buildah run --isolation chroot "${fresh}" -- sh -c 'exit 37'; then
    echo "failing command unexpectedly succeeded" >&2
    exit 1
else
    test "$?" -eq 37
fi
echo "PASS command failure propagation"

# Changing a copied context file must execute the dependent Dockerfile instruction again.
printf 'changed context\n' > "${work_dir}/context/build-input"
build_environment
changed_image=$(cat "${work_dir}/image.id")
test "${changed_image}" != "${image}"
changed=$(buildah from --isolation chroot --userns host "${changed_image}")
containers+=("${changed}")
buildah run --isolation chroot "${changed}" -- cat /environment-build-id \
    > "${work_dir}/changed-build-id"
if cmp -s "${work_dir}/output/environment-build-id" "${work_dir}/changed-build-id"; then
    echo "changed context reused the previous build token" >&2
    exit 1
fi
echo "PASS context change invalidated environment cache"

for container in "${containers[@]}"; do
    buildah rm "${container}" >/dev/null
done
containers=()
test -z "$(buildah containers --quiet)"
echo "PASS ${mode} pass ${pass}"
