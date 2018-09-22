"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//vendored/non_cratesio_library/cargo", which limits external
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


# Unsupported target "bilock" with type "bench" omitted

rust_library(
    name = "futures_util",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/either-1.4.0:either",
        "//vendored/non_cratesio_library/cargo/vendor/futures-channel-0.2.0-alpha:futures_channel",
        "//vendored/non_cratesio_library/cargo/vendor/futures-core-0.2.0-alpha:futures_core",
        "//vendored/non_cratesio_library/cargo/vendor/futures-io-0.2.0-alpha:futures_io",
        "//vendored/non_cratesio_library/cargo/vendor/futures-sink-0.2.0-alpha:futures_sink",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.2.0-alpha",
    crate_features = [
        "either",
        "futures-core",
        "futures-io",
        "futures-sink",
        "std",
    ],
)

# Unsupported target "futures_unordered" with type "bench" omitted
