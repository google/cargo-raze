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


# Unsupported target "custom_default_format" with type "example" omitted
# Unsupported target "custom_format" with type "example" omitted
# Unsupported target "custom_logger" with type "example" omitted
# Unsupported target "default" with type "example" omitted
# Unsupported target "direct_logger" with type "example" omitted

rust_library(
    name = "env_logger",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/atty-0.2.8:atty",
        "//vendored/non_cratesio_library/cargo/vendor/humantime-1.1.1:humantime",
        "//vendored/non_cratesio_library/cargo/vendor/log-0.4.1:log",
        "//vendored/non_cratesio_library/cargo/vendor/regex-0.2.9:regex",
        "//vendored/non_cratesio_library/cargo/vendor/termcolor-0.3.5:termcolor",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.5.5",
    crate_features = [
        "default",
        "regex",
    ],
)

# Unsupported target "log-in-log" with type "test" omitted
# Unsupported target "regexp_filter" with type "test" omitted
