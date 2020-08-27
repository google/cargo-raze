"""
@generated
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

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
# Unsupported target "benches" with type "bench" omitted

# buildifier: leave-alone
rust_library(
    name = "hibitset",
    crate_type = "lib",
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/atom-0.3.5:atom",
        "//vendored/complicated_cargo_library/cargo/vendor/rayon-0.8.2:rayon",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.2",
    tags = ["cargo-raze"],
    crate_features = [
        "default",
        "parallel",
        "rayon",
    ],
)
