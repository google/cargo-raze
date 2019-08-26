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


# Unsupported target "all" with type "test" omitted
# Unsupported target "async_await_tests" with type "test" omitted
# Unsupported target "buffer_unordered" with type "test" omitted
# Unsupported target "eager_drop" with type "test" omitted
# Unsupported target "eventual" with type "test" omitted
# Unsupported target "fuse" with type "test" omitted
# Unsupported target "future_flatten_stream" with type "test" omitted

rust_library(
    name = "futures",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    edition = "2015",
    srcs = glob(["**/*.rs"]),
    deps = [
        "//vendored/non_cratesio_library/cargo/vendor/futures-channel-0.2.0:futures_channel",
        "//vendored/non_cratesio_library/cargo/vendor/futures-core-0.2.0:futures_core",
        "//vendored/non_cratesio_library/cargo/vendor/futures-executor-0.2.0:futures_executor",
        "//vendored/non_cratesio_library/cargo/vendor/futures-io-0.2.0:futures_io",
        "//vendored/non_cratesio_library/cargo/vendor/futures-sink-0.2.0:futures_sink",
        "//vendored/non_cratesio_library/cargo/vendor/futures-stable-0.2.0:futures_stable",
        "//vendored/non_cratesio_library/cargo/vendor/futures-util-0.2.0:futures_util",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.2.0",
    crate_features = [
        "default",
        "futures-core",
        "futures-executor",
        "futures-io",
        "futures-sink",
        "futures-stable",
        "futures-util",
        "std",
    ],
)

# Unsupported target "futures_ordered" with type "test" omitted
# Unsupported target "futures_unordered" with type "test" omitted
# Unsupported target "inspect" with type "test" omitted
# Unsupported target "ready_queue" with type "test" omitted
# Unsupported target "recurse" with type "test" omitted
# Unsupported target "select_all" with type "test" omitted
# Unsupported target "select_ok" with type "test" omitted
# Unsupported target "shared" with type "test" omitted
# Unsupported target "sink" with type "test" omitted
# Unsupported target "split" with type "test" omitted
# Unsupported target "stream" with type "test" omitted
# Unsupported target "stream_catch_unwind" with type "test" omitted
# Unsupported target "stream_select_all" with type "test" omitted
# Unsupported target "unfold" with type "test" omitted
