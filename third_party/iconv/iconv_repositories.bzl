"""A module defining the third party dependency iconv"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def iconv_repositories():
    maybe(
        http_archive,
        name = "cargo_raze__iconv",
        urls = [
            "https://github.com/apple-oss-distributions/libiconv/archive/libiconv-61.tar.gz",
        ],
        type = "tar.gz",
        sha256 = "cfc6eb15fc7de4413323bdd01bfd050da54577895fcc6832020534647812aacd",
        strip_prefix = "libiconv-libiconv-61/libiconv",
        build_file = Label("//third_party/iconv:BUILD.iconv.bazel"),
    )
