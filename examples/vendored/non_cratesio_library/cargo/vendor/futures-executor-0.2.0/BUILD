"""
@generated
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
  "notice", # MIT from expression "MIT OR Apache-2.0"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "futures_executor",
    crate_type = "lib",
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/futures-channel-0.2.0:futures_channel",
        "//vendored/non_cratesio_library/cargo/vendor/futures-core-0.2.0:futures_core",
        "//vendored/non_cratesio_library/cargo/vendor/futures-util-0.2.0:futures_util",
        "//vendored/non_cratesio_library/cargo/vendor/lazy_static-1.0.0:lazy_static",
        "//vendored/non_cratesio_library/cargo/vendor/num_cpus-1.8.0:num_cpus",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.2.0",
    tags = ["cargo-raze"],
    crate_features = [
        "lazy_static",
        "num_cpus",
        "std",
    ],
)

# Unsupported target "local_pool" with type "test" omitted
# Unsupported target "poll" with type "bench" omitted
# Unsupported target "thread_notify" with type "bench" omitted
