load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "complicated_cargo_library",
    srcs = ["src/main.rs"],
    deps = [
        "//remote/complicated_cargo_library/cargo:regex",
        "//remote/complicated_cargo_library/cargo:specs",
    ],
)
