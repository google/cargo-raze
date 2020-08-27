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
    # Prefer access through "//vendored/non_cratesio_library/cargo", which limits external
    # visibility to explicit Cargo.toml dependencies.
    "//visibility:public",
])

licenses([
    "notice",  # Apache-2.0 from expression "Apache-2.0 OR MIT"
])

# Generated targets
# Unsupported target "thread_local" with type "bench" omitted

# buildifier: leave-alone
rust_library(
    name = "thread_local",
    crate_type = "lib",
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/lazy_static-1.0.0:lazy_static",
        "//vendored/non_cratesio_library/cargo/vendor/unreachable-1.0.0:unreachable",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.5",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)
