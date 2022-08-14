"""A module defining the third party dependency OpenSSL"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def openssl_repositories():
    maybe(
        http_archive,
        name = "cargo_raze__openssl",
        build_file = Label("//third_party/openssl:BUILD.openssl.bazel"),
        sha256 = "9384a2b0570dd80358841464677115df785edb941c71211f75076d72fe6b438f",
        strip_prefix = "openssl-1.1.1o",
        urls = [
            "https://www.openssl.org/source/openssl-1.1.1o.tar.gz",
        ],
    )
