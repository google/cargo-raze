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
  "notice", # "Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "barrier" with type "test" omitted
# Unsupported target "benches" with type "bench" omitted

rust_library(
    name = "pulse",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__atom__0_3_5//:atom",
        "@complicated_cargo_library__time__0_1_42//:time",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.3",
    crate_features = [
        "default",
    ],
)

# Unsupported target "select" with type "test" omitted
# Unsupported target "simple" with type "test" omitted
