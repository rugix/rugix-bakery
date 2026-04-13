#!/usr/bin/env bash

set -euo pipefail

pushd crates/apps/rugix-bakery
sidex generate rust src/config/generated
./generate-json-schema.sh
popd

cargo fmt