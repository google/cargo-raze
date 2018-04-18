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



rust_library(
    name = "futures_executor",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/futures-channel-0.2.0-alpha:futures_channel",
        "//vendored/non_cratesio_library/cargo/vendor/futures-core-0.2.0-alpha:futures_core",
        "//vendored/non_cratesio_library/cargo/vendor/futures-util-0.2.0-alpha:futures_util",
        "//vendored/non_cratesio_library/cargo/vendor/num_cpus-1.8.0:num_cpus",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "futures-channel",
        "futures-core",
        "futures-util",
        "num_cpus",
        "std",
    ],
)

# Unsupported target "local_pool" with type "test" omitted
# Unsupported target "poll" with type "bench" omitted
# Unsupported target "thread_notify" with type "bench" omitted
