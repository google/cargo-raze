"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//vendored/complicated_cargo_library/cargo:__subpackages__"])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)



rust_library(
    name = "conduit_mime_types",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/rustc-serialize-0.3.24:rustc_serialize",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    data = glob(["data/**"]),
    crate_features = [
    ],
)

