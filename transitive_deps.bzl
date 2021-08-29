"""A module defining the transitive dependencies of cargo-raze"""

load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")
load("@rules_rust//rust:repositories.bzl", "rust_repositories")

def cargo_raze_transitive_deps():
    """Loads all dependnecies from repositories required for cargo-raze"""
    rules_foreign_cc_dependencies()
    rust_repositories(include_rustc_srcs = True)
