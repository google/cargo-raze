load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "hello_cargo_library",
    srcs = ["src/main.rs"],
    deps = [
        "//vendored/hello_cargo_library/cargo:fern__0_3_5",
        "//vendored/hello_cargo_library/cargo:log__0_3_9",
    ],
)
