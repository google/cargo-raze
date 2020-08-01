"""
@generated
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//remote/non_cratesio/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "unencumbered", # Unlicense from expression "Unlicense OR MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_aho_corasick_dot",
    deps = [
        # Binaries get an implicit dependency on their crate's lib
        ":aho_corasick",
        "@non_cratesio__memchr__2_2_0//:memchr",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/main.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.6.10",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)


rust_library(
    name = "aho_corasick",
    crate_type = "lib",
    deps = [
        "@non_cratesio__memchr__2_2_0//:memchr",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.6.10",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)

# Unsupported target "bench" with type "bench" omitted
# Unsupported target "dict-search" with type "example" omitted
