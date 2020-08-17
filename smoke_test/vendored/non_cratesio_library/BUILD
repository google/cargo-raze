load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "non_cratesio_library",
    srcs = ["src/main.rs"],
    deps = [
        "//vendored/non_cratesio_library/cargo:futures__0_2_0",
        "//vendored/non_cratesio_library/cargo:env_logger__0_5_5",
        "//vendored/non_cratesio_library/cargo:log__0_4_0"
    ],
)
