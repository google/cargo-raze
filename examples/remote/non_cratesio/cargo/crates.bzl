"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""

def non_cratesio_fetch_remote_crates():

    native.new_http_archive(
        name = "non_cratesio__aho_corasick__0_6_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.4.crate",
        type = "tar.gz",
        strip_prefix = "aho-corasick-0.6.4",
        build_file = "//remote/non_cratesio/cargo/remote:aho-corasick-0.6.4.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__atty__0_2_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.8.crate",
        type = "tar.gz",
        strip_prefix = "atty-0.2.8",
        build_file = "//remote/non_cratesio/cargo/remote:atty-0.2.8.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__bitflags__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.1.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-1.0.1",
        build_file = "//remote/non_cratesio/cargo/remote:bitflags-1.0.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__cfg_if__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "cfg-if-0.1.2",
        build_file = "//remote/non_cratesio/cargo/remote:cfg-if-0.1.2.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__env_logger__0_5_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.5.5.crate",
        type = "tar.gz",
        strip_prefix = "env_logger-0.5.5",
        build_file = "//remote/non_cratesio/cargo/remote:env_logger-0.5.5.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__fuchsia_zircon__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon/fuchsia-zircon-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = "//remote/non_cratesio/cargo/remote:fuchsia-zircon-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__fuchsia_zircon_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon-sys/fuchsia-zircon-sys-0.3.3.crate",
        type = "tar.gz",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = "//remote/non_cratesio/cargo/remote:fuchsia-zircon-sys-0.3.3.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__humantime__1_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/humantime/humantime-1.1.1.crate",
        type = "tar.gz",
        strip_prefix = "humantime-1.1.1",
        build_file = "//remote/non_cratesio/cargo/remote:humantime-1.1.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__lazy_static__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-1.0.0",
        build_file = "//remote/non_cratesio/cargo/remote:lazy_static-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__libc__0_2_39",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.39.crate",
        type = "tar.gz",
        strip_prefix = "libc-0.2.39",
        build_file = "//remote/non_cratesio/cargo/remote:libc-0.2.39.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__log__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "log-0.4.0",
        build_file = "//remote/non_cratesio/cargo/remote:log-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__log__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.1.crate",
        type = "tar.gz",
        strip_prefix = "log-0.4.1",
        build_file = "//remote/non_cratesio/cargo/remote:log-0.4.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__memchr__2_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.0.1.crate",
        type = "tar.gz",
        strip_prefix = "memchr-2.0.1",
        build_file = "//remote/non_cratesio/cargo/remote:memchr-2.0.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__quick_error__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quick-error/quick-error-1.2.1.crate",
        type = "tar.gz",
        strip_prefix = "quick-error-1.2.1",
        build_file = "//remote/non_cratesio/cargo/remote:quick-error-1.2.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__rand__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.4.1.crate",
        type = "tar.gz",
        strip_prefix = "rand-0.4.1",
        build_file = "//remote/non_cratesio/cargo/remote:rand-0.4.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__redox_syscall__0_1_37",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.37.crate",
        type = "tar.gz",
        strip_prefix = "redox_syscall-0.1.37",
        build_file = "//remote/non_cratesio/cargo/remote:redox_syscall-0.1.37.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "redox_termios-0.1.1",
        build_file = "//remote/non_cratesio/cargo/remote:redox_termios-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__regex__0_2_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.9.crate",
        type = "tar.gz",
        strip_prefix = "regex-0.2.9",
        build_file = "//remote/non_cratesio/cargo/remote:regex-0.2.9.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__regex_syntax__0_5_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.5.3.crate",
        type = "tar.gz",
        strip_prefix = "regex-syntax-0.5.3",
        build_file = "//remote/non_cratesio/cargo/remote:regex-syntax-0.5.3.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__termcolor__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-0.3.5.crate",
        type = "tar.gz",
        strip_prefix = "termcolor-0.3.5",
        build_file = "//remote/non_cratesio/cargo/remote:termcolor-0.3.5.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__termion__1_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.1.crate",
        type = "tar.gz",
        strip_prefix = "termion-1.5.1",
        build_file = "//remote/non_cratesio/cargo/remote:termion-1.5.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__thread_local__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.5.crate",
        type = "tar.gz",
        strip_prefix = "thread_local-0.3.5",
        build_file = "//remote/non_cratesio/cargo/remote:thread_local-0.3.5.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__ucd_util__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "ucd-util-0.1.1",
        build_file = "//remote/non_cratesio/cargo/remote:ucd-util-0.1.1.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__unreachable__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unreachable/unreachable-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "unreachable-1.0.0",
        build_file = "//remote/non_cratesio/cargo/remote:unreachable-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__utf8_ranges__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.0.crate",
        type = "tar.gz",
        strip_prefix = "utf8-ranges-1.0.0",
        build_file = "//remote/non_cratesio/cargo/remote:utf8-ranges-1.0.0.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__void__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/void/void-1.0.2.crate",
        type = "tar.gz",
        strip_prefix = "void-1.0.2",
        build_file = "//remote/non_cratesio/cargo/remote:void-1.0.2.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__winapi__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.4.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.3.4",
        build_file = "//remote/non_cratesio/cargo/remote:winapi-0.3.4.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = "//remote/non_cratesio/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = "//remote/non_cratesio/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"
    )

    native.new_http_archive(
        name = "non_cratesio__wincolor__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wincolor/wincolor-0.1.6.crate",
        type = "tar.gz",
        strip_prefix = "wincolor-0.1.6",
        build_file = "//remote/non_cratesio/cargo/remote:wincolor-0.1.6.BUILD"
    )

