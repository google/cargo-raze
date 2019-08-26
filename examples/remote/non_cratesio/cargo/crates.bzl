"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        new_git_repository(name=name, **kwargs)

def non_cratesio_fetch_remote_crates():

    _new_http_archive(
        name = "non_cratesio__aho_corasick__0_6_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.10.crate",
        type = "tar.gz",
        sha256 = "81ce3d38065e618af2d7b77e10c5ad9a069859b4be3c2250f674af3840d9c8a5",
        strip_prefix = "aho-corasick-0.6.10",
        build_file = Label("//remote/non_cratesio/cargo/remote:aho-corasick-0.6.10.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__atty__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.11.crate",
        type = "tar.gz",
        sha256 = "9a7d5b8723950951411ee34d271d99dddcc2035a16ab25310ea2c8cfd4369652",
        strip_prefix = "atty-0.2.11",
        build_file = Label("//remote/non_cratesio/cargo/remote:atty-0.2.11.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__bitflags__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.4.crate",
        type = "tar.gz",
        sha256 = "228047a76f468627ca71776ecdebd732a3423081fcf5125585bcd7c49886ce12",
        strip_prefix = "bitflags-1.0.4",
        build_file = Label("//remote/non_cratesio/cargo/remote:bitflags-1.0.4.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__cfg_if__0_1_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.7.crate",
        type = "tar.gz",
        sha256 = "11d43355396e872eefb45ce6342e4374ed7bc2b3a502d1b28e36d6e23c05d1f4",
        strip_prefix = "cfg-if-0.1.7",
        build_file = Label("//remote/non_cratesio/cargo/remote:cfg-if-0.1.7.BUILD")
    )

    _new_git_repository(
        name = "non_cratesio__env_logger__0_5_5",
        remote = "https://github.com/sebasmagri/env_logger.git",
        commit = "4d7926c515a8620e7fa0e91a07047023170c1064",
        build_file = Label("//remote/non_cratesio/cargo/remote:env_logger-0.5.5.BUILD"),
        init_submodules = True
    )

    _new_http_archive(
        name = "non_cratesio__fuchsia_zircon__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon/fuchsia-zircon-0.3.3.crate",
        type = "tar.gz",
        sha256 = "2e9763c69ebaae630ba35f74888db465e49e259ba1bc0eda7d06f4a067615d82",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = Label("//remote/non_cratesio/cargo/remote:fuchsia-zircon-0.3.3.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__fuchsia_zircon_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon-sys/fuchsia-zircon-sys-0.3.3.crate",
        type = "tar.gz",
        sha256 = "3dcaa9ae7725d12cdb85b3ad99a434db70b468c09ded17e012d86b5c1010f7a7",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = Label("//remote/non_cratesio/cargo/remote:fuchsia-zircon-sys-0.3.3.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__humantime__1_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/humantime/humantime-1.2.0.crate",
        type = "tar.gz",
        sha256 = "3ca7e5f2e110db35f93b837c81797f3714500b81d517bf20c431b16d3ca4f114",
        strip_prefix = "humantime-1.2.0",
        build_file = Label("//remote/non_cratesio/cargo/remote:humantime-1.2.0.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__lazy_static__1_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.3.0.crate",
        type = "tar.gz",
        sha256 = "bc5729f27f159ddd61f4df6228e827e86643d4d3e7c32183cb30a1c08f604a14",
        strip_prefix = "lazy_static-1.3.0",
        build_file = Label("//remote/non_cratesio/cargo/remote:lazy_static-1.3.0.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__libc__0_2_53",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.53.crate",
        type = "tar.gz",
        sha256 = "ec350a9417dfd244dc9a6c4a71e13895a4db6b92f0b106f07ebbc3f3bc580cee",
        strip_prefix = "libc-0.2.53",
        build_file = Label("//remote/non_cratesio/cargo/remote:libc-0.2.53.BUILD")
    )

    _new_git_repository(
        name = "non_cratesio__log__0_4_0",
        remote = "https://github.com/rust-lang-nursery/log.git",
        commit = "bf40d1f563cf3eef63233d935ce56f2198b381d3",
        build_file = Label("//remote/non_cratesio/cargo/remote:log-0.4.0.BUILD"),
        init_submodules = True
    )

    _new_http_archive(
        name = "non_cratesio__log__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.6.crate",
        type = "tar.gz",
        sha256 = "c84ec4b527950aa83a329754b01dbe3f58361d1c5efacd1f6d68c494d08a17c6",
        strip_prefix = "log-0.4.6",
        build_file = Label("//remote/non_cratesio/cargo/remote:log-0.4.6.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__memchr__2_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.2.0.crate",
        type = "tar.gz",
        sha256 = "2efc7bc57c883d4a4d6e3246905283d8dae951bb3bd32f49d6ef297f546e1c39",
        strip_prefix = "memchr-2.2.0",
        build_file = Label("//remote/non_cratesio/cargo/remote:memchr-2.2.0.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__numtoa__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/numtoa/numtoa-0.1.0.crate",
        type = "tar.gz",
        sha256 = "b8f8bdf33df195859076e54ab11ee78a1b208382d3a26ec40d142ffc1ecc49ef",
        strip_prefix = "numtoa-0.1.0",
        build_file = Label("//remote/non_cratesio/cargo/remote:numtoa-0.1.0.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__quick_error__1_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quick-error/quick-error-1.2.2.crate",
        type = "tar.gz",
        sha256 = "9274b940887ce9addde99c4eee6b5c44cc494b182b97e73dc8ffdcb3397fd3f0",
        strip_prefix = "quick-error-1.2.2",
        build_file = Label("//remote/non_cratesio/cargo/remote:quick-error-1.2.2.BUILD")
    )

    _new_git_repository(
        name = "non_cratesio__rand__0_4_1",
        remote = "https://github.com/rust-random/rand.git",
        commit = "b333d687138b5cb9b225f49ba80bf1bfef01e068",
        build_file = Label("//remote/non_cratesio/cargo/remote:rand-0.4.1.BUILD"),
        init_submodules = True
    )

    _new_http_archive(
        name = "non_cratesio__redox_syscall__0_1_54",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.54.crate",
        type = "tar.gz",
        sha256 = "12229c14a0f65c4f1cb046a3b52047cdd9da1f4b30f8a39c5063c8bae515e252",
        strip_prefix = "redox_syscall-0.1.54",
        build_file = Label("//remote/non_cratesio/cargo/remote:redox_syscall-0.1.54.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        sha256 = "7e891cfe48e9100a70a3b6eb652fef28920c117d366339687bd5576160db0f76",
        strip_prefix = "redox_termios-0.1.1",
        build_file = Label("//remote/non_cratesio/cargo/remote:redox_termios-0.1.1.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__regex__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.11.crate",
        type = "tar.gz",
        sha256 = "9329abc99e39129fcceabd24cf5d85b4671ef7c29c50e972bc5afe32438ec384",
        strip_prefix = "regex-0.2.11",
        build_file = Label("//remote/non_cratesio/cargo/remote:regex-0.2.11.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__regex_syntax__0_5_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.5.6.crate",
        type = "tar.gz",
        sha256 = "7d707a4fa2637f2dca2ef9fd02225ec7661fe01a53623c1e6515b6916511f7a7",
        strip_prefix = "regex-syntax-0.5.6",
        build_file = Label("//remote/non_cratesio/cargo/remote:regex-syntax-0.5.6.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__termcolor__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-0.3.6.crate",
        type = "tar.gz",
        sha256 = "adc4587ead41bf016f11af03e55a624c06568b5a19db4e90fde573d805074f83",
        strip_prefix = "termcolor-0.3.6",
        build_file = Label("//remote/non_cratesio/cargo/remote:termcolor-0.3.6.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__termion__1_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.2.crate",
        type = "tar.gz",
        sha256 = "dde0593aeb8d47accea5392b39350015b5eccb12c0d98044d856983d89548dea",
        strip_prefix = "termion-1.5.2",
        build_file = Label("//remote/non_cratesio/cargo/remote:termion-1.5.2.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__thread_local__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.6.crate",
        type = "tar.gz",
        sha256 = "c6b53e329000edc2b34dbe8545fd20e55a333362d0a321909685a19bd28c3f1b",
        strip_prefix = "thread_local-0.3.6",
        build_file = Label("//remote/non_cratesio/cargo/remote:thread_local-0.3.6.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__ucd_util__0_1_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.3.crate",
        type = "tar.gz",
        sha256 = "535c204ee4d8434478593480b8f86ab45ec9aae0e83c568ca81abf0fd0e88f86",
        strip_prefix = "ucd-util-0.1.3",
        build_file = Label("//remote/non_cratesio/cargo/remote:ucd-util-0.1.3.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__utf8_ranges__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.2.crate",
        type = "tar.gz",
        sha256 = "796f7e48bef87609f7ade7e06495a87d5cd06c7866e6a5cbfceffc558a243737",
        strip_prefix = "utf8-ranges-1.0.2",
        build_file = Label("//remote/non_cratesio/cargo/remote:utf8-ranges-1.0.2.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__winapi__0_3_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.7.crate",
        type = "tar.gz",
        sha256 = "f10e386af2b13e47c89e7236a7a14a086791a2b88ebad6df9bf42040195cf770",
        strip_prefix = "winapi-0.3.7",
        build_file = Label("//remote/non_cratesio/cargo/remote:winapi-0.3.7.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/non_cratesio/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//remote/non_cratesio/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD")
    )

    _new_http_archive(
        name = "non_cratesio__wincolor__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wincolor/wincolor-0.1.6.crate",
        type = "tar.gz",
        sha256 = "eeb06499a3a4d44302791052df005d5232b927ed1a9658146d842165c4de7767",
        strip_prefix = "wincolor-0.1.6",
        build_file = Label("//remote/non_cratesio/cargo/remote:wincolor-0.1.6.BUILD")
    )

