"""A module defining the third party dependency curl"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def curl_repositories():
    maybe(
        http_archive,
        name = "cargo_raze__curl",
        urls = [
            "https://curl.se/download/curl-7.76.0.tar.gz",
        ],
        type = "tar.gz",
        sha256 = "3b4378156ba09e224008e81dcce854b7ce4d182b1f9cfb97fe5ed9e9c18c6bd3",
        strip_prefix = "curl-7.76.0",
        build_file = Label("//third_party/curl:BUILD.curl.bazel"),
    )
