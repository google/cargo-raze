"""This module is the single location for all dependencies for the Cargo Raze examples"""

load("@cargo_raze_examples//remote/binary_dependencies/cargo:crates.bzl", "remote_binary_dependencies_fetch_remote_crates")
load("@cargo_raze_examples//remote/build_dependencies/cargo:crates.bzl", "remote_build_dependencies_fetch_remote_crates")
load("@cargo_raze_examples//remote/cargo_workspace/cargo:crates.bzl", "remote_cargo_workspace_fetch_remote_crates")
load("@cargo_raze_examples//remote/complicated_cargo_library/cargo:crates.bzl", "remote_complicated_cargo_library_fetch_remote_crates")
load("@cargo_raze_examples//remote/dev_dependencies/cargo:crates.bzl", "remote_dev_dependencies_fetch_remote_crates")
load("@cargo_raze_examples//remote/no_deps/cargo:crates.bzl", "remote_no_deps_fetch_remote_crates")
load("@cargo_raze_examples//remote/non_cratesio/cargo:crates.bzl", "remote_non_cratesio_fetch_remote_crates")
load("@cargo_raze_examples//remote/parent_directory_workspace/cargo:crates.bzl", "remote_parent_directory_workspace_fetch_remote_crates")

def repositories(local_path_prefix = None):
    """Defines all the cargo dependencies of the Cargo-raze examples"""
    remote_binary_dependencies_fetch_remote_crates(local_path_prefix = local_path_prefix)
    remote_cargo_workspace_fetch_remote_crates(local_path_prefix = local_path_prefix)
    remote_parent_directory_workspace_fetch_remote_crates(local_path_prefix = local_path_prefix)
    remote_complicated_cargo_library_fetch_remote_crates(local_path_prefix = local_path_prefix)
    remote_no_deps_fetch_remote_crates(local_path_prefix = local_path_prefix)
    remote_non_cratesio_fetch_remote_crates(local_path_prefix = local_path_prefix)
    remote_dev_dependencies_fetch_remote_crates(local_path_prefix = local_path_prefix)
    remote_build_dependencies_fetch_remote_crates(local_path_prefix = local_path_prefix)
