"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = ["//vendored/non_cratesio_library/cargo:__subpackages__"])

licenses([
  "notice", # "MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_bench_test",
)


# Unsupported target "alternate_screen" with type "example" omitted
# Unsupported target "alternate_screen_raw" with type "example" omitted
# Unsupported target "async" with type "example" omitted
# Unsupported target "click" with type "example" omitted
# Unsupported target "color" with type "example" omitted
# Unsupported target "commie" with type "example" omitted
# Unsupported target "detect_color" with type "example" omitted
# Unsupported target "is_tty" with type "example" omitted
# Unsupported target "keys" with type "example" omitted
# Unsupported target "mouse" with type "example" omitted
# Unsupported target "rainbow" with type "example" omitted
# Unsupported target "read" with type "example" omitted
# Unsupported target "rustc_fun" with type "example" omitted
# Unsupported target "simple" with type "example" omitted
# Unsupported target "size" with type "example" omitted

rust_library(
    name = "termion",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/libc-0.2.39:libc",
    ],
    rustc_flags = [
        "--cap-lints allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    crate_features = [
    ],
)

# Unsupported target "truecolor" with type "example" omitted
