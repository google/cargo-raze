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
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "thread_local" with type "bench" omitted

rust_library(
    name = "thread_local",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/lazy_static-1.0.0:lazy_static",
        "//vendored/non_cratesio_library/cargo/vendor/unreachable-1.0.0:unreachable",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.5",
    crate_features = [
    ],
)

