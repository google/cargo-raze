load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")
load("//vendored/hello_cargo_library/cargo:crates.bzl", "all_crates")

package(default_visibility = ["//visibility:public"])

rust_binary(
    name = "hello_cargo_library",
    srcs = ["src/main.rs"],
    deps = all_crates(),
)
