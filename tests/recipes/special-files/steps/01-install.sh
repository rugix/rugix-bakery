#!/bin/bash

set -euo pipefail

DIR=/usr/share/rugix-test
mkdir -p "${DIR}"
install -m 4755 /usr/bin/true "${DIR}/setuid"
echo content >"${DIR}/original"
ln "${DIR}/original" "${DIR}/hardlink"
mkfifo "${DIR}/fifo"
setfattr -n user.rugix -v test "${DIR}/original"
cp /usr/bin/true "${DIR}/capability"
setcap cap_net_bind_service=ep "${DIR}/capability"
setfacl -m u:nobody:r "${DIR}/original"
