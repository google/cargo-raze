"""
@generated
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

package(default_visibility = [
    # Public for visibility by "@raze__crate__version//" targets.
    #
    # Prefer access through "//remote/non_cratesio/cargo", which limits external
    # visibility to explicit Cargo.toml dependencies.
    "//visibility:public",
])

licenses([
    "notice",  # MIT from expression "MIT OR Apache-2.0"
])

# Generated targets
# Unsupported target "datetime_format" with type "bench" omitted
# Unsupported target "datetime_parse" with type "bench" omitted

# buildifier: leave-alone
rust_library(
    name = "humantime",
    crate_type = "lib",
    deps = [
        "@non_cratesio__quick_error__1_2_2//:quick_error",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.2.0",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)
