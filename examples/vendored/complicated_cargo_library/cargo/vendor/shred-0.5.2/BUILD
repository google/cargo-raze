"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//vendored/complicated_cargo_library/cargo", which limits external
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
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/arrayvec-0.3.25:arrayvec",
        "//vendored/complicated_cargo_library/cargo/vendor/fnv-1.0.6:fnv",
        "//vendored/complicated_cargo_library/cargo/vendor/mopa-0.2.2:mopa",
        "//vendored/complicated_cargo_library/cargo/vendor/pulse-0.5.3:pulse",
        "//vendored/complicated_cargo_library/cargo/vendor/rayon-0.8.2:rayon",
        "//vendored/complicated_cargo_library/cargo/vendor/shred-derive-0.3.0:shred_derive",
        "//vendored/complicated_cargo_library/cargo/vendor/smallvec-0.4.4:smallvec",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.5.2",
    crate_features = [
    ],
)

# Unsupported target "thread_local" with type "example" omitted
