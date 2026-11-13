#!/bin/sh
set -eu

work=/run/rugix/bakery/work
# SoftHSM writes lock files, so it needs a writable copy of the token.
tokens="$(mktemp -d)"
cp -R "$work/inputs/token/." "$tokens"
SOFTHSM2_CONF="$(mktemp)"
export SOFTHSM2_CONF
printf 'directories.tokendir = %s\n' "$tokens" >"$SOFTHSM2_CONF"

label="$(jq -r '."key-label"' "$work/parameters.json")"
pin="$(cat "$work/secrets/pin")"
time="$(jq -r '."signing-time"' "$work/parameters.json")"
osslsigncode sign \
    -pkcs11module /usr/lib/softhsm/libsofthsm2.so \
    -key "pkcs11:token=rugix;object=$label;type=private;pin-value=$pin" \
    -certs "$work/inputs/certificate" \
    -h sha256 \
    -time "${time:-$SOURCE_DATE_EPOCH}" \
    -in "$work/inputs/image" \
    -out "$work/outputs/signed.efi"
