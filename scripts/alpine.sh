#!/bin/bash
 
set -euxo pipefail

ALPINE_VERSION="3.19"
ALPINE_ARCH="x86_64"
ROOTFS_DIR="./.rootfs/alpine"
ALPINE_URL="https://dl-cdn.alpinelinux.org/alpine/v${ALPINE_VERSION}/releases/${ALPINE_ARCH}/alpine-minirootfs-${ALPINE_VERSION}.0-${ALPINE_ARCH}.tar.gz"

if [ -d "${ROOTFS_DIR}" ]; then
    rm -rf "${ROOTFS_DIR}"
fi

mkdir -p "${ROOTFS_DIR}"

TMP_TAR="/tmp/alpine-minirootfs.tar.gz"
curl -L -o "${TMP_TAR}" "${ALPINE_URL}"

tar -xzf "${TMP_TAR}" -C "${ROOTFS_DIR}"

rm -f "${TMP_TAR}"
