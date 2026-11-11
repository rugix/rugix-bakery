# PKCS#11 signing tools with SoftHSM standing in for an HSM or KMS.
FROM docker.io/library/debian:bookworm
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        faketime jq libengine-pkcs11-openssl openssl osslsigncode softhsm2 \
    && rm -rf /var/lib/apt/lists/*
