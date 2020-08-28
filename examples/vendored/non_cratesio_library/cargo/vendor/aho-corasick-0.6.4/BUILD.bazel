"""
@generated
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

# buildifier: disable=load
load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

package(default_visibility = [
    # Public for visibility by "@raze__crate__version//" targets.
    #
    # Prefer access through "//vendored/non_cratesio_library/cargo", which limits external
    # visibility to explicit Cargo.toml dependencies.
    "//visibility:public",
])

licenses([
    "unencumbered",  # Unlicense from expression "Unlicense OR MIT"
])

# Generated targets

# buildifier: leave-alone
rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_aho_corasick_dot",
    deps = [
        # Binaries get an implicit dependency on their crate's lib
        ":aho_corasick",
        "//vendored/non_cratesio_library/cargo/vendor/memchr-2.0.1:memchr",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/main.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.6.4",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)

# buildifier: leave-alone
rust_library(
    name = "aho_corasick",
    crate_type = "lib",
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/memchr-2.0.1:memchr",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.6.4",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)
# Unsupported target "bench" with type "bench" omitted
# Unsupported target "dict-search" with type "example" omitted
