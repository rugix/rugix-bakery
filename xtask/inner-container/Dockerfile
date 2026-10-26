ARG BAKERY_IMAGE=ghcr.io/rugix/rugix-bakery:dev
FROM ${BAKERY_IMAGE}

RUN apt-get update \
    && apt-get install -y --no-install-recommends buildah fuse-overlayfs openssl util-linux \
    && rm -rf /var/lib/apt/lists/*

COPY xtask/inner-container/check.sh /opt/inner-container/check.sh
COPY xtask/inner-container/environment /opt/inner-container/environment

ENTRYPOINT ["rugix-isolate", "bash", "/opt/inner-container/check.sh"]
