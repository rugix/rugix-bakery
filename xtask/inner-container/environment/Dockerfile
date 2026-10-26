FROM docker.io/library/alpine:3.23

RUN apk add --no-cache openssl
WORKDIR /work
ENV POC_TOOLCHAIN=alpine-openssl
COPY build-input /build-input
# A different token proves that this instruction actually ran again.
RUN openssl rand -hex 16 > /environment-build-id
