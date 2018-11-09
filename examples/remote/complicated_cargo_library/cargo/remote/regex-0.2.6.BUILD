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


# Unsupported target "backtrack" with type "test" omitted
# Unsupported target "backtrack-bytes" with type "test" omitted
# Unsupported target "backtrack-utf8bytes" with type "test" omitted
# Unsupported target "default" with type "test" omitted
# Unsupported target "default-bytes" with type "test" omitted
# Unsupported target "nfa" with type "test" omitted
# Unsupported target "nfa-bytes" with type "test" omitted
# Unsupported target "nfa-utf8bytes" with type "test" omitted

rust_library(
    name = "regex",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__aho_corasick__0_6_4//:aho_corasick",
        "@complicated_cargo_library__memchr__2_0_1//:memchr",
        "@complicated_cargo_library__regex_syntax__0_4_2//:regex_syntax",
        "@complicated_cargo_library__thread_local__0_3_5//:thread_local",
        "@complicated_cargo_library__utf8_ranges__1_0_0//:utf8_ranges",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.2.6",
    crate_features = [
    ],
)

# Unsupported target "shootout-regex-dna" with type "example" omitted
# Unsupported target "shootout-regex-dna-bytes" with type "example" omitted
# Unsupported target "shootout-regex-dna-cheat" with type "example" omitted
# Unsupported target "shootout-regex-dna-replace" with type "example" omitted
# Unsupported target "shootout-regex-dna-single" with type "example" omitted
# Unsupported target "shootout-regex-dna-single-cheat" with type "example" omitted
