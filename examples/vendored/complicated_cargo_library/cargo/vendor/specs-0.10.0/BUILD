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
# Unsupported target "async" with type "example" omitted
# Unsupported target "basic" with type "example" omitted
# Unsupported target "bitset" with type "example" omitted
# Unsupported target "cluster_bomb" with type "example" omitted
# Unsupported target "common" with type "example" omitted
# Unsupported target "full" with type "example" omitted
# Unsupported target "parallel" with type "bench" omitted
# Unsupported target "serialize" with type "example" omitted

# buildifier: leave-alone
rust_library(
    name = "specs",
    crate_type = "lib",
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/crossbeam-0.3.2:crossbeam",
        "//vendored/complicated_cargo_library/cargo/vendor/fnv-1.0.6:fnv",
        "//vendored/complicated_cargo_library/cargo/vendor/hibitset-0.3.2:hibitset",
        "//vendored/complicated_cargo_library/cargo/vendor/mopa-0.2.2:mopa",
        "//vendored/complicated_cargo_library/cargo/vendor/rayon-0.8.2:rayon",
        "//vendored/complicated_cargo_library/cargo/vendor/shred-0.5.2:shred",
        "//vendored/complicated_cargo_library/cargo/vendor/tuple_utils-0.2.0:tuple_utils",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    proc_macro_deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/derivative-1.0.2:derivative",
        "//vendored/complicated_cargo_library/cargo/vendor/shred-derive-0.3.0:shred_derive",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.10.0",
    tags = ["cargo-raze"],
    crate_features = [
    ],
)
# Unsupported target "storage" with type "bench" omitted
# Unsupported target "tests" with type "test" omitted
# Unsupported target "world" with type "bench" omitted
