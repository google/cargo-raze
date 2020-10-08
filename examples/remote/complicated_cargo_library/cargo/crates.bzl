"""
@generated
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def remote_complicated_cargo_library_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__MacTypes_sys__1_3_0",
        url = "https://crates.io/api/v1/crates/MacTypes-sys/1.3.0/download",
        type = "tar.gz",
        sha256 = "7dbbe033994ae2198a18517c7132d952a29fb1db44249a1234779da7c50f4698",
        strip_prefix = "MacTypes-sys-1.3.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:MacTypes-sys-1.3.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__aho_corasick__0_6_10",
        url = "https://crates.io/api/v1/crates/aho-corasick/0.6.10/download",
        type = "tar.gz",
        sha256 = "81ce3d38065e618af2d7b77e10c5ad9a069859b4be3c2250f674af3840d9c8a5",
        strip_prefix = "aho-corasick-0.6.10",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:aho-corasick-0.6.10.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__arrayvec__0_3_25",
        url = "https://crates.io/api/v1/crates/arrayvec/0.3.25/download",
        type = "tar.gz",
        sha256 = "06f59fe10306bb78facd90d28c2038ad23ffaaefa85bac43c8a434cde383334f",
        strip_prefix = "arrayvec-0.3.25",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:arrayvec-0.3.25.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__atom__0_3_5",
        url = "https://crates.io/api/v1/crates/atom/0.3.5/download",
        type = "tar.gz",
        sha256 = "3c86699c3f02778ec07158376991c8f783dd1f2f95c579ffaf0738dc984b2fe2",
        strip_prefix = "atom-0.3.5",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:atom-0.3.5.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__autocfg__1_0_1",
        url = "https://crates.io/api/v1/crates/autocfg/1.0.1/download",
        type = "tar.gz",
        sha256 = "cdb031dd78e28731d87d56cc8ffef4a8f36ca26c38fe2de700543e627f8a464a",
        strip_prefix = "autocfg-1.0.1",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:autocfg-1.0.1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__cc__1_0_60",
        url = "https://crates.io/api/v1/crates/cc/1.0.60/download",
        type = "tar.gz",
        sha256 = "ef611cc68ff783f18535d77ddd080185275713d852c4f5cbb6122c462a7a825c",
        strip_prefix = "cc-1.0.60",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:cc-1.0.60.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__cfg_if__0_1_10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:cfg-if-0.1.10.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__core_foundation_sys__0_5_1",
        url = "https://crates.io/api/v1/crates/core-foundation-sys/0.5.1/download",
        type = "tar.gz",
        sha256 = "716c271e8613ace48344f723b60b900a93150271e5be206212d052bbc0883efa",
        strip_prefix = "core-foundation-sys-0.5.1",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:core-foundation-sys-0.5.1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__crossbeam__0_3_2",
        url = "https://crates.io/api/v1/crates/crossbeam/0.3.2/download",
        type = "tar.gz",
        sha256 = "24ce9782d4d5c53674646a6a4c1863a21a8fc0cb649b3c94dfc16e45071dea19",
        strip_prefix = "crossbeam-0.3.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:crossbeam-0.3.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__crossbeam_channel__0_4_4",
        url = "https://crates.io/api/v1/crates/crossbeam-channel/0.4.4/download",
        type = "tar.gz",
        sha256 = "b153fe7cbef478c567df0f972e02e6d736db11affe43dfc9c56a9374d1adfb87",
        strip_prefix = "crossbeam-channel-0.4.4",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:crossbeam-channel-0.4.4.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__crossbeam_deque__0_7_3",
        url = "https://crates.io/api/v1/crates/crossbeam-deque/0.7.3/download",
        type = "tar.gz",
        sha256 = "9f02af974daeee82218205558e51ec8768b48cf524bd01d550abe5573a608285",
        strip_prefix = "crossbeam-deque-0.7.3",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:crossbeam-deque-0.7.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__crossbeam_epoch__0_8_2",
        url = "https://crates.io/api/v1/crates/crossbeam-epoch/0.8.2/download",
        type = "tar.gz",
        sha256 = "058ed274caafc1f60c4997b5fc07bf7dc7cca454af7c6e81edffe5f33f70dace",
        strip_prefix = "crossbeam-epoch-0.8.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:crossbeam-epoch-0.8.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__crossbeam_utils__0_7_2",
        url = "https://crates.io/api/v1/crates/crossbeam-utils/0.7.2/download",
        type = "tar.gz",
        sha256 = "c3c7c73a2d1e9fc0886a08b93e98eb643461230d5f1925e4036204d5f2e261a8",
        strip_prefix = "crossbeam-utils-0.7.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:crossbeam-utils-0.7.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__derivative__1_0_4",
        url = "https://crates.io/api/v1/crates/derivative/1.0.4/download",
        type = "tar.gz",
        sha256 = "3c6d883546668a3e2011b6a716a7330b82eabb0151b138217f632c8243e17135",
        strip_prefix = "derivative-1.0.4",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:derivative-1.0.4.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__fnv__1_0_7",
        url = "https://crates.io/api/v1/crates/fnv/1.0.7/download",
        type = "tar.gz",
        sha256 = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1",
        strip_prefix = "fnv-1.0.7",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:fnv-1.0.7.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__hermit_abi__0_1_15",
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.15/download",
        type = "tar.gz",
        sha256 = "3deed196b6e7f9e44a2ae8d94225d80302d81208b1bb673fd21fe634645c85a9",
        strip_prefix = "hermit-abi-0.1.15",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:hermit-abi-0.1.15.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__hibitset__0_3_2",
        url = "https://crates.io/api/v1/crates/hibitset/0.3.2/download",
        type = "tar.gz",
        sha256 = "b78998e3c243d71525596e8f373dfc4b82703f25907b9e4d260383cff8307d84",
        strip_prefix = "hibitset-0.3.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:hibitset-0.3.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:lazy_static-1.4.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__libc__0_2_77",
        url = "https://crates.io/api/v1/crates/libc/0.2.77/download",
        type = "tar.gz",
        sha256 = "f2f96b10ec2560088a8e76961b00d47107b3a625fecb76dedb29ee7ccbf98235",
        strip_prefix = "libc-0.2.77",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:libc-0.2.77.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__libloading__0_5_2",
        url = "https://crates.io/api/v1/crates/libloading/0.5.2/download",
        type = "tar.gz",
        sha256 = "f2b111a074963af1d37a139918ac6d49ad1d0d5e47f72fd55388619691a7d753",
        strip_prefix = "libloading-0.5.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:libloading-0.5.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__maybe_uninit__2_0_0",
        url = "https://crates.io/api/v1/crates/maybe-uninit/2.0.0/download",
        type = "tar.gz",
        sha256 = "60302e4db3a61da70c0cb7991976248362f30319e88850c487b9b95bbf059e00",
        strip_prefix = "maybe-uninit-2.0.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:maybe-uninit-2.0.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__memchr__2_3_3",
        url = "https://crates.io/api/v1/crates/memchr/2.3.3/download",
        type = "tar.gz",
        sha256 = "3728d817d99e5ac407411fa471ff9800a778d88a24685968b36824eaf4bee400",
        strip_prefix = "memchr-2.3.3",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:memchr-2.3.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__memoffset__0_5_5",
        url = "https://crates.io/api/v1/crates/memoffset/0.5.5/download",
        type = "tar.gz",
        sha256 = "c198b026e1bbf08a937e94c6c60f9ec4a2267f5b0d2eec9c1b21b061ce2be55f",
        strip_prefix = "memoffset-0.5.5",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:memoffset-0.5.5.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__mopa__0_2_2",
        url = "https://crates.io/api/v1/crates/mopa/0.2.2/download",
        type = "tar.gz",
        sha256 = "a785740271256c230f57462d3b83e52f998433a7062fc18f96d5999474a9f915",
        strip_prefix = "mopa-0.2.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:mopa-0.2.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__nodrop__0_1_14",
        url = "https://crates.io/api/v1/crates/nodrop/0.1.14/download",
        type = "tar.gz",
        sha256 = "72ef4a56884ca558e5ddb05a1d1e7e1bfd9a68d9ed024c21704cc98872dae1bb",
        strip_prefix = "nodrop-0.1.14",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:nodrop-0.1.14.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__num_cpus__1_13_0",
        url = "https://crates.io/api/v1/crates/num_cpus/1.13.0/download",
        type = "tar.gz",
        sha256 = "05499f3756671c15885fee9034446956fff3f243d6077b91e5767df161f766b3",
        strip_prefix = "num_cpus-1.13.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:num_cpus-1.13.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__odds__0_2_26",
        url = "https://crates.io/api/v1/crates/odds/0.2.26/download",
        type = "tar.gz",
        sha256 = "4eae0151b9dacf24fcc170d9995e511669a082856a91f958a2fe380bfab3fb22",
        strip_prefix = "odds-0.2.26",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:odds-0.2.26.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__proc_macro2__0_4_30",
        url = "https://crates.io/api/v1/crates/proc-macro2/0.4.30/download",
        type = "tar.gz",
        sha256 = "cf3d2011ab5c909338f7887f4fc896d35932e29146c12c8d01da6b22a80ba759",
        strip_prefix = "proc-macro2-0.4.30",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:proc-macro2-0.4.30.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__pulse__0_5_3",
        url = "https://crates.io/api/v1/crates/pulse/0.5.3/download",
        type = "tar.gz",
        sha256 = "655612b6c8d96a8a02f331fe296cb4f925b68e87c1d195544675abca2d9b9af0",
        strip_prefix = "pulse-0.5.3",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:pulse-0.5.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__quote__0_3_15",
        url = "https://crates.io/api/v1/crates/quote/0.3.15/download",
        type = "tar.gz",
        sha256 = "7a6e920b65c65f10b2ae65c831a81a073a89edd28c7cce89475bff467ab4167a",
        strip_prefix = "quote-0.3.15",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:quote-0.3.15.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__quote__0_6_13",
        url = "https://crates.io/api/v1/crates/quote/0.6.13/download",
        type = "tar.gz",
        sha256 = "6ce23b6b870e8f94f81fb0a363d65d86675884b34a09043c81e5562f11c1f8e1",
        strip_prefix = "quote-0.6.13",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:quote-0.6.13.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__rayon__0_8_2",
        url = "https://crates.io/api/v1/crates/rayon/0.8.2/download",
        type = "tar.gz",
        sha256 = "b614fe08b6665cb9a231d07ac1364b0ef3cb3698f1239ee0c4c3a88a524f54c8",
        strip_prefix = "rayon-0.8.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:rayon-0.8.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__rayon_core__1_8_1",
        url = "https://crates.io/api/v1/crates/rayon-core/1.8.1/download",
        type = "tar.gz",
        sha256 = "e8c4fec834fb6e6d2dd5eece3c7b432a52f0ba887cf40e595190c4107edc08bf",
        strip_prefix = "rayon-core-1.8.1",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:rayon-core-1.8.1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__regex__0_2_5",
        url = "https://crates.io/api/v1/crates/regex/0.2.5/download",
        type = "tar.gz",
        sha256 = "744554e01ccbd98fff8c457c3b092cd67af62a555a43bfe97ae8a0451f7799fa",
        strip_prefix = "regex-0.2.5",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:regex-0.2.5.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__regex_syntax__0_4_2",
        url = "https://crates.io/api/v1/crates/regex-syntax/0.4.2/download",
        type = "tar.gz",
        sha256 = "8e931c58b93d86f080c734bfd2bce7dd0079ae2331235818133c8be7f422e20e",
        strip_prefix = "regex-syntax-0.4.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:regex-syntax-0.4.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__scopeguard__1_1_0",
        url = "https://crates.io/api/v1/crates/scopeguard/1.1.0/download",
        type = "tar.gz",
        sha256 = "d29ab0c6d3fc0ee92fe66e2d99f700eab17a8d57d1c1d3b748380fb20baa78cd",
        strip_prefix = "scopeguard-1.1.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:scopeguard-1.1.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__security_framework_sys__0_2_2",
        url = "https://crates.io/api/v1/crates/security-framework-sys/0.2.2/download",
        type = "tar.gz",
        sha256 = "40d95f3d7da09612affe897f320d78264f0d2320f3e8eea27d12bd1bd94445e2",
        strip_prefix = "security-framework-sys-0.2.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:security-framework-sys-0.2.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__shred__0_5_2",
        url = "https://crates.io/api/v1/crates/shred/0.5.2/download",
        type = "tar.gz",
        sha256 = "7d3abceaa9d0a9b47ab84b53c6029c21bcad7d7dd63e14db51ea0680faee2159",
        strip_prefix = "shred-0.5.2",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:shred-0.5.2.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__shred_derive__0_3_0",
        url = "https://crates.io/api/v1/crates/shred-derive/0.3.0/download",
        type = "tar.gz",
        sha256 = "a4a894913b6e93fe2cd712a3bc955ec6f6b01c675c1c58b02fdfa13f77868049",
        strip_prefix = "shred-derive-0.3.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:shred-derive-0.3.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__smallvec__0_4_5",
        url = "https://crates.io/api/v1/crates/smallvec/0.4.5/download",
        type = "tar.gz",
        sha256 = "f90c5e5fe535e48807ab94fc611d323935f39d4660c52b26b96446a7b33aef10",
        strip_prefix = "smallvec-0.4.5",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:smallvec-0.4.5.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__specs__0_10_0",
        url = "https://crates.io/api/v1/crates/specs/0.10.0/download",
        type = "tar.gz",
        sha256 = "a210dc96ea065cb88391aa6956ed1b2a14051c668b5bc18bac66a95c215b639f",
        strip_prefix = "specs-0.10.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:specs-0.10.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__syn__0_11_11",
        url = "https://crates.io/api/v1/crates/syn/0.11.11/download",
        type = "tar.gz",
        sha256 = "d3b891b9015c88c576343b9b3e41c2c11a51c219ef067b264bd9c8aa9b441dad",
        strip_prefix = "syn-0.11.11",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:syn-0.11.11.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__syn__0_15_44",
        url = "https://crates.io/api/v1/crates/syn/0.15.44/download",
        type = "tar.gz",
        sha256 = "9ca4b3b69a77cbe1ffc9e198781b7acb0c7365a883670e8f1c1bc66fba79a5c5",
        strip_prefix = "syn-0.15.44",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:syn-0.15.44.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__synom__0_11_3",
        url = "https://crates.io/api/v1/crates/synom/0.11.3/download",
        type = "tar.gz",
        sha256 = "a393066ed9010ebaed60b9eafa373d4b1baac186dd7e008555b0f702b51945b6",
        strip_prefix = "synom-0.11.3",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:synom-0.11.3.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__thread_local__0_3_6",
        url = "https://crates.io/api/v1/crates/thread_local/0.3.6/download",
        type = "tar.gz",
        sha256 = "c6b53e329000edc2b34dbe8545fd20e55a333362d0a321909685a19bd28c3f1b",
        strip_prefix = "thread_local-0.3.6",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:thread_local-0.3.6.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__time__0_1_44",
        url = "https://crates.io/api/v1/crates/time/0.1.44/download",
        type = "tar.gz",
        sha256 = "6db9e6914ab8b1ae1c260a4ae7a49b6c5611b40328a735b21862567685e73255",
        strip_prefix = "time-0.1.44",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:time-0.1.44.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__tuple_utils__0_2_0",
        url = "https://crates.io/api/v1/crates/tuple_utils/0.2.0/download",
        type = "tar.gz",
        sha256 = "cbfecd7bb8f0a3e96b3b31c46af2677a55a588767c0091f484601424fcb20e7e",
        strip_prefix = "tuple_utils-0.2.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:tuple_utils-0.2.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__unicode_xid__0_0_4",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.0.4/download",
        type = "tar.gz",
        sha256 = "8c1f860d7d29cf02cb2f3f359fd35991af3d30bac52c57d265a3c461074cb4dc",
        strip_prefix = "unicode-xid-0.0.4",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:unicode-xid-0.0.4.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__unicode_xid__0_1_0",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.1.0/download",
        type = "tar.gz",
        sha256 = "fc72304796d0818e357ead4e000d19c9c174ab23dc11093ac919054d20a6a7fc",
        strip_prefix = "unicode-xid-0.1.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:unicode-xid-0.1.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__utf8_ranges__1_0_4",
        url = "https://crates.io/api/v1/crates/utf8-ranges/1.0.4/download",
        type = "tar.gz",
        sha256 = "b4ae116fef2b7fea257ed6440d3cfcff7f190865f170cdad00bb6465bf18ecba",
        strip_prefix = "utf8-ranges-1.0.4",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:utf8-ranges-1.0.4.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__wasi__0_10_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.10.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "1a143597ca7c7793eff794def352d41792a93c481eb1042423ff7ff72ba2c31f",
        strip_prefix = "wasi-0.10.0+wasi-snapshot-preview1",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:wasi-0.10.0+wasi-snapshot-preview1.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__winapi__0_3_9",
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:winapi-0.3.9.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_complicated_cargo_library__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/complicated_cargo_library/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD.bazel"),
    )
