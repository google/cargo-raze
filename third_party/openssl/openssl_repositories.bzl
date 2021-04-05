"""A module defining the third party dependency OpenSSL"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def openssl_repositories():
    maybe(
        http_archive,
        name = "cargo_raze__openssl",
        build_file = Label("//third_party/openssl:BUILD.openssl.bazel"),
        sha256 = "892a0875b9872acd04a9fde79b1f943075d5ea162415de3047c327df33fbaee5",
        strip_prefix = "openssl-1.1.1k",
        urls = [
            "https://www.openssl.org/source/openssl-1.1.1k.tar.gz",
        ],
    )
