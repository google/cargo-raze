"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def remote_binary_dependencies_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "remote_binary_dependencies__addr2line__0_13_0",
        url = "https://crates.io/api/v1/crates/addr2line/0.13.0/download",
        type = "tar.gz",
        sha256 = "1b6a2d3371669ab3ca9797670853d61402b03d0b4b9ebf33d677dfa720203072",
        strip_prefix = "addr2line-0.13.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.addr2line-0.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__adler__0_2_3",
        url = "https://crates.io/api/v1/crates/adler/0.2.3/download",
        type = "tar.gz",
        sha256 = "ee2a4ec343196209d6594e19543ae87a39f96d5534d7174822a3ad825dd6ed7e",
        strip_prefix = "adler-0.2.3",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.adler-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__ansi_term__0_11_0",
        url = "https://crates.io/api/v1/crates/ansi_term/0.11.0/download",
        type = "tar.gz",
        sha256 = "ee49baf6cb617b853aa8d93bf420db2383fab46d314482ca2803b40d5fde979b",
        strip_prefix = "ansi_term-0.11.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.ansi_term-0.11.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__atty__0_2_14",
        url = "https://crates.io/api/v1/crates/atty/0.2.14/download",
        type = "tar.gz",
        sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8",
        strip_prefix = "atty-0.2.14",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.atty-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__autocfg__1_0_1",
        url = "https://crates.io/api/v1/crates/autocfg/1.0.1/download",
        type = "tar.gz",
        sha256 = "cdb031dd78e28731d87d56cc8ffef4a8f36ca26c38fe2de700543e627f8a464a",
        strip_prefix = "autocfg-1.0.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.autocfg-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__backtrace__0_3_50",
        url = "https://crates.io/api/v1/crates/backtrace/0.3.50/download",
        type = "tar.gz",
        sha256 = "46254cf2fdcdf1badb5934448c1bcbe046a56537b3987d96c51a7afc5d03f293",
        strip_prefix = "backtrace-0.3.50",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.backtrace-0.3.50.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__bitflags__1_2_1",
        url = "https://crates.io/api/v1/crates/bitflags/1.2.1/download",
        type = "tar.gz",
        sha256 = "cf1de2fe8c75bc145a2f577add951f8134889b4795d47466a54a5c846d691693",
        strip_prefix = "bitflags-1.2.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.bitflags-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__cfg_if__0_1_10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.cfg-if-0.1.10.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__clap__2_33_3",
        url = "https://crates.io/api/v1/crates/clap/2.33.3/download",
        type = "tar.gz",
        sha256 = "37e58ac78573c40708d45522f0d80fa2f01cc4f9b4e2bf749807255454312002",
        strip_prefix = "clap-2.33.3",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.clap-2.33.3.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__error_chain__0_10_0",
        url = "https://crates.io/api/v1/crates/error-chain/0.10.0/download",
        type = "tar.gz",
        sha256 = "d9435d864e017c3c6afeac1654189b06cdb491cf2ff73dbf0d73b0f292f42ff8",
        strip_prefix = "error-chain-0.10.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.error-chain-0.10.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__ferris_says__0_2_0",
        url = "https://crates.io/api/v1/crates/ferris-says/0.2.0/download",
        type = "tar.gz",
        sha256 = "7f34f82e9a8b1533c027d018abd90b8687bf923be287b2617dfce4bea4ea3687",
        strip_prefix = "ferris-says-0.2.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.ferris-says-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__gimli__0_22_0",
        url = "https://crates.io/api/v1/crates/gimli/0.22.0/download",
        type = "tar.gz",
        sha256 = "aaf91faf136cb47367fa430cd46e37a788775e7fa104f8b4bcb3861dc389b724",
        strip_prefix = "gimli-0.22.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.gimli-0.22.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__hermit_abi__0_1_15",
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.15/download",
        type = "tar.gz",
        sha256 = "3deed196b6e7f9e44a2ae8d94225d80302d81208b1bb673fd21fe634645c85a9",
        strip_prefix = "hermit-abi-0.1.15",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.hermit-abi-0.1.15.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__libc__0_2_77",
        url = "https://crates.io/api/v1/crates/libc/0.2.77/download",
        type = "tar.gz",
        sha256 = "f2f96b10ec2560088a8e76961b00d47107b3a625fecb76dedb29ee7ccbf98235",
        strip_prefix = "libc-0.2.77",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.libc-0.2.77.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__miniz_oxide__0_4_2",
        url = "https://crates.io/api/v1/crates/miniz_oxide/0.4.2/download",
        type = "tar.gz",
        sha256 = "c60c0dfe32c10b43a144bad8fc83538c52f58302c92300ea7ec7bf7b38d5a7b9",
        strip_prefix = "miniz_oxide-0.4.2",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.miniz_oxide-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__object__0_20_0",
        url = "https://crates.io/api/v1/crates/object/0.20.0/download",
        type = "tar.gz",
        sha256 = "1ab52be62400ca80aa00285d25253d7f7c437b7375c4de678f5405d3afe82ca5",
        strip_prefix = "object-0.20.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.object-0.20.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rustc_demangle__0_1_16",
        url = "https://crates.io/api/v1/crates/rustc-demangle/0.1.16/download",
        type = "tar.gz",
        sha256 = "4c691c0e608126e00913e33f0ccf3727d5fc84573623b8d65b2df340b5201783",
        strip_prefix = "rustc-demangle-0.1.16",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rustc-demangle-0.1.16.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__smallvec__0_4_5",
        url = "https://crates.io/api/v1/crates/smallvec/0.4.5/download",
        type = "tar.gz",
        sha256 = "f90c5e5fe535e48807ab94fc611d323935f39d4660c52b26b96446a7b33aef10",
        strip_prefix = "smallvec-0.4.5",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.smallvec-0.4.5.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__strsim__0_8_0",
        url = "https://crates.io/api/v1/crates/strsim/0.8.0/download",
        type = "tar.gz",
        sha256 = "8ea5119cdb4c55b55d432abb513a0429384878c15dde60cc77b1c99de1a95a6a",
        strip_prefix = "strsim-0.8.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.strsim-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__textwrap__0_11_0",
        url = "https://crates.io/api/v1/crates/textwrap/0.11.0/download",
        type = "tar.gz",
        sha256 = "d326610f408c7a4eb6f51c37c330e496b08506c9457c9d34287ecc38809fb060",
        strip_prefix = "textwrap-0.11.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.textwrap-0.11.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__unicode_width__0_1_8",
        url = "https://crates.io/api/v1/crates/unicode-width/0.1.8/download",
        type = "tar.gz",
        sha256 = "9337591893a19b88d8d87f2cec1e73fad5cdfd10e5a6f349f498ad6ea2ffb1e3",
        strip_prefix = "unicode-width-0.1.8",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.unicode-width-0.1.8.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__vec_map__0_8_2",
        url = "https://crates.io/api/v1/crates/vec_map/0.8.2/download",
        type = "tar.gz",
        sha256 = "f1bddf1187be692e79c5ffeab891132dfb0f236ed36a43c7ed39f1165ee20191",
        strip_prefix = "vec_map-0.8.2",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.vec_map-0.8.2.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__winapi__0_3_9",
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.winapi-0.3.9.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.winapi-i686-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.winapi-x86_64-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__adler32__1_0_4",
        url = "https://crates.io/api/v1/crates/adler32/1.0.4/download",
        type = "tar.gz",
        sha256 = "5d2e7343e7fc9de883d1b0341e0b13970f764c14101234857d2ddafa1cb1cac2",
        strip_prefix = "adler32-1.0.4",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.adler32-1.0.4.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__ansi_term__0_11_0",
        url = "https://crates.io/api/v1/crates/ansi_term/0.11.0/download",
        type = "tar.gz",
        sha256 = "ee49baf6cb617b853aa8d93bf420db2383fab46d314482ca2803b40d5fde979b",
        strip_prefix = "ansi_term-0.11.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.ansi_term-0.11.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__atty__0_2_14",
        url = "https://crates.io/api/v1/crates/atty/0.2.14/download",
        type = "tar.gz",
        sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8",
        strip_prefix = "atty-0.2.14",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.atty-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__autocfg__1_0_0",
        url = "https://crates.io/api/v1/crates/autocfg/1.0.0/download",
        type = "tar.gz",
        sha256 = "f8aac770f1885fd7e387acedd76065302551364496e46b3dd00860b2f8359b9d",
        strip_prefix = "autocfg-1.0.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.autocfg-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__bitflags__1_2_1",
        url = "https://crates.io/api/v1/crates/bitflags/1.2.1/download",
        type = "tar.gz",
        sha256 = "cf1de2fe8c75bc145a2f577add951f8134889b4795d47466a54a5c846d691693",
        strip_prefix = "bitflags-1.2.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.bitflags-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__bytemuck__1_2_0",
        url = "https://crates.io/api/v1/crates/bytemuck/1.2.0/download",
        type = "tar.gz",
        sha256 = "37fa13df2292ecb479ec23aa06f4507928bef07839be9ef15281411076629431",
        strip_prefix = "bytemuck-1.2.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.bytemuck-1.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__byteorder__1_3_4",
        url = "https://crates.io/api/v1/crates/byteorder/1.3.4/download",
        type = "tar.gz",
        sha256 = "08c48aae112d48ed9f069b33538ea9e3e90aa263cfa3d1c24309612b1f7472de",
        strip_prefix = "byteorder-1.3.4",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.byteorder-1.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__c2_chacha__0_2_3",
        url = "https://crates.io/api/v1/crates/c2-chacha/0.2.3/download",
        type = "tar.gz",
        sha256 = "214238caa1bf3a496ec3392968969cab8549f96ff30652c9e56885329315f6bb",
        strip_prefix = "c2-chacha-0.2.3",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.c2-chacha-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__cfg_if__0_1_10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.cfg-if-0.1.10.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__clap__2_33_0",
        url = "https://crates.io/api/v1/crates/clap/2.33.0/download",
        type = "tar.gz",
        sha256 = "5067f5bb2d80ef5d68b4c87db81601f0b75bca627bc2ef76b141d7b846a3c6d9",
        strip_prefix = "clap-2.33.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.clap-2.33.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__clicolors_control__1_0_1",
        url = "https://crates.io/api/v1/crates/clicolors-control/1.0.1/download",
        type = "tar.gz",
        sha256 = "90082ee5dcdd64dc4e9e0d37fbf3ee325419e39c0092191e0393df65518f741e",
        strip_prefix = "clicolors-control-1.0.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.clicolors-control-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__console__0_9_2",
        url = "https://crates.io/api/v1/crates/console/0.9.2/download",
        type = "tar.gz",
        sha256 = "45e0f3986890b3acbc782009e2629dfe2baa430ac091519ce3be26164a2ae6c0",
        strip_prefix = "console-0.9.2",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.console-0.9.2.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__crc32fast__1_2_0",
        url = "https://crates.io/api/v1/crates/crc32fast/1.2.0/download",
        type = "tar.gz",
        sha256 = "ba125de2af0df55319f41944744ad91c71113bf74a4646efff39afe1f6842db1",
        strip_prefix = "crc32fast-1.2.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.crc32fast-1.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__crossbeam_utils__0_7_2",
        url = "https://crates.io/api/v1/crates/crossbeam-utils/0.7.2/download",
        type = "tar.gz",
        sha256 = "c3c7c73a2d1e9fc0886a08b93e98eb643461230d5f1925e4036204d5f2e261a8",
        strip_prefix = "crossbeam-utils-0.7.2",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.crossbeam-utils-0.7.2.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__deflate__0_7_20",
        url = "https://crates.io/api/v1/crates/deflate/0.7.20/download",
        type = "tar.gz",
        sha256 = "707b6a7b384888a70c8d2e8650b3e60170dfc6a67bb4aa67b6dfca57af4bedb4",
        strip_prefix = "deflate-0.7.20",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.deflate-0.7.20.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__encode_unicode__0_3_6",
        url = "https://crates.io/api/v1/crates/encode_unicode/0.3.6/download",
        type = "tar.gz",
        sha256 = "a357d28ed41a50f9c765dbfe56cbc04a64e53e5fc58ba79fbc34c10ef3df831f",
        strip_prefix = "encode_unicode-0.3.6",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.encode_unicode-0.3.6.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__heck__0_3_1",
        url = "https://crates.io/api/v1/crates/heck/0.3.1/download",
        type = "tar.gz",
        sha256 = "20564e78d53d2bb135c343b3f47714a56af2061f1c928fdb541dc7b9fdd94205",
        strip_prefix = "heck-0.3.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.heck-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__hermit_abi__0_1_8",
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.8/download",
        type = "tar.gz",
        sha256 = "1010591b26bbfe835e9faeabeb11866061cc7dcebffd56ad7d0942d0e61aefd8",
        strip_prefix = "hermit-abi-0.1.8",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.hermit-abi-0.1.8.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__image__0_23_0",
        url = "https://crates.io/api/v1/crates/image/0.23.0/download",
        type = "tar.gz",
        sha256 = "ef4e336ec01a678e7ab692914c641181528e8656451e6252f8f9e33728882eaf",
        strip_prefix = "image-0.23.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.image-0.23.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__indicatif__0_14_0",
        url = "https://crates.io/api/v1/crates/indicatif/0.14.0/download",
        type = "tar.gz",
        sha256 = "49a68371cf417889c9d7f98235b7102ea7c54fc59bcbd22f3dea785be9d27e40",
        strip_prefix = "indicatif-0.14.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.indicatif-0.14.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__inflate__0_4_5",
        url = "https://crates.io/api/v1/crates/inflate/0.4.5/download",
        type = "tar.gz",
        sha256 = "1cdb29978cc5797bd8dcc8e5bf7de604891df2a8dc576973d71a281e916db2ff",
        strip_prefix = "inflate-0.4.5",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.inflate-0.4.5.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__jpeg_decoder__0_1_18",
        url = "https://crates.io/api/v1/crates/jpeg-decoder/0.1.18/download",
        type = "tar.gz",
        sha256 = "0256f0aec7352539102a9efbcb75543227b7ab1117e0f95450023af730128451",
        strip_prefix = "jpeg-decoder-0.1.18",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.jpeg-decoder-0.1.18.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.lazy_static-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__libc__0_2_67",
        url = "https://crates.io/api/v1/crates/libc/0.2.67/download",
        type = "tar.gz",
        sha256 = "eb147597cdf94ed43ab7a9038716637d2d1bf2bc571da995d0028dec06bd3018",
        strip_prefix = "libc-0.2.67",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.libc-0.2.67.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__num_integer__0_1_42",
        url = "https://crates.io/api/v1/crates/num-integer/0.1.42/download",
        type = "tar.gz",
        sha256 = "3f6ea62e9d81a77cd3ee9a2a5b9b609447857f3d358704331e4ef39eb247fcba",
        strip_prefix = "num-integer-0.1.42",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.num-integer-0.1.42.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__num_iter__0_1_40",
        url = "https://crates.io/api/v1/crates/num-iter/0.1.40/download",
        type = "tar.gz",
        sha256 = "dfb0800a0291891dd9f4fe7bd9c19384f98f7fbe0cd0f39a2c6b88b9868bbc00",
        strip_prefix = "num-iter-0.1.40",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.num-iter-0.1.40.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__num_rational__0_2_3",
        url = "https://crates.io/api/v1/crates/num-rational/0.2.3/download",
        type = "tar.gz",
        sha256 = "da4dc79f9e6c81bef96148c8f6b8e72ad4541caa4a24373e900a36da07de03a3",
        strip_prefix = "num-rational-0.2.3",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.num-rational-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__num_traits__0_2_11",
        url = "https://crates.io/api/v1/crates/num-traits/0.2.11/download",
        type = "tar.gz",
        sha256 = "c62be47e61d1842b9170f0fdeec8eba98e60e90e5446449a0545e5152acd7096",
        strip_prefix = "num-traits-0.2.11",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.num-traits-0.2.11.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__num_cpus__1_12_0",
        url = "https://crates.io/api/v1/crates/num_cpus/1.12.0/download",
        type = "tar.gz",
        sha256 = "46203554f085ff89c235cd12f7075f3233af9b11ed7c9e16dfe2560d03313ce6",
        strip_prefix = "num_cpus-1.12.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.num_cpus-1.12.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__number_prefix__0_3_0",
        url = "https://crates.io/api/v1/crates/number_prefix/0.3.0/download",
        type = "tar.gz",
        sha256 = "17b02fc0ff9a9e4b35b3342880f48e896ebf69f2967921fe8646bf5b7125956a",
        strip_prefix = "number_prefix-0.3.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.number_prefix-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__pdqselect__0_1_0",
        url = "https://crates.io/api/v1/crates/pdqselect/0.1.0/download",
        type = "tar.gz",
        sha256 = "4ec91767ecc0a0bbe558ce8c9da33c068066c57ecc8bb8477ef8c1ad3ef77c27",
        strip_prefix = "pdqselect-0.1.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.pdqselect-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__png__0_15_3",
        url = "https://crates.io/api/v1/crates/png/0.15.3/download",
        type = "tar.gz",
        sha256 = "ef859a23054bbfee7811284275ae522f0434a3c8e7f4b74bd4a35ae7e1c4a283",
        strip_prefix = "png-0.15.3",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.png-0.15.3.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__ppv_lite86__0_2_6",
        url = "https://crates.io/api/v1/crates/ppv-lite86/0.2.6/download",
        type = "tar.gz",
        sha256 = "74490b50b9fbe561ac330df47c08f3f33073d2d00c150f719147d7c54522fa1b",
        strip_prefix = "ppv-lite86-0.2.6",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.ppv-lite86-0.2.6.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__proc_macro_error__0_4_9",
        url = "https://crates.io/api/v1/crates/proc-macro-error/0.4.9/download",
        type = "tar.gz",
        sha256 = "052b3c9af39c7e5e94245f820530487d19eb285faedcb40e0c3275132293f242",
        strip_prefix = "proc-macro-error-0.4.9",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.proc-macro-error-0.4.9.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__proc_macro_error_attr__0_4_9",
        url = "https://crates.io/api/v1/crates/proc-macro-error-attr/0.4.9/download",
        type = "tar.gz",
        sha256 = "d175bef481c7902e63e3165627123fff3502f06ac043d3ef42d08c1246da9253",
        strip_prefix = "proc-macro-error-attr-0.4.9",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.proc-macro-error-attr-0.4.9.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__proc_macro2__1_0_9",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.9/download",
        type = "tar.gz",
        sha256 = "6c09721c6781493a2a492a96b5a5bf19b65917fe6728884e7c44dd0c60ca3435",
        strip_prefix = "proc-macro2-1.0.9",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.proc-macro2-1.0.9.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__quote__1_0_2",
        url = "https://crates.io/api/v1/crates/quote/1.0.2/download",
        type = "tar.gz",
        sha256 = "053a8c8bcc71fcce321828dc897a98ab9760bef03a4fc36693c231e5b3216cfe",
        strip_prefix = "quote-1.0.2",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.quote-1.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rand__0_7_3",
        url = "https://crates.io/api/v1/crates/rand/0.7.3/download",
        type = "tar.gz",
        sha256 = "6a6b1679d49b24bbfe0c803429aa1874472f50d9b363131f0e89fc356b544d03",
        strip_prefix = "rand-0.7.3",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rand-0.7.3.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rand_chacha__0_2_1",
        url = "https://crates.io/api/v1/crates/rand_chacha/0.2.1/download",
        type = "tar.gz",
        sha256 = "03a2a90da8c7523f554344f921aa97283eadf6ac484a6d2a7d0212fa7f8d6853",
        strip_prefix = "rand_chacha-0.2.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rand_chacha-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rand_core__0_5_1",
        url = "https://crates.io/api/v1/crates/rand_core/0.5.1/download",
        type = "tar.gz",
        sha256 = "90bde5296fc891b0cef12a6d03ddccc162ce7b2aff54160af9338f8d40df6d19",
        strip_prefix = "rand_core-0.5.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rand_core-0.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rand_hc__0_2_0",
        url = "https://crates.io/api/v1/crates/rand_hc/0.2.0/download",
        type = "tar.gz",
        sha256 = "ca3129af7b92a17112d59ad498c6f81eaf463253766b90396d39ea7a39d6613c",
        strip_prefix = "rand_hc-0.2.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rand_hc-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rand_pcg__0_2_1",
        url = "https://crates.io/api/v1/crates/rand_pcg/0.2.1/download",
        type = "tar.gz",
        sha256 = "16abd0c1b639e9eb4d7c50c0b8100b0d0f849be2349829c740fe8e6eb4816429",
        strip_prefix = "rand_pcg-0.2.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rand_pcg-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__regex__1_3_4",
        url = "https://crates.io/api/v1/crates/regex/1.3.4/download",
        type = "tar.gz",
        sha256 = "322cf97724bea3ee221b78fe25ac9c46114ebb51747ad5babd51a2fc6a8235a8",
        strip_prefix = "regex-1.3.4",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.regex-1.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__regex_syntax__0_6_14",
        url = "https://crates.io/api/v1/crates/regex-syntax/0.6.14/download",
        type = "tar.gz",
        sha256 = "b28dfe3fe9badec5dbf0a79a9cccad2cfc2ab5484bdb3e44cbd1ae8b3ba2be06",
        strip_prefix = "regex-syntax-0.6.14",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.regex-syntax-0.6.14.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rstar__0_7_1",
        url = "https://crates.io/api/v1/crates/rstar/0.7.1/download",
        type = "tar.gz",
        sha256 = "0650eaaa56cbd1726fd671150fce8ac6ed9d9a25d1624430d7ee9d196052f6b6",
        strip_prefix = "rstar-0.7.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rstar-0.7.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__rustversion__1_0_2",
        url = "https://crates.io/api/v1/crates/rustversion/1.0.2/download",
        type = "tar.gz",
        sha256 = "b3bba175698996010c4f6dce5e7f173b6eb781fce25d2cfc45e27091ce0b79f6",
        strip_prefix = "rustversion-1.0.2",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.rustversion-1.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__strsim__0_8_0",
        url = "https://crates.io/api/v1/crates/strsim/0.8.0/download",
        type = "tar.gz",
        sha256 = "8ea5119cdb4c55b55d432abb513a0429384878c15dde60cc77b1c99de1a95a6a",
        strip_prefix = "strsim-0.8.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.strsim-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__structopt__0_3_9",
        url = "https://crates.io/api/v1/crates/structopt/0.3.9/download",
        type = "tar.gz",
        sha256 = "a1bcbed7d48956fcbb5d80c6b95aedb553513de0a1b451ea92679d999c010e98",
        strip_prefix = "structopt-0.3.9",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.structopt-0.3.9.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__structopt_derive__0_4_2",
        url = "https://crates.io/api/v1/crates/structopt-derive/0.4.2/download",
        type = "tar.gz",
        sha256 = "095064aa1f5b94d14e635d0a5684cf140c43ae40a0fd990708d38f5d669e5f64",
        strip_prefix = "structopt-derive-0.4.2",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.structopt-derive-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__syn__1_0_16",
        url = "https://crates.io/api/v1/crates/syn/1.0.16/download",
        type = "tar.gz",
        sha256 = "123bd9499cfb380418d509322d7a6d52e5315f064fe4b3ad18a53d6b92c07859",
        strip_prefix = "syn-1.0.16",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.syn-1.0.16.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__syn_mid__0_5_0",
        url = "https://crates.io/api/v1/crates/syn-mid/0.5.0/download",
        type = "tar.gz",
        sha256 = "7be3539f6c128a931cf19dcee741c1af532c7fd387baa739c03dd2e96479338a",
        strip_prefix = "syn-mid-0.5.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.syn-mid-0.5.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__termios__0_3_1",
        url = "https://crates.io/api/v1/crates/termios/0.3.1/download",
        type = "tar.gz",
        sha256 = "72b620c5ea021d75a735c943269bb07d30c9b77d6ac6b236bc8b5c496ef05625",
        strip_prefix = "termios-0.3.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.termios-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__texture_synthesis__0_8_0",
        url = "https://crates.io/api/v1/crates/texture-synthesis/0.8.0/download",
        type = "tar.gz",
        sha256 = "2ff7e9b61c0c11d66b78f7474d69217a4e0fc5b758ff33b67c6dc0b87b126191",
        strip_prefix = "texture-synthesis-0.8.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.texture-synthesis-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__texture_synthesis_cli__0_8_0",
        url = "https://crates.io/api/v1/crates/texture-synthesis-cli/0.8.0/download",
        type = "tar.gz",
        sha256 = "4d0a65298b735ba486081633e90469182d7c0ca59018fec1e81383bde852be2e",
        strip_prefix = "texture-synthesis-cli-0.8.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.texture-synthesis-cli-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__textwrap__0_11_0",
        url = "https://crates.io/api/v1/crates/textwrap/0.11.0/download",
        type = "tar.gz",
        sha256 = "d326610f408c7a4eb6f51c37c330e496b08506c9457c9d34287ecc38809fb060",
        strip_prefix = "textwrap-0.11.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.textwrap-0.11.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__unicode_segmentation__1_6_0",
        url = "https://crates.io/api/v1/crates/unicode-segmentation/1.6.0/download",
        type = "tar.gz",
        sha256 = "e83e153d1053cbb5a118eeff7fd5be06ed99153f00dbcd8ae310c5fb2b22edc0",
        strip_prefix = "unicode-segmentation-1.6.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.unicode-segmentation-1.6.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__unicode_width__0_1_7",
        url = "https://crates.io/api/v1/crates/unicode-width/0.1.7/download",
        type = "tar.gz",
        sha256 = "caaa9d531767d1ff2150b9332433f32a24622147e5ebb1f26409d5da67afd479",
        strip_prefix = "unicode-width-0.1.7",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.unicode-width-0.1.7.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__unicode_xid__0_2_0",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.0/download",
        type = "tar.gz",
        sha256 = "826e7639553986605ec5979c7dd957c7895e93eabed50ab2ffa7f6128a75097c",
        strip_prefix = "unicode-xid-0.2.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.unicode-xid-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__vec_map__0_8_1",
        url = "https://crates.io/api/v1/crates/vec_map/0.8.1/download",
        type = "tar.gz",
        sha256 = "05c78687fb1a80548ae3250346c3db86a80a7cdd77bda190189f2d0a0987c81a",
        strip_prefix = "vec_map-0.8.1",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.vec_map-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__winapi__0_3_8",
        url = "https://crates.io/api/v1/crates/winapi/0.3.8/download",
        type = "tar.gz",
        sha256 = "8093091eeb260906a183e6ae1abdba2ef5ef2257a21801128899c3fc699229c6",
        strip_prefix = "winapi-0.3.8",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.winapi-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.winapi-i686-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "remote_binary_dependencies__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/binary_dependencies/cargo/remote:BUILD.winapi-x86_64-pc-windows-gnu-0.4.0.bazel"),
    )
