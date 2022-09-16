"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

"""
Args:
    local_path_prefix: An optional prefix to append to local paths within the Bazel repository.
        Many uses should use `bazel_workspace_path` in the raze settings instead, this is only
        for unusual sitations which use the same fetch_remote_crates from multiple repositories.
"""
def remote_no_deps_fetch_remote_crates(local_path_prefix = ""):
    _ = local_path_prefix
    """No crates were detected in the source Cargo.toml. This is a no-op"""
    pass
