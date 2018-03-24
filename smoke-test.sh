#!/usr/bin/env bash

set -eu

PWD="$(pwd)"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR"

EXAMPLES_DIR="$DIR/examples"
TEST_DIR="$DIR/smoke_test"

command -v "cargo-vendor" >/dev/null 2>&1 || ( echo "Cargo Vendor doesn't exist. Please install with `cargo install cargo-vendor`."; exit 1 )
command -v "bazel" >/dev/null 2>&1 || ( echo "Bazel doesn't exist. Please install using instructions at https://bazel.build/"; exit 1 )

# Clean the `examples` directory
rm -r "$EXAMPLES_DIR/remote" "$EXAMPLES_DIR/vendored"
cp -r "$TEST_DIR/remote" "$TEST_DIR/vendored" "$EXAMPLES_DIR"

# Set up the new WORKSPACE file
cat > "$EXAMPLES_DIR/WORKSPACE" << EOF
workspace(name = "io_bazel_rules_rust")

git_repository(
    name = "io_bazel_rules_rust",
    commit = "ee61ddff0b97cfc36278902e5210d1929ba5b119",           
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

EOF

for ex in $(ls $TEST_DIR/remote); do
    cat >> "$EXAMPLES_DIR/WORKSPACE" << EOF
load("//remote/${ex}/cargo:crates.bzl", "${ex}_fetch_remote_crates")
${ex}_fetch_remote_crates()

EOF
done

# Run Cargo Vendor over the appropriate projects
for ex in $(find $EXAMPLES_DIR/vendored -maxdepth 1 -type d | tail -n+2); do
    echo "Running Cargo Vendor for $(basename "$ex")"
    cd "$ex/cargo"
    cargo vendor -xq
done

# Ensure Cargo Raze build is up-to-date
cd "$DIR"
echo "Building local Cargo Raze"
cargo build --quiet
RAZE="$DIR/target/debug/cargo-raze raze"
for ex in $(find $EXAMPLES_DIR -mindepth 2 -maxdepth 2 -type d); do
    echo "Running Cargo Raze for $(basename $ex)"
    cd "$ex/cargo"
    eval "$RAZE"
done

# Run the Bazel build for all targets
cd "$EXAMPLES_DIR"
for ex in $(find $EXAMPLES_DIR -mindepth 2 -maxdepth 2 -type d); do
    ex_name="$(basename "$ex")"
    ex_type="$(basename $(dirname "$ex"))"
    bazel_path="//$ex_type/$ex_name:all"
    bazel_cargo_path="//$ex_type/$ex_name/cargo:all"

    echo "Running Bazel build for $bazel_path, $bazel_cargo_path"
    bazel build "$bazel_path" && bazel build "$bazel_cargo_path"
done