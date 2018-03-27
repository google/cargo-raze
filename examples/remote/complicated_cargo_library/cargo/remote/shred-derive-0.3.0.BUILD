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


rust_library(
    name = "shred_derive",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__quote__0_3_15//:quote",
        "@complicated_cargo_library__syn__0_11_11//:syn",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

