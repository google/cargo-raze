"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//visibility:public"])

licenses([
  "notice", # "MIT,Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)

# Unsupported target "compile-test" with type "test" omitted

rust_library(
    name = "derivative",
    crate_root = "src/lib.rs",
    crate_type = "proc-macro",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__itertools__0_5_10//:itertools",
        "@complicated_cargo_library__quote__0_3_15//:quote",
        "@complicated_cargo_library__syn__0_10_8//:syn",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "derive-clone" with type "test" omitted
# Unsupported target "derive-debug" with type "test" omitted
# Unsupported target "derive-debug-bounds" with type "test" omitted
# Unsupported target "derive-debug-generics" with type "test" omitted
# Unsupported target "derive-debug-transparent" with type "test" omitted
# Unsupported target "derive-default" with type "test" omitted
# Unsupported target "derive-default-bounds" with type "test" omitted
# Unsupported target "derive-eq" with type "test" omitted
# Unsupported target "derive-hash" with type "test" omitted
# Unsupported target "derive-partial-eq" with type "test" omitted
# Unsupported target "rustc-class-implement-traits" with type "test" omitted
# Unsupported target "rustc-deriving-bounds" with type "test" omitted
# Unsupported target "rustc-deriving-clone-array" with type "test" omitted
# Unsupported target "rustc-deriving-clone-enum" with type "test" omitted
# Unsupported target "rustc-deriving-clone-generic-enum" with type "test" omitted
# Unsupported target "rustc-deriving-clone-generic-tuple-struct" with type "test" omitted
# Unsupported target "rustc-deriving-clone-struct" with type "test" omitted
# Unsupported target "rustc-deriving-clone-tuple-struct" with type "test" omitted
# Unsupported target "rustc-deriving-cmp-generic-enum" with type "test" omitted
# Unsupported target "rustc-deriving-copyclone" with type "test" omitted
# Unsupported target "rustc-deriving-default-box" with type "test" omitted
# Unsupported target "rustc-deriving-enum-single-variant" with type "test" omitted
# Unsupported target "rustc-deriving-hash" with type "test" omitted
# Unsupported target "rustc-deriving-in-fn" with type "test" omitted
# Unsupported target "rustc-deriving-meta" with type "test" omitted
# Unsupported target "rustc-deriving-meta-multiple" with type "test" omitted
# Unsupported target "rustc-deriving-show" with type "test" omitted
# Unsupported target "rustc-deriving-show-2" with type "test" omitted
# Unsupported target "rustc-deriving-via-extension-hash-enum" with type "test" omitted
# Unsupported target "rustc-deriving-via-extension-hash-struct" with type "test" omitted
# Unsupported target "rustc-deriving-via-extension-type-params" with type "test" omitted
# Unsupported target "rustc-expr-copy" with type "test" omitted
# Unsupported target "rustc-exterior" with type "test" omitted
# Unsupported target "rustc-issue-12860" with type "test" omitted
# Unsupported target "rustc-issue-13434" with type "test" omitted
# Unsupported target "rustc-issue-16530" with type "test" omitted
# Unsupported target "rustc-issue-19037" with type "test" omitted
# Unsupported target "rustc-issue-19102" with type "test" omitted
# Unsupported target "rustc-issue-19135" with type "test" omitted
# Unsupported target "rustc-issue-19358" with type "test" omitted
# Unsupported target "rustc-issue-21402" with type "test" omitted
# Unsupported target "rustc-issue-23649-3" with type "test" omitted
# Unsupported target "rustc-issue-24085" with type "test" omitted
# Unsupported target "rustc-issue-25394" with type "test" omitted
# Unsupported target "rustc-issue-28561" with type "test" omitted
# Unsupported target "rustc-issue-29030" with type "test" omitted
# Unsupported target "rustc-issue-29540" with type "test" omitted
# Unsupported target "rustc-issue-29710" with type "test" omitted
# Unsupported target "rustc-issue-32292" with type "test" omitted
# Unsupported target "rustc-issue-3935" with type "test" omitted
# Unsupported target "rustc-issue-6341" with type "test" omitted
# Unsupported target "rustc-typeclasses-eq-example" with type "test" omitted
# Unsupported target "rustc-zero-sized-btreemap-insert" with type "test" omitted
