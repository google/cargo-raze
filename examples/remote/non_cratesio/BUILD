load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")
load("//remote/non_cratesio/cargo:crates.bzl", "all_crates")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "non_cratesio_remote",
    srcs = ["src/main.rs"],
    deps = all_crates(),
)
