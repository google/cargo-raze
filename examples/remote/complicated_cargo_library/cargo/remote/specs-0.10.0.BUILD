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


# Unsupported target "async" with type "example" omitted
# Unsupported target "basic" with type "example" omitted
# Unsupported target "bitset" with type "example" omitted
# Unsupported target "cluster_bomb" with type "example" omitted
# Unsupported target "common" with type "example" omitted
# Unsupported target "full" with type "example" omitted
# Unsupported target "parallel" with type "bench" omitted
# Unsupported target "serialize" with type "example" omitted

rust_library(
    name = "specs",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__crossbeam__0_3_2//:crossbeam",
        "@complicated_cargo_library__derivative__1_0_0//:derivative",
        "@complicated_cargo_library__fnv__1_0_6//:fnv",
        "@complicated_cargo_library__hibitset__0_3_2//:hibitset",
        "@complicated_cargo_library__mopa__0_2_2//:mopa",
        "@complicated_cargo_library__rayon__0_8_2//:rayon",
        "@complicated_cargo_library__shred__0_5_2//:shred",
        "@complicated_cargo_library__shred_derive__0_3_0//:shred_derive",
        "@complicated_cargo_library__tuple_utils__0_2_0//:tuple_utils",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.10.0",
    crate_features = [
    ],
)

# Unsupported target "storage" with type "bench" omitted
# Unsupported target "tests" with type "test" omitted
# Unsupported target "world" with type "bench" omitted
