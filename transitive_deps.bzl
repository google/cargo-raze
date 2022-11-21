"""A module defining the transitive dependencies of cargo-raze"""

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")
load("@rules_rust//rust:defs.bzl", "rust_common")
load("@rules_rust//rust:repositories.bzl", "DEFAULT_TOOLCHAIN_TRIPLES", "rust_repositories", "rust_toolchain_tools_repository")

def cargo_raze_transitive_deps():
    """Loads all dependnecies from repositories required for cargo-raze"""

    bazel_skylib_workspace()

    rules_foreign_cc_dependencies()
    rust_repositories(edition = "2021", include_rustc_srcs = True)
    for exec_triple, name in DEFAULT_TOOLCHAIN_TRIPLES.items():
        maybe(
            rust_toolchain_tools_repository,
            name + "_tools",
            edition = "2021",
            exec_triple = exec_triple,
            target_triple = exec_triple,
            version = rust_common.default_version,
        )
