"""
@generated
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

# buildifier: disable=load
load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)

package(default_visibility = [
    # Public for visibility by "@raze__crate__version//" targets.
    #
    # Prefer access through "//vendored/complicated_cargo_library/cargo", which limits external
    # visibility to explicit Cargo.toml dependencies.
    "//visibility:public",
])

licenses([
    "notice",  # Apache-2.0 from expression "Apache-2.0"
])

# Generated targets
# Unsupported target "barrier" with type "test" omitted
# Unsupported target "benches" with type "bench" omitted

# buildifier: leave-alone
rust_library(
    name = "pulse",
    crate_type = "lib",
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/atom-0.3.5:atom",
        "//vendored/complicated_cargo_library/cargo/vendor/time-0.1.42:time",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.3",
    tags = ["cargo-raze"],
    crate_features = [
        "default",
    ],
)
# Unsupported target "select" with type "test" omitted
# Unsupported target "simple" with type "test" omitted
