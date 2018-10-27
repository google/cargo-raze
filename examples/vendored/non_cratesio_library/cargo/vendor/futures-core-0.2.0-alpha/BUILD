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



rust_library(
    name = "futures_core",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/either-1.4.0:either",
        "//vendored/non_cratesio_library/cargo/vendor/pin-api-0.1.3:pin_api",
    ],
    rustc_flags = [
        "--cap-lints allow",
    ],
    version = "0.2.0-alpha",
    crate_features = [
        "either",
        "pin-api",
        "std",
    ],
)

