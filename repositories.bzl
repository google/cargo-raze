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
        sha256 = "696b01deea96a5e549f1b5ae18589e1bbd5a1d71a36a243b5cf76a9433487cf2",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.11.0/rules_rust-v0.11.0.tar.gz",
            "https://github.com/bazelbuild/rules_rust/releases/download/0.11.0/rules_rust-v0.11.0.tar.gz",
        ],
    )

    maybe(
        http_archive,
        name = "rules_foreign_cc",
        sha256 = "2a4d07cd64b0719b39a7c12218a3e507672b82a97b98c6a89d38565894cf7c51",
        strip_prefix = "rules_foreign_cc-0.9.0",
        url = "https://github.com/bazelbuild/rules_foreign_cc/archive/0.9.0.tar.gz",
    )

    maybe(
        http_archive,
        name = "rules_cc",
        url = "https://github.com/bazelbuild/rules_cc/archive/c612c9581b9e740a49ed4c006edb93912c8ab205.zip",
        sha256 = "1bef6433ba1a4288b5853dc0ebd6cf436dc1c83cce6e16abf73e7ad1b785def4",
        strip_prefix = "rules_cc-c612c9581b9e740a49ed4c006edb93912c8ab205",
        type = "zip",
    )

    maybe(
        http_archive,
        name = "bazel_skylib",
        urls = [
            "https://github.com/bazelbuild/bazel-skylib/releases/download/1.2.0/bazel-skylib-1.2.0.tar.gz",
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.2.0/bazel-skylib-1.2.0.tar.gz",
        ],
        sha256 = "af87959afe497dc8dfd4c6cb66e1279cb98ccc84284619ebfec27d9c09a903de",
    )

    curl_repositories()
    iconv_repositories()
    libgit2_repositories()
    libssh2_repositories()
    openssl_repositories()
    pcre_repositories()
    zlib_repositories()

    cargo_raze_fetch_remote_crates()
