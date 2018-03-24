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
    name = "crossbeam_epoch",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__arrayvec__0_4_7//:arrayvec",
        "@complicated_cargo_library__cfg_if__0_1_2//:cfg_if",
        "@complicated_cargo_library__crossbeam_utils__0_2_2//:crossbeam_utils",
        "@complicated_cargo_library__lazy_static__0_2_11//:lazy_static",
        "@complicated_cargo_library__memoffset__0_2_1//:memoffset",
        "@complicated_cargo_library__nodrop__0_1_12//:nodrop",
        "@complicated_cargo_library__scopeguard__0_3_3//:scopeguard",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
        "crossbeam-utils",
        "default",
        "lazy_static",
        "use_std",
    ],
)

# Unsupported target "defer" with type "bench" omitted
# Unsupported target "flush" with type "bench" omitted
# Unsupported target "pin" with type "bench" omitted
# Unsupported target "sanitize" with type "example" omitted
