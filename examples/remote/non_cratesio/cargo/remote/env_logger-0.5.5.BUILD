"""
@generated
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//remote/non_cratesio/cargo", which limits external
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


# Unsupported target "custom_default_format" with type "example" omitted
# Unsupported target "custom_format" with type "example" omitted
# Unsupported target "custom_logger" with type "example" omitted
# Unsupported target "default" with type "example" omitted
# Unsupported target "direct_logger" with type "example" omitted

rust_library(
    name = "env_logger",
    crate_type = "lib",
    deps = [
        "@non_cratesio__atty__0_2_11//:atty",
        "@non_cratesio__humantime__1_2_0//:humantime",
        "@non_cratesio__log__0_4_6//:log",
        "@non_cratesio__regex__0_2_11//:regex",
        "@non_cratesio__termcolor__0_3_6//:termcolor",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.5",
    tags = ["cargo-raze"],
    crate_features = [
        "default",
        "regex",
    ],
)

# Unsupported target "log-in-log" with type "test" omitted
# Unsupported target "regexp_filter" with type "test" omitted
