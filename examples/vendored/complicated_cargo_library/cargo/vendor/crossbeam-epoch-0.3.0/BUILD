"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//vendored/complicated_cargo_library/cargo", which limits external
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
    name = "crossbeam_epoch",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/arrayvec-0.4.7:arrayvec",
        "//vendored/complicated_cargo_library/cargo/vendor/cfg-if-0.1.2:cfg_if",
        "//vendored/complicated_cargo_library/cargo/vendor/crossbeam-utils-0.2.2:crossbeam_utils",
        "//vendored/complicated_cargo_library/cargo/vendor/lazy_static-0.2.11:lazy_static",
        "//vendored/complicated_cargo_library/cargo/vendor/memoffset-0.2.1:memoffset",
        "//vendored/complicated_cargo_library/cargo/vendor/nodrop-0.1.12:nodrop",
        "//vendored/complicated_cargo_library/cargo/vendor/scopeguard-0.3.3:scopeguard",
    ],
    rustc_flags = [
        "--cap-lints allow",
    ],
    version = "0.3.0",
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
