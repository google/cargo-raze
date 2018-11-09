"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//remote/complicated_cargo_library/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "syn",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__quote__0_3_15//:quote",
        "@complicated_cargo_library__unicode_xid__0_0_4//:unicode_xid",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.10.8",
    crate_features = [
        "aster",
        "default",
        "full",
        "parsing",
        "printing",
        "quote",
        "unicode-xid",
        "visit",
    ],
)

