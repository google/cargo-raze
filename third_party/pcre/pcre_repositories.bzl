"""A module defining the third party dependency PCRE"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def pcre_repositories():
    maybe(
        http_archive,
        name = "cargo_raze__pcre",
        build_file = Label("//third_party/pcre:BUILD.pcre.bazel"),
        sha256 = "aecafd4af3bd0f3935721af77b889d9024b2e01d96b58471bd91a3063fb47728",
        strip_prefix = "pcre-8.44",
        urls = [
            "https://mirror.bazel.build/ftp.pcre.org/pub/pcre/pcre-8.44.tar.gz",
            "https://downloads.sourceforge.net/project/pcre/pcre/8.44/pcre-8.44.tar.gz",
        ],
    )
