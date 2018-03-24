"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "bench" with type "bench" omitted
# Unsupported target "count_ones" with type "bench" omitted
# Unsupported target "find" with type "bench" omitted

rust_library(
    name = "odds",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "std",
    ],
)

# Unsupported target "slice" with type "test" omitted
# Unsupported target "stride" with type "test" omitted
# Unsupported target "tests" with type "test" omitted
