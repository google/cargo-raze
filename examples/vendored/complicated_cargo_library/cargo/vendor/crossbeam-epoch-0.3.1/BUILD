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
    "notice",  # MIT from expression "MIT OR Apache-2.0"
])

# Generated targets

# buildifier: leave-alone
rust_library(
    name = "crossbeam_epoch",
    crate_type = "lib",
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/arrayvec-0.4.10:arrayvec",
        "//vendored/complicated_cargo_library/cargo/vendor/cfg-if-0.1.7:cfg_if",
        "//vendored/complicated_cargo_library/cargo/vendor/crossbeam-utils-0.2.2:crossbeam_utils",
        "//vendored/complicated_cargo_library/cargo/vendor/lazy_static-1.3.0:lazy_static",
        "//vendored/complicated_cargo_library/cargo/vendor/memoffset-0.2.1:memoffset",
        "//vendored/complicated_cargo_library/cargo/vendor/nodrop-0.1.13:nodrop",
        "//vendored/complicated_cargo_library/cargo/vendor/scopeguard-0.3.3:scopeguard",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.3.1",
    tags = ["cargo-raze"],
    crate_features = [
        "default",
        "lazy_static",
        "use_std",
    ],
)
# Unsupported target "defer" with type "bench" omitted
# Unsupported target "flush" with type "bench" omitted
# Unsupported target "pin" with type "bench" omitted
# Unsupported target "sanitize" with type "example" omitted
