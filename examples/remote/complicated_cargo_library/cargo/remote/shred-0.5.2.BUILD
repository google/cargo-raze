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
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "basic_dispatch" with type "example" omitted
# Unsupported target "bench" with type "bench" omitted
# Unsupported target "custom_bundle" with type "example" omitted
# Unsupported target "derive_bundle" with type "example" omitted
# Unsupported target "dispatch" with type "test" omitted
# Unsupported target "fetch_opt" with type "example" omitted
# Unsupported target "generic_derive" with type "example" omitted
# Unsupported target "par_seq" with type "example" omitted
# Unsupported target "seq_dispatch" with type "example" omitted

rust_library(
    name = "shred",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__arrayvec__0_3_25//:arrayvec",
        "@complicated_cargo_library__fnv__1_0_6//:fnv",
        "@complicated_cargo_library__mopa__0_2_2//:mopa",
        "@complicated_cargo_library__pulse__0_5_3//:pulse",
        "@complicated_cargo_library__rayon__0_8_2//:rayon",
        "@complicated_cargo_library__shred_derive__0_3_0//:shred_derive",
        "@complicated_cargo_library__smallvec__0_4_5//:smallvec",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.5.2",
    crate_features = [
    ],
)

# Unsupported target "thread_local" with type "example" omitted
