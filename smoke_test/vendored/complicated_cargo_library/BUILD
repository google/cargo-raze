load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "complicated_cargo_library",
    srcs = ["src/main.rs"],
    deps = [
        "//vendored/complicated_cargo_library/cargo:regex__0_2_11",
        "//vendored/complicated_cargo_library/cargo:specs__0_10_0",
    ],
)
