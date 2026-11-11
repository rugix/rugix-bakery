#!/bin/sh
# Stand in for a provisioned HSM by importing the test keys into a SoftHSM token.
set -eu

work=/run/rugix/bakery/work
token="$work/outputs/token"
mkdir "$token"
SOFTHSM2_CONF="$(mktemp)"
export SOFTHSM2_CONF
printf 'directories.tokendir = %s\n' "$token" >"$SOFTHSM2_CONF"
pin="$(cat "$work/secrets/pin")"
softhsm2-util --init-token --free --label rugix --pin "$pin" --so-pin "$pin"
id=1
for name in boot bundle; do
    key="$(mktemp)"
    openssl pkcs8 -topk8 -nocrypt -in "$work/secrets/$name-key" -out "$key"
    softhsm2-util --import "$key" --token rugix --label "$name" --id "0$id" --pin "$pin"
    rm "$key"
    id=$((id + 1))
done
