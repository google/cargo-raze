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
  "notice", # "BSD-3-Clause"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)



rust_library(
    name = "fuchsia_zircon",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@complicated_cargo_library__bitflags__1_0_1//:bitflags",
        "@complicated_cargo_library__fuchsia_zircon_sys__0_3_3//:fuchsia_zircon_sys",
    ],
    rustc_flags = [
        "--cap-lints=allow",
        "--target=x86_64-unknown-linux-gnu",
    ],
    version = "0.3.3",
    crate_features = [
    ],
)

