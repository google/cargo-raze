"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//vendored/non_cratesio_library/cargo", which limits external
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



rust_library(
    name = "futures_executor",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/futures-channel-0.2.0-alpha:futures_channel",
        "//vendored/non_cratesio_library/cargo/vendor/futures-core-0.2.0-alpha:futures_core",
        "//vendored/non_cratesio_library/cargo/vendor/futures-util-0.2.0-alpha:futures_util",
        "//vendored/non_cratesio_library/cargo/vendor/num_cpus-1.8.0:num_cpus",
    ],
    rustc_flags = [
        "--cap-lints allow",
    ],
    version = "0.2.0-alpha",
    crate_features = [
        "futures-channel",
        "futures-core",
        "futures-util",
        "num_cpus",
        "std",
    ],
)

# Unsupported target "local_pool" with type "test" omitted
# Unsupported target "poll" with type "bench" omitted
# Unsupported target "thread_notify" with type "bench" omitted
