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


# Unsupported target "bench1" with type "bench" omitted
# Unsupported target "iris" with type "example" omitted

rust_library(
    name = "itertools",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__either__1_4_0//:either",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.5.10",
    crate_features = [
    ],
)

# Unsupported target "peeking_take_while" with type "test" omitted
# Unsupported target "quick" with type "test" omitted
# Unsupported target "tests" with type "test" omitted
# Unsupported target "tuple_combinations" with type "bench" omitted
# Unsupported target "tuples" with type "bench" omitted
# Unsupported target "tuples" with type "test" omitted
# Unsupported target "zip" with type "test" omitted
