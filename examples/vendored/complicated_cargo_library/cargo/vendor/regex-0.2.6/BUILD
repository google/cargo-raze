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
        "//vendored/complicated_cargo_library/cargo/vendor/aho-corasick-0.6.4:aho_corasick",
        "//vendored/complicated_cargo_library/cargo/vendor/memchr-2.0.1:memchr",
        "//vendored/complicated_cargo_library/cargo/vendor/regex-syntax-0.4.2:regex_syntax",
        "//vendored/complicated_cargo_library/cargo/vendor/thread_local-0.3.5:thread_local",
        "//vendored/complicated_cargo_library/cargo/vendor/utf8-ranges-1.0.0:utf8_ranges",
    ],
    rustc_flags = [
        "--cap-lints allow",
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
