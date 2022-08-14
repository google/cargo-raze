"""A module defining the third party dependency OpenSSL"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def openssl_repositories():
    maybe(
        http_archive,
        name = "cargo_raze__openssl",
        build_file = Label("//third_party/openssl:BUILD.openssl.bazel"),
        sha256 = "0f745b85519aab2ce444a3dcada93311ba926aea2899596d01e7f948dbd99981",
        strip_prefix = "openssl-1.1.1o",
        urls = [
            "https://www.openssl.org/source/openssl-1.1.1o.tar.gz",
        ],
    )
