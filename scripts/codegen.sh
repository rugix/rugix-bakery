#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

(
    cd crates/apps/rugix-bakery
    sidex generate rust src/config/generated
    ./generate-json-schema.sh
)
cargo fmt
