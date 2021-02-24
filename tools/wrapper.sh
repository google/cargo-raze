#!/bin/bash

set -euo pipefail

exec env \
CARGO="$(pwd)/${CARGO}" \
RUSTC="$(pwd)/${RUSTC}" \
"${BUILD_WORKSPACE_DIRECTORY}/${CARGO_RAZE}" "$@"
