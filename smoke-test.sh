#!/usr/bin/env bash

set -eu

function command_exists {
    command -v "$1" >/dev/null 2>&1 || ( echo "Command \`$1\` isn't available. Please install before continuing."; exit 1 )
}

PWD="$(pwd)"

REPO_ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$REPO_ROOT"

IMPL_DIR="$REPO_ROOT/impl"
EXAMPLES_DIR="$REPO_ROOT/examples"
TEST_DIR="$REPO_ROOT/smoke_test"

command_exists "cargo"
command_exists "bazel"

# Clean the `examples` directory
rm -rf "$EXAMPLES_DIR/remote" "$EXAMPLES_DIR/vendored"
cp -r "$TEST_DIR/remote" "$TEST_DIR/vendored" "$EXAMPLES_DIR"

# Set up root BUILD file
touch "$EXAMPLES_DIR/BUILD"

# Set up the new WORKSPACE file
cat > "$EXAMPLES_DIR/WORKSPACE" << EOF
workspace(name = "examples")

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

git_repository(
    name = "io_bazel_rules_rust",
    commit = "6e5fa2c570ac2f17ac1df840d060fc8aab521a07",
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "bazel_skylib",
    sha256 = "eb5c57e4c12e68c0c20bc774bfbc60a568e800d025557bc4ea022c6479acc867",
    strip_prefix = "bazel-skylib-0.6.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/0.6.0.tar.gz",
)
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

EOF

for ex in $(find $TEST_DIR/remote -maxdepth 1 -type d | tail -n+2); do
    name="$(basename "$ex")"
    cat >> "$EXAMPLES_DIR/WORKSPACE" << EOF
load("//remote/${name}/cargo:crates.bzl", "${name}_fetch_remote_crates")
${name}_fetch_remote_crates()

EOF
done

# Run Cargo Vendor over the appropriate projects
for ex in $(find $EXAMPLES_DIR/vendored -maxdepth 1 -type d | tail -n+2); do
    echo "Running Cargo Vendor for $(basename "$ex")"
    cd "$ex/cargo"
    cargo vendor -q --versioned-dirs
done

# Ensure Cargo Raze build is up-to-date
echo "Building local Cargo Raze"
cd "$IMPL_DIR"
cargo build --quiet
RAZE="$IMPL_DIR/target/debug/cargo-raze raze"
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
    bazel build "$bazel_path"
    bazel build "$bazel_cargo_path"
done

cd "$PWD"
