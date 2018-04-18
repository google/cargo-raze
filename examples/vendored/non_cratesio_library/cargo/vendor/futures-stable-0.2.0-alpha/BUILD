"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//vendored/non_cratesio_library/cargo:__subpackages__"])

licenses([
  "restricted", # "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)



rust_library(
    name = "futures_stable",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/futures-core-0.2.0-alpha:futures_core",
        "//vendored/non_cratesio_library/cargo/vendor/futures-executor-0.2.0-alpha:futures_executor",
        "//vendored/non_cratesio_library/cargo/vendor/pin-api-0.1.3:pin_api",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "futures-core",
        "futures-executor",
        "pin-api",
        "std",
    ],
)

