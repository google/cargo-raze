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
    # Prefer access through "//remote/non_cratesio/cargo", which limits external
    # visibility to explicit Cargo.toml dependencies.
    "//visibility:public",
])

licenses([
    "notice",  # BSD-3-Clause from expression "BSD-3-Clause"
])

# Generated targets

# buildifier: leave-alone
rust_library(
    name = "fuchsia_zircon",
    crate_type = "lib",
    deps = [
        "@non_cratesio__bitflags__1_0_4//:bitflags",
        "@non_cratesio__fuchsia_zircon_sys__0_3_3//:fuchsia_zircon_sys",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.3",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)
