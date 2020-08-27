"""
@generated
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def regression_test_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "regression_test__MacTypes_sys__2_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/MacTypes-sys/MacTypes-sys-2.1.0.crate",
        type = "tar.gz",
        sha256 = "eaf9f0d0b1cc33a4d2aee14fb4b2eac03462ef4db29c8ac4057327d8a71ad86f",
        strip_prefix = "MacTypes-sys-2.1.0",
        build_file = Label("//remote/regression_test/cargo/remote:MacTypes-sys-2.1.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__aho_corasick__0_6_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.10.crate",
        type = "tar.gz",
        sha256 = "81ce3d38065e618af2d7b77e10c5ad9a069859b4be3c2250f674af3840d9c8a5",
        strip_prefix = "aho-corasick-0.6.10",
        build_file = Label("//remote/regression_test/cargo/remote:aho-corasick-0.6.10.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__arrayvec__0_3_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.3.25.crate",
        type = "tar.gz",
        sha256 = "06f59fe10306bb78facd90d28c2038ad23ffaaefa85bac43c8a434cde383334f",
        strip_prefix = "arrayvec-0.3.25",
        build_file = Label("//remote/regression_test/cargo/remote:arrayvec-0.3.25.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__arrayvec__0_4_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.4.10.crate",
        type = "tar.gz",
        sha256 = "92c7fb76bc8826a8b33b4ee5bb07a247a81e76764ab4d55e8f73e3a4d8808c71",
        strip_prefix = "arrayvec-0.4.10",
        build_file = Label("//remote/regression_test/cargo/remote:arrayvec-0.4.10.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__atom__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atom/atom-0.3.5.crate",
        type = "tar.gz",
        sha256 = "3c86699c3f02778ec07158376991c8f783dd1f2f95c579ffaf0738dc984b2fe2",
        strip_prefix = "atom-0.3.5",
        build_file = Label("//remote/regression_test/cargo/remote:atom-0.3.5.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__cc__1_0_58",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cc/cc-1.0.58.crate",
        type = "tar.gz",
        sha256 = "f9a06fb2e53271d7c279ec1efea6ab691c35a2ae67ec0d91d7acec0caf13b518",
        strip_prefix = "cc-1.0.58",
        build_file = Label("//remote/regression_test/cargo/remote:cc-1.0.58.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__cfg_if__0_1_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.7.crate",
        type = "tar.gz",
        sha256 = "11d43355396e872eefb45ce6342e4374ed7bc2b3a502d1b28e36d6e23c05d1f4",
        strip_prefix = "cfg-if-0.1.7",
        build_file = Label("//remote/regression_test/cargo/remote:cfg-if-0.1.7.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__core_foundation_sys__0_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/core-foundation-sys/core-foundation-sys-0.5.1.crate",
        type = "tar.gz",
        sha256 = "716c271e8613ace48344f723b60b900a93150271e5be206212d052bbc0883efa",
        strip_prefix = "core-foundation-sys-0.5.1",
        build_file = Label("//remote/regression_test/cargo/remote:core-foundation-sys-0.5.1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__crossbeam__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam/crossbeam-0.3.2.crate",
        type = "tar.gz",
        sha256 = "24ce9782d4d5c53674646a6a4c1863a21a8fc0cb649b3c94dfc16e45071dea19",
        strip_prefix = "crossbeam-0.3.2",
        build_file = Label("//remote/regression_test/cargo/remote:crossbeam-0.3.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__crossbeam_deque__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-deque/crossbeam-deque-0.2.0.crate",
        type = "tar.gz",
        sha256 = "f739f8c5363aca78cfb059edf753d8f0d36908c348f3d8d1503f03d8b75d9cf3",
        strip_prefix = "crossbeam-deque-0.2.0",
        build_file = Label("//remote/regression_test/cargo/remote:crossbeam-deque-0.2.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__crossbeam_epoch__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-epoch/crossbeam-epoch-0.3.1.crate",
        type = "tar.gz",
        sha256 = "927121f5407de9956180ff5e936fe3cf4324279280001cd56b669d28ee7e9150",
        strip_prefix = "crossbeam-epoch-0.3.1",
        build_file = Label("//remote/regression_test/cargo/remote:crossbeam-epoch-0.3.1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__crossbeam_utils__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-utils/crossbeam-utils-0.2.2.crate",
        type = "tar.gz",
        sha256 = "2760899e32a1d58d5abb31129f8fae5de75220bc2176e77ff7c627ae45c918d9",
        strip_prefix = "crossbeam-utils-0.2.2",
        build_file = Label("//remote/regression_test/cargo/remote:crossbeam-utils-0.2.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__derivative__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derivative/derivative-1.0.2.crate",
        type = "tar.gz",
        sha256 = "6073e9676dbebdddeabaeb63e3b7cefd23c86f5c41d381ee1237cc77b1079898",
        strip_prefix = "derivative-1.0.2",
        build_file = Label("//remote/regression_test/cargo/remote:derivative-1.0.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__fnv__1_0_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fnv/fnv-1.0.6.crate",
        type = "tar.gz",
        sha256 = "2fad85553e09a6f881f739c29f0b00b0f01357c743266d478b68951ce23285f3",
        strip_prefix = "fnv-1.0.6",
        build_file = Label("//remote/regression_test/cargo/remote:fnv-1.0.6.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__hibitset__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hibitset/hibitset-0.3.2.crate",
        type = "tar.gz",
        sha256 = "b78998e3c243d71525596e8f373dfc4b82703f25907b9e4d260383cff8307d84",
        strip_prefix = "hibitset-0.3.2",
        build_file = Label("//remote/regression_test/cargo/remote:hibitset-0.3.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__lazy_static__1_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.3.0.crate",
        type = "tar.gz",
        sha256 = "bc5729f27f159ddd61f4df6228e827e86643d4d3e7c32183cb30a1c08f604a14",
        strip_prefix = "lazy_static-1.3.0",
        build_file = Label("//remote/regression_test/cargo/remote:lazy_static-1.3.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__libc__0_2_53",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.53.crate",
        type = "tar.gz",
        sha256 = "ec350a9417dfd244dc9a6c4a71e13895a4db6b92f0b106f07ebbc3f3bc580cee",
        strip_prefix = "libc-0.2.53",
        build_file = Label("//remote/regression_test/cargo/remote:libc-0.2.53.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__libloading__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libloading/libloading-0.5.2.crate",
        type = "tar.gz",
        sha256 = "f2b111a074963af1d37a139918ac6d49ad1d0d5e47f72fd55388619691a7d753",
        strip_prefix = "libloading-0.5.2",
        build_file = Label("//remote/regression_test/cargo/remote:libloading-0.5.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__memchr__2_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.2.0.crate",
        type = "tar.gz",
        sha256 = "2efc7bc57c883d4a4d6e3246905283d8dae951bb3bd32f49d6ef297f546e1c39",
        strip_prefix = "memchr-2.2.0",
        build_file = Label("//remote/regression_test/cargo/remote:memchr-2.2.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__memoffset__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memoffset/memoffset-0.2.1.crate",
        type = "tar.gz",
        sha256 = "0f9dc261e2b62d7a622bf416ea3c5245cdd5d9a7fcc428c0d06804dfce1775b3",
        strip_prefix = "memoffset-0.2.1",
        build_file = Label("//remote/regression_test/cargo/remote:memoffset-0.2.1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__mopa__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mopa/mopa-0.2.2.crate",
        type = "tar.gz",
        sha256 = "a785740271256c230f57462d3b83e52f998433a7062fc18f96d5999474a9f915",
        strip_prefix = "mopa-0.2.2",
        build_file = Label("//remote/regression_test/cargo/remote:mopa-0.2.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__nodrop__0_1_13",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nodrop/nodrop-0.1.13.crate",
        type = "tar.gz",
        sha256 = "2f9667ddcc6cc8a43afc9b7917599d7216aa09c463919ea32c59ed6cac8bc945",
        strip_prefix = "nodrop-0.1.13",
        build_file = Label("//remote/regression_test/cargo/remote:nodrop-0.1.13.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__num_cpus__1_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num_cpus/num_cpus-1.10.0.crate",
        type = "tar.gz",
        sha256 = "1a23f0ed30a54abaa0c7e83b1d2d87ada7c3c23078d1d87815af3e3b6385fbba",
        strip_prefix = "num_cpus-1.10.0",
        build_file = Label("//remote/regression_test/cargo/remote:num_cpus-1.10.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__odds__0_2_26",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/odds/odds-0.2.26.crate",
        type = "tar.gz",
        sha256 = "4eae0151b9dacf24fcc170d9995e511669a082856a91f958a2fe380bfab3fb22",
        strip_prefix = "odds-0.2.26",
        build_file = Label("//remote/regression_test/cargo/remote:odds-0.2.26.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__proc_macro2__0_4_28",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-0.4.28.crate",
        type = "tar.gz",
        sha256 = "ba92c84f814b3f9a44c5cfca7d2ad77fa10710867d2bbb1b3d175ab5f47daa12",
        strip_prefix = "proc-macro2-0.4.28",
        build_file = Label("//remote/regression_test/cargo/remote:proc-macro2-0.4.28.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__pulse__0_5_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/pulse/pulse-0.5.3.crate",
        type = "tar.gz",
        sha256 = "655612b6c8d96a8a02f331fe296cb4f925b68e87c1d195544675abca2d9b9af0",
        strip_prefix = "pulse-0.5.3",
        build_file = Label("//remote/regression_test/cargo/remote:pulse-0.5.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__quote__0_3_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.3.15.crate",
        type = "tar.gz",
        sha256 = "7a6e920b65c65f10b2ae65c831a81a073a89edd28c7cce89475bff467ab4167a",
        strip_prefix = "quote-0.3.15",
        build_file = Label("//remote/regression_test/cargo/remote:quote-0.3.15.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__quote__0_6_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.6.12.crate",
        type = "tar.gz",
        sha256 = "faf4799c5d274f3868a4aae320a0a182cbd2baee377b378f080e16a23e9d80db",
        strip_prefix = "quote-0.6.12",
        build_file = Label("//remote/regression_test/cargo/remote:quote-0.6.12.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__rayon__0_8_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon/rayon-0.8.2.crate",
        type = "tar.gz",
        sha256 = "b614fe08b6665cb9a231d07ac1364b0ef3cb3698f1239ee0c4c3a88a524f54c8",
        strip_prefix = "rayon-0.8.2",
        build_file = Label("//remote/regression_test/cargo/remote:rayon-0.8.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__rayon_core__1_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon-core/rayon-core-1.4.1.crate",
        type = "tar.gz",
        sha256 = "b055d1e92aba6877574d8fe604a63c8b5df60f60e5982bf7ccbb1338ea527356",
        strip_prefix = "rayon-core-1.4.1",
        build_file = Label("//remote/regression_test/cargo/remote:rayon-core-1.4.1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__redox_syscall__0_1_54",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.54.crate",
        type = "tar.gz",
        sha256 = "12229c14a0f65c4f1cb046a3b52047cdd9da1f4b30f8a39c5063c8bae515e252",
        strip_prefix = "redox_syscall-0.1.54",
        build_file = Label("//remote/regression_test/cargo/remote:redox_syscall-0.1.54.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__regex__0_2_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.5.crate",
        type = "tar.gz",
        sha256 = "744554e01ccbd98fff8c457c3b092cd67af62a555a43bfe97ae8a0451f7799fa",
        strip_prefix = "regex-0.2.5",
        build_file = Label("//remote/regression_test/cargo/remote:regex-0.2.5.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__regex_syntax__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.4.2.crate",
        type = "tar.gz",
        sha256 = "8e931c58b93d86f080c734bfd2bce7dd0079ae2331235818133c8be7f422e20e",
        strip_prefix = "regex-syntax-0.4.2",
        build_file = Label("//remote/regression_test/cargo/remote:regex-syntax-0.4.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__scopeguard__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scopeguard/scopeguard-0.3.3.crate",
        type = "tar.gz",
        sha256 = "94258f53601af11e6a49f722422f6e3425c52b06245a5cf9bc09908b174f5e27",
        strip_prefix = "scopeguard-0.3.3",
        build_file = Label("//remote/regression_test/cargo/remote:scopeguard-0.3.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__security_framework_sys__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/security-framework-sys/security-framework-sys-0.2.3.crate",
        type = "tar.gz",
        sha256 = "3d6696852716b589dff9e886ff83778bb635150168e83afa8ac6b8a78cb82abc",
        strip_prefix = "security-framework-sys-0.2.3",
        build_file = Label("//remote/regression_test/cargo/remote:security-framework-sys-0.2.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__shred__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred/shred-0.5.2.crate",
        type = "tar.gz",
        sha256 = "7d3abceaa9d0a9b47ab84b53c6029c21bcad7d7dd63e14db51ea0680faee2159",
        strip_prefix = "shred-0.5.2",
        build_file = Label("//remote/regression_test/cargo/remote:shred-0.5.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__shred_derive__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred-derive/shred-derive-0.3.0.crate",
        type = "tar.gz",
        sha256 = "a4a894913b6e93fe2cd712a3bc955ec6f6b01c675c1c58b02fdfa13f77868049",
        strip_prefix = "shred-derive-0.3.0",
        build_file = Label("//remote/regression_test/cargo/remote:shred-derive-0.3.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__smallvec__0_4_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.4.5.crate",
        type = "tar.gz",
        sha256 = "f90c5e5fe535e48807ab94fc611d323935f39d4660c52b26b96446a7b33aef10",
        strip_prefix = "smallvec-0.4.5",
        build_file = Label("//remote/regression_test/cargo/remote:smallvec-0.4.5.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__specs__0_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/specs/specs-0.10.0.crate",
        type = "tar.gz",
        sha256 = "a210dc96ea065cb88391aa6956ed1b2a14051c668b5bc18bac66a95c215b639f",
        strip_prefix = "specs-0.10.0",
        build_file = Label("//remote/regression_test/cargo/remote:specs-0.10.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__syn__0_11_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.11.11.crate",
        type = "tar.gz",
        sha256 = "d3b891b9015c88c576343b9b3e41c2c11a51c219ef067b264bd9c8aa9b441dad",
        strip_prefix = "syn-0.11.11",
        build_file = Label("//remote/regression_test/cargo/remote:syn-0.11.11.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__syn__0_15_33",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.15.33.crate",
        type = "tar.gz",
        sha256 = "ec52cd796e5f01d0067225a5392e70084acc4c0013fa71d55166d38a8b307836",
        strip_prefix = "syn-0.15.33",
        build_file = Label("//remote/regression_test/cargo/remote:syn-0.15.33.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__synom__0_11_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/synom/synom-0.11.3.crate",
        type = "tar.gz",
        sha256 = "a393066ed9010ebaed60b9eafa373d4b1baac186dd7e008555b0f702b51945b6",
        strip_prefix = "synom-0.11.3",
        build_file = Label("//remote/regression_test/cargo/remote:synom-0.11.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__thread_local__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.6.crate",
        type = "tar.gz",
        sha256 = "c6b53e329000edc2b34dbe8545fd20e55a333362d0a321909685a19bd28c3f1b",
        strip_prefix = "thread_local-0.3.6",
        build_file = Label("//remote/regression_test/cargo/remote:thread_local-0.3.6.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__time__0_1_42",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.42.crate",
        type = "tar.gz",
        sha256 = "db8dcfca086c1143c9270ac42a2bbd8a7ee477b78ac8e45b19abfb0cbede4b6f",
        strip_prefix = "time-0.1.42",
        build_file = Label("//remote/regression_test/cargo/remote:time-0.1.42.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__tuple_utils__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tuple_utils/tuple_utils-0.2.0.crate",
        type = "tar.gz",
        sha256 = "cbfecd7bb8f0a3e96b3b31c46af2677a55a588767c0091f484601424fcb20e7e",
        strip_prefix = "tuple_utils-0.2.0",
        build_file = Label("//remote/regression_test/cargo/remote:tuple_utils-0.2.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__unicode_xid__0_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.0.4.crate",
        type = "tar.gz",
        sha256 = "8c1f860d7d29cf02cb2f3f359fd35991af3d30bac52c57d265a3c461074cb4dc",
        strip_prefix = "unicode-xid-0.0.4",
        build_file = Label("//remote/regression_test/cargo/remote:unicode-xid-0.0.4.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__unicode_xid__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.1.0.crate",
        type = "tar.gz",
        sha256 = "fc72304796d0818e357ead4e000d19c9c174ab23dc11093ac919054d20a6a7fc",
        strip_prefix = "unicode-xid-0.1.0",
        build_file = Label("//remote/regression_test/cargo/remote:unicode-xid-0.1.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__utf8_ranges__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.2.crate",
        type = "tar.gz",
        sha256 = "796f7e48bef87609f7ade7e06495a87d5cd06c7866e6a5cbfceffc558a243737",
        strip_prefix = "utf8-ranges-1.0.2",
        build_file = Label("//remote/regression_test/cargo/remote:utf8-ranges-1.0.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__winapi__0_3_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.7.crate",
        type = "tar.gz",
        sha256 = "f10e386af2b13e47c89e7236a7a14a086791a2b88ebad6df9bf42040195cf770",
        strip_prefix = "winapi-0.3.7",
        build_file = Label("//remote/regression_test/cargo/remote:winapi-0.3.7.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/regression_test/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "regression_test__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/regression_test/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD.bazel"),
    )
