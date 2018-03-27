"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//vendored/non_cratesio_library/cargo:__subpackages__"])

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


# Unsupported target "channel" with type "test" omitted

rust_library(
    name = "futures_channel",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/futures-core-0.2.0-alpha:futures_core",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "futures-core",
        "std",
    ],
)

# Unsupported target "mpsc" with type "test" omitted
# Unsupported target "mpsc-close" with type "test" omitted
# Unsupported target "oneshot" with type "test" omitted
# Unsupported target "sync_mpsc" with type "bench" omitted
