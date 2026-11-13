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
# `openssl cms` records the current time; faketime makes it the signing time.
faketime "@${time:-$SOURCE_DATE_EPOCH}" openssl cms -sign \
    -binary -nodetach -nosmimecap -outform DER -md sha256 \
    -engine pkcs11 -keyform engine \
    -signer "$work/inputs/certificate" \
    -inkey "pkcs11:token=rugix;object=$label;type=private;pin-value=$pin" \
    -in "$work/inputs/content" \
    -out "$work/outputs/signature.der"
