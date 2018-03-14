#! /usr/bin/env bash
set -e

mkdir /tmp/cargo_raze_scratch && cd /tmp/cargo_raze_scratch
git clone https://github.com/acmcarther/cargo-raze-examples
cd cargo-raze-examples
(cd ./internal && ./evaluate-local-raze.sh)
