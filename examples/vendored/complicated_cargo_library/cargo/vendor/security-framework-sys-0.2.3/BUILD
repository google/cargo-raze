"""
@generated
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
  "notice", # MIT from expression "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "build-script-build" with type "custom-build" omitted

rust_library(
    name = "security_framework_sys",
    crate_type = "lib",
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/MacTypes-sys-2.1.0:MacTypes_sys",
        "//vendored/complicated_cargo_library/cargo/vendor/core-foundation-sys-0.5.1:core_foundation_sys",
        "//vendored/complicated_cargo_library/cargo/vendor/libc-0.2.53:libc",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.2.3",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)

