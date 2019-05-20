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


# Unsupported target "build-script-build" with type "custom-build" omitted

rust_library(
    name = "security_framework_sys",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__MacTypes_sys__2_1_0//:MacTypes_sys",
        "@complicated_cargo_library__core_foundation_sys__0_5_1//:core_foundation_sys",
        "@complicated_cargo_library__libc__0_2_53//:libc",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.2.3",
    crate_features = [
    ],
)

