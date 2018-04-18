"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//vendored/complicated_cargo_library/cargo:__subpackages__"])

licenses([
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


# Unsupported target "thread_local" with type "bench" omitted

rust_library(
    name = "thread_local",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/lazy_static-1.0.0:lazy_static",
        "//vendored/complicated_cargo_library/cargo/vendor/unreachable-1.0.0:unreachable",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

