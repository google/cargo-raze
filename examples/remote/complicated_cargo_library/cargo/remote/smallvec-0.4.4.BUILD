"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "reciprocal", # "MPL-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "bench" with type "bench" omitted

rust_library(
    name = "smallvec",
    crate_root = "lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "default",
        "std",
    ],
)

