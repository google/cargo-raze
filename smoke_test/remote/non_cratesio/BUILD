load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "non_cratesio_remote",
    srcs = ["src/main.rs"],
    deps = [
        "//remote/non_cratesio/cargo:env_logger__0_5_5",
        "//remote/non_cratesio/cargo:log__0_4_0",
        "//remote/non_cratesio/cargo:rand__0_4_1",
    ],
)
