"""A module for defining repositories cargo-raze depends on"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("//third_party/cargo:crates.bzl", "cargo_raze_fetch_remote_crates")
load("//third_party/curl:curl_repositories.bzl", "curl_repositories")
load("//third_party/iconv:iconv_repositories.bzl", "iconv_repositories")
load("//third_party/libgit2:libgit2_repositories.bzl", "libgit2_repositories")
load("//third_party/libssh2:libssh2_repositories.bzl", "libssh2_repositories")
load("//third_party/openssl:openssl_repositories.bzl", "openssl_repositories")
load("//third_party/pcre:pcre_repositories.bzl", "pcre_repositories")
load("//third_party/zlib:zlib_repositories.bzl", "zlib_repositories")

def cargo_raze_repositories():
    """Creates repository definitions for all cargo-raze third party dependencies"""

    maybe(
        http_archive,
        name = "rules_rust",
        sha256 = "617082067629939c0a22f587811a3e822a50a203119a90380e21f5aec3373da9",
        strip_prefix = "rules_rust-e07881fa22a5f0d16230d8b23bbff2bf358823b8",
        # Main branch as of 2022-04-27
        url = "https://github.com/bazelbuild/rules_rust/archive/e07881fa22a5f0d16230d8b23bbff2bf358823b8.tar.gz",
    )

    maybe(
        http_archive,
        name = "rules_foreign_cc",
        sha256 = "69023642d5781c68911beda769f91fcbc8ca48711db935a75da7f6536b65047f",
        strip_prefix = "rules_foreign_cc-0.6.0",
        url = "https://github.com/bazelbuild/rules_foreign_cc/archive/0.6.0.tar.gz",
    )

    maybe(
        http_archive,
        name = "rules_cc",
        url = "https://github.com/bazelbuild/rules_cc/archive/c612c9581b9e740a49ed4c006edb93912c8ab205.zip",
        sha256 = "1bef6433ba1a4288b5853dc0ebd6cf436dc1c83cce6e16abf73e7ad1b785def4",
        strip_prefix = "rules_cc-c612c9581b9e740a49ed4c006edb93912c8ab205",
        type = "zip",
    )

    curl_repositories()
    iconv_repositories()
    libgit2_repositories()
    libssh2_repositories()
    openssl_repositories()
    pcre_repositories()
    zlib_repositories()

    cargo_raze_fetch_remote_crates()
