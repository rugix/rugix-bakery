#!/bin/sh
set -eu
work=/run/rugix/bakery/work
mkdir keys
openssl x509 -in "$work/inputs/certificate" -outform DER -out /tmp/boot.cer
sbsiglist --owner c1d62147-383a-4c03-833a-34e9c2db4df7 --type x509 \
    --output /tmp/boot.esl /tmp/boot.cer
for name in PK KEK db; do
    sbvarsign --attr NON_VOLATILE,RUNTIME_ACCESS,BOOTSERVICE_ACCESS,TIME_BASED_AUTHENTICATED_WRITE_ACCESS \
        --key "$work/secrets/key" --cert "$work/inputs/certificate" \
        --output "keys/$name.auth" "$name" /tmp/boot.esl
done
