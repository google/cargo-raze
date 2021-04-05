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
        sha256 = "cdc853460de2245b3b49a0a9b178d20d7768c94da4d78d0b2c6d7048df2d4f11",
        strip_prefix = "rules_rust-a667ff9bdc3a800e3005e8f19b89283a6a1be5a4",
        # Main branch as of 2021-04-02
        url = "https://github.com/bazelbuild/rules_rust/archive/a667ff9bdc3a800e3005e8f19b89283a6a1be5a4.tar.gz",
    )

    maybe(
        http_archive,
        name = "rules_foreign_cc",
        sha256 = "a45511a054598dd9b87d4d5765a18df4e5777736026087cf96ffc30704e6c918",
        strip_prefix = "rules_foreign_cc-87df6b25f6c009883da87f07ea680d38780a4d6f",
        urls = [
            "https://github.com/bazelbuild/rules_foreign_cc/archive/87df6b25f6c009883da87f07ea680d38780a4d6f.zip",
        ],
    )

    maybe(
        http_archive,
        name = "rules_cc",
        url = "https://github.com/bazelbuild/rules_cc/archive/624b5d59dfb45672d4239422fa1e3de1822ee110.zip",
        sha256 = "8c7e8bf24a2bf515713445199a677ee2336e1c487fa1da41037c6026de04bbc3",
        strip_prefix = "rules_cc-624b5d59dfb45672d4239422fa1e3de1822ee110",
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
