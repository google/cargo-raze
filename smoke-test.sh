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

# Ensure Cargo Raze build is up-to-date
echo "Building local Cargo Raze"
cd "$IMPL_DIR"
cargo build --quiet
RAZE="$IMPL_DIR/target/debug/cargo-raze raze"

# Clean the `examples` directory
echo "Cleaning examples directory"
rm -rf "$EXAMPLES_DIR/remote" "$EXAMPLES_DIR/vendored"
cp -r "$TEST_DIR/remote" "$TEST_DIR/vendored" "$TEST_DIR/tests" "$REPO_ROOT/.bazelversion" $"$EXAMPLES_DIR"

# Set up root BUILD file
touch "$EXAMPLES_DIR/BUILD"

# Set up the new WORKSPACE file
cat > "$EXAMPLES_DIR/WORKSPACE" << EOF
workspace(name = "com_github_google_cargo_raze_examples")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "5ed804fcd10a506a5b8e9e59bc6b3b7f43bc30c87ce4670e6f78df43604894fd",
    strip_prefix = "rules_rust-fdf9655ba95616e0314b4e0ebab40bb0c5fe005c",
    urls = [
        # Master branch as of 2020-07-27
        "https://github.com/bazelbuild/rules_rust/archive/fdf9655ba95616e0314b4e0ebab40bb0c5fe005c.tar.gz",
    ],
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")
EOF

for ex in $(find $TEST_DIR/remote -maxdepth 1 -type d | tail -n+2 | sort); do
    name="$(basename "$ex")"
    gen_mode="$(basename "$(dirname "$ex")")"
    cat >> "$EXAMPLES_DIR/WORKSPACE" << EOF

load("//remote/${name}/cargo:crates.bzl", "${gen_mode}_${name}_fetch_remote_crates")

${gen_mode}_${name}_fetch_remote_crates()
EOF
done

# Run Cargo Vendor over the appropriate projects
for ex in $(find $EXAMPLES_DIR/vendored -maxdepth 1 -type d | tail -n+2); do
    echo "Running Cargo Vendor for $(basename "$ex")"
    if [ "$(basename $ex)" = "regression_test" ]; then
        cd "$ex/cargo"
        cargo vendor -q --versioned-dirs
    else
        cd "$ex"
        cargo vendor -q --versioned-dirs "$ex/cargo/vendor"
    fi
done

for ex in $(find $EXAMPLES_DIR -mindepth 2 -maxdepth 2 -type d); do
    echo "Running Cargo Raze for $(basename $ex)"
    if [ "$(basename $ex)" = "regression_test" ]; then
        cd "$ex/cargo"
    else
        cd "$ex"
    fi
    eval "$RAZE"
done

# Print Bazel version info
echo "Bazel version info"
bazel version

# Run the Bazel build for all targets
cd "$EXAMPLES_DIR"
echo "Running Bazel 'build' and 'test' for all examples"
bazel build //...
bazel test //...

cd "$PWD"
