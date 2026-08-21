#!/bin/bash

set -euo pipefail

# This test verifies that ext4 extraction, layer archiving, and deterministic
# ext4 construction preserve deployment-relevant filesystem metadata.

source_date_epoch=1751328000
filesystem_uuid=12341234-a4ec-4304-a70f-c549ea829da9
hash_seed=035cb65d-0a86-404a-bad7-19c88d05e400
test_dir=$(mktemp -d)
trap 'rm -rf "${test_dir}"' EXIT

fixture_dir="${test_dir}/fixture"
extracted_dir="${test_dir}/extracted"
roundtrip_dir="${test_dir}/roundtrip"
final_dir="${test_dir}/final"
mkdir -p "${fixture_dir}/sticky" "${extracted_dir}" "${roundtrip_dir}" "${final_dir}"

cp /bin/true "${fixture_dir}/cap-file"
cp /bin/true "${fixture_dir}/setuid-file"
chown 1234:2345 "${fixture_dir}/setuid-file"
chmod 4755 "${fixture_dir}/setuid-file"
chmod 1777 "${fixture_dir}/sticky"
ln "${fixture_dir}/cap-file" "${fixture_dir}/cap-hardlink"
ln -s cap-file "${fixture_dir}/cap-symlink"
touch -h -d "@${source_date_epoch}" "${fixture_dir}/cap-symlink"
mkfifo "${fixture_dir}/fifo"
truncate -s 4194304 "${fixture_dir}/sparse"
printf x | dd of="${fixture_dir}/sparse" bs=1 seek=4194303 conv=notrunc status=none
setfattr -n user.rugix -v metadata "${fixture_dir}/cap-file"
setfacl -m u:1234:r-- "${fixture_dir}/cap-file"
setfacl -m d:u:1234:r-x "${fixture_dir}/sticky"
setcap cap_net_raw=ep "${fixture_dir}/cap-file"

assert_metadata() {
    local root=$1
    local file_inode
    local hardlink_inode

    file_inode=$(stat -c %i "${root}/cap-file")
    hardlink_inode=$(stat -c %i "${root}/cap-hardlink")
    test "${file_inode}" = "${hardlink_inode}"
    test "$(stat -c %h "${root}/cap-file")" = 2
    test "$(stat -c %a "${root}/setuid-file")" = 4755
    test "$(stat -c %u:%g "${root}/setuid-file")" = 1234:2345
    test "$(stat -c %a "${root}/sticky")" = 1777
    test -L "${root}/cap-symlink"
    test "$(readlink "${root}/cap-symlink")" = cap-file
    test "$(stat -c %Y "${root}/cap-symlink")" = "${source_date_epoch}"
    test -p "${root}/fifo"
    test "$(stat -c %b "${root}/sparse")" -lt 128
    getcap "${root}/cap-file" | grep -q 'cap_net_raw=ep'
    test "$(getfattr -n user.rugix --only-values "${root}/cap-file")" = metadata
    getfacl -cp "${root}/cap-file" | grep -q '^user:1234:r--$'
    getfacl -cp "${root}/sticky" | grep -q '^default:user:1234:r-x$'
}

source_image="${test_dir}/source.img"
truncate -s 48M "${source_image}"
LC_ALL=C SOURCE_DATE_EPOCH=${source_date_epoch} mkfs.ext4 \
    -q -F -O '^has_journal' \
    -U "${filesystem_uuid}" -E "hash_seed=${hash_seed}" \
    -d "${fixture_dir}" "${source_image}"

(cd "${extracted_dir}" && debugfs -R 'rdump / .' "${source_image}")
assert_metadata "${extracted_dir}"

create_archive() {
    local archive=$1

    tar \
        --create \
        --file "${archive}" \
        --directory "${extracted_dir}" \
        --format=pax \
        --sort=name \
        --numeric-owner \
        --atime-preserve=system \
        --acls \
        --selinux \
        --xattrs \
        --xattrs-include='*' \
        --sparse \
        --sparse-version=0.0 \
        --pax-option='exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime' \
        --clamp-mtime \
        --mtime="@${source_date_epoch}" \
        .
}

archive="${test_dir}/layer-a.tar"
create_archive "${archive}"
create_archive "${test_dir}/layer-b.tar"
cmp "${archive}" "${test_dir}/layer-b.tar"
tar \
    --extract \
    --file "${archive}" \
    --directory "${roundtrip_dir}" \
    --same-owner \
    --same-permissions \
    --acls \
    --selinux \
    --xattrs \
    --xattrs-include='*'
assert_metadata "${roundtrip_dir}"

for image in "${test_dir}/final-a.img" "${test_dir}/final-b.img"; do
    truncate -s 48M "${image}"
    LC_ALL=C SOURCE_DATE_EPOCH=${source_date_epoch} mkfs.ext4 \
        -q -F -O '^has_journal' \
        -U "${filesystem_uuid}" -E "hash_seed=${hash_seed}" \
        -d "${roundtrip_dir}" "${image}"
done
cmp "${test_dir}/final-a.img" "${test_dir}/final-b.img"

(cd "${final_dir}" && debugfs -R 'rdump / .' "${test_dir}/final-a.img")
assert_metadata "${final_dir}"

# Unsupported inode semantics must stop the build instead of being dropped.
debugfs -w -R 'set_inode_field /cap-file flags 0x10' "${test_dir}/final-a.img"
mkdir "${test_dir}/unsupported"
if (cd "${test_dir}/unsupported" && debugfs -R 'rdump / .' "${test_dir}/final-a.img"); then
    echo "rdump unexpectedly accepted an immutable inode" >&2
    exit 1
fi

debugfs -w -R 'set_inode_field /cap-file projid 42' "${test_dir}/final-b.img"
mkdir "${test_dir}/unsupported-project"
if (cd "${test_dir}/unsupported-project" && debugfs -R 'rdump / .' "${test_dir}/final-b.img"); then
    echo "rdump unexpectedly accepted a project quota ID" >&2
    exit 1
fi
