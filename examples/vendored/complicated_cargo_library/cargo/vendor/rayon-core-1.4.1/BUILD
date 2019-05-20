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
  "notice", # "Apache-2.0,MIT"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "build-script-build" with type "custom-build" omitted
# Unsupported target "double_init_fail" with type "test" omitted
# Unsupported target "init_zero_threads" with type "test" omitted

rust_library(
    name = "rayon_core",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/complicated_cargo_library/cargo/vendor/crossbeam-deque-0.2.0:crossbeam_deque",
        "//vendored/complicated_cargo_library/cargo/vendor/lazy_static-1.3.0:lazy_static",
        "//vendored/complicated_cargo_library/cargo/vendor/libc-0.2.53:libc",
        "//vendored/complicated_cargo_library/cargo/vendor/num_cpus-1.10.0:num_cpus",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "1.4.1",
    crate_features = [
    ],
)

# Unsupported target "scope_join" with type "test" omitted
# Unsupported target "simple_panic" with type "test" omitted
# Unsupported target "stack_overflow_crash" with type "test" omitted
