"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        new_git_repository(name=name, **kwargs)

def cargo_workspace_fetch_remote_crates():



    _new_http_archive(
        name = "cargo_workspace__cfg_if__0_1_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.10.crate",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//remote/cargo_workspace/remote:cfg-if-0.1.10.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__getrandom__0_1_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/getrandom/getrandom-0.1.14.crate",
        type = "tar.gz",
        sha256 = "7abc8dd8451921606d809ba32e95b6111925cd2906060d2dcc29c070220503eb",
        strip_prefix = "getrandom-0.1.14",
        build_file = Label("//remote/cargo_workspace/remote:getrandom-0.1.14.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__libc__0_2_74",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.74.crate",
        type = "tar.gz",
        sha256 = "a2f02823cf78b754822df5f7f268fb59822e7296276d3e069d8e8cb26a14bd10",
        strip_prefix = "libc-0.2.74",
        build_file = Label("//remote/cargo_workspace/remote:libc-0.2.74.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__ppv_lite86__0_2_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ppv-lite86/ppv-lite86-0.2.8.crate",
        type = "tar.gz",
        sha256 = "237a5ed80e274dbc66f86bd59c1e25edc039660be53194b5fe0a482e0f2612ea",
        strip_prefix = "ppv-lite86-0.2.8",
        build_file = Label("//remote/cargo_workspace/remote:ppv-lite86-0.2.8.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__rand__0_7_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.7.3.crate",
        type = "tar.gz",
        sha256 = "6a6b1679d49b24bbfe0c803429aa1874472f50d9b363131f0e89fc356b544d03",
        strip_prefix = "rand-0.7.3",
        build_file = Label("//remote/cargo_workspace/remote:rand-0.7.3.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__rand_chacha__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand_chacha/rand_chacha-0.2.2.crate",
        type = "tar.gz",
        sha256 = "f4c8ed856279c9737206bf725bf36935d8666ead7aa69b52be55af369d193402",
        strip_prefix = "rand_chacha-0.2.2",
        build_file = Label("//remote/cargo_workspace/remote:rand_chacha-0.2.2.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__rand_core__0_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand_core/rand_core-0.5.1.crate",
        type = "tar.gz",
        sha256 = "90bde5296fc891b0cef12a6d03ddccc162ce7b2aff54160af9338f8d40df6d19",
        strip_prefix = "rand_core-0.5.1",
        build_file = Label("//remote/cargo_workspace/remote:rand_core-0.5.1.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__rand_hc__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand_hc/rand_hc-0.2.0.crate",
        type = "tar.gz",
        sha256 = "ca3129af7b92a17112d59ad498c6f81eaf463253766b90396d39ea7a39d6613c",
        strip_prefix = "rand_hc-0.2.0",
        build_file = Label("//remote/cargo_workspace/remote:rand_hc-0.2.0.BUILD.bazel"),
    )

    _new_http_archive(
        name = "cargo_workspace__wasi__0_9_0_wasi_snapshot_preview1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wasi/wasi-0.9.0+wasi-snapshot-preview1.crate",
        type = "tar.gz",
        sha256 = "cccddf32554fecc6acb585f82a32a72e28b48f8c4c1883ddfeeeaa96f7d8e519",
        strip_prefix = "wasi-0.9.0+wasi-snapshot-preview1",
        build_file = Label("//remote/cargo_workspace/remote:wasi-0.9.0+wasi-snapshot-preview1.BUILD.bazel"),
    )

