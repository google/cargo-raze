"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        native.new_http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        native.new_git_repository(name=name, **kwargs)

def non_cratesio_fetch_remote_crates():

    _new_http_archive(
        name = "non_cratesio__aho_corasick__0_6_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.4.crate",
        type = "tar.gz",
        sha256 = "d6531d44de723825aa81398a6415283229725a00fa30713812ab9323faa82fc4",
        strip_prefix = "aho-corasick-0.6.4",
        build_file = "//remote/non_cratesio/cargo/remote:aho-corasick-0.6.4.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__atty__0_2_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.8.crate",
        type = "tar.gz",
        sha256 = "af80143d6f7608d746df1520709e5d141c96f240b0e62b0aa41bdfb53374d9d4",
        strip_prefix = "atty-0.2.8",
        build_file = "//remote/non_cratesio/cargo/remote:atty-0.2.8.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__bitflags__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.1.crate",
        type = "tar.gz",
        sha256 = "b3c30d3802dfb7281680d6285f2ccdaa8c2d8fee41f93805dba5c4cf50dc23cf",
        strip_prefix = "bitflags-1.0.1",
        build_file = "//remote/non_cratesio/cargo/remote:bitflags-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__cfg_if__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.2.crate",
        type = "tar.gz",
        sha256 = "d4c819a1287eb618df47cc647173c5c4c66ba19d888a6e50d605672aed3140de",
        strip_prefix = "cfg-if-0.1.2",
        build_file = "//remote/non_cratesio/cargo/remote:cfg-if-0.1.2.BUILD"
    )

    _new_git_repository(
        name = "non_cratesio__env_logger__0_5_5",
        remote = "https://github.com/sebasmagri/env_logger.git",
        commit = "4d7926c515a8620e7fa0e91a07047023170c1064",
        build_file = "//remote/non_cratesio/cargo/remote:env_logger-0.5.5.BUILD",
        init_submodules = True
    )

    _new_http_archive(
        name = "non_cratesio__fuchsia_zircon__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon/fuchsia-zircon-0.3.3.crate",
        type = "tar.gz",
        sha256 = "2e9763c69ebaae630ba35f74888db465e49e259ba1bc0eda7d06f4a067615d82",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = "//remote/non_cratesio/cargo/remote:fuchsia-zircon-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__fuchsia_zircon_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon-sys/fuchsia-zircon-sys-0.3.3.crate",
        type = "tar.gz",
        sha256 = "3dcaa9ae7725d12cdb85b3ad99a434db70b468c09ded17e012d86b5c1010f7a7",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = "//remote/non_cratesio/cargo/remote:fuchsia-zircon-sys-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__humantime__1_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/humantime/humantime-1.1.1.crate",
        type = "tar.gz",
        sha256 = "0484fda3e7007f2a4a0d9c3a703ca38c71c54c55602ce4660c419fd32e188c9e",
        strip_prefix = "humantime-1.1.1",
        build_file = "//remote/non_cratesio/cargo/remote:humantime-1.1.1.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__lazy_static__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.0.0.crate",
        type = "tar.gz",
        sha256 = "c8f31047daa365f19be14b47c29df4f7c3b581832407daabe6ae77397619237d",
        strip_prefix = "lazy_static-1.0.0",
        build_file = "//remote/non_cratesio/cargo/remote:lazy_static-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__libc__0_2_39",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.39.crate",
        type = "tar.gz",
        sha256 = "f54263ad99207254cf58b5f701ecb432c717445ea2ee8af387334bdd1a03fdff",
        strip_prefix = "libc-0.2.39",
        build_file = "//remote/non_cratesio/cargo/remote:libc-0.2.39.BUILD"
    )

    _new_git_repository(
        name = "non_cratesio__log__0_4_0",
        remote = "https://github.com/rust-lang-nursery/log.git",
        commit = "bf40d1f563cf3eef63233d935ce56f2198b381d3",
        build_file = "//remote/non_cratesio/cargo/remote:log-0.4.0.BUILD",
        init_submodules = True
    )

    _new_http_archive(
        name = "non_cratesio__log__0_4_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.1.crate",
        type = "tar.gz",
        sha256 = "89f010e843f2b1a31dbd316b3b8d443758bc634bed37aabade59c686d644e0a2",
        strip_prefix = "log-0.4.1",
        build_file = "//remote/non_cratesio/cargo/remote:log-0.4.1.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__memchr__2_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.0.1.crate",
        type = "tar.gz",
        sha256 = "796fba70e76612589ed2ce7f45282f5af869e0fdd7cc6199fa1aa1f1d591ba9d",
        strip_prefix = "memchr-2.0.1",
        build_file = "//remote/non_cratesio/cargo/remote:memchr-2.0.1.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__quick_error__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quick-error/quick-error-1.2.1.crate",
        type = "tar.gz",
        sha256 = "eda5fe9b71976e62bc81b781206aaa076401769b2143379d3eb2118388babac4",
        strip_prefix = "quick-error-1.2.1",
        build_file = "//remote/non_cratesio/cargo/remote:quick-error-1.2.1.BUILD"
    )

    _new_git_repository(
        name = "non_cratesio__rand__0_4_1",
        remote = "https://github.com/rust-lang-nursery/rand.git",
        commit = "b8c6ed8e54fc56d535bb60edbdc35462971c4b80",
        build_file = "//remote/non_cratesio/cargo/remote:rand-0.4.1.BUILD",
        init_submodules = True
    )

    _new_http_archive(
        name = "non_cratesio__redox_syscall__0_1_37",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.37.crate",
        type = "tar.gz",
        sha256 = "0d92eecebad22b767915e4d529f89f28ee96dbbf5a4810d2b844373f136417fd",
        strip_prefix = "redox_syscall-0.1.37",
        build_file = "//remote/non_cratesio/cargo/remote:redox_syscall-0.1.37.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        sha256 = "7e891cfe48e9100a70a3b6eb652fef28920c117d366339687bd5576160db0f76",
        strip_prefix = "redox_termios-0.1.1",
        build_file = "//remote/non_cratesio/cargo/remote:redox_termios-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__regex__0_2_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.9.crate",
        type = "tar.gz",
        sha256 = "bde64a9b799f85750f6469fd658cff5fce8d910a7d510858a1f9d15ca9f023bf",
        strip_prefix = "regex-0.2.9",
        build_file = "//remote/non_cratesio/cargo/remote:regex-0.2.9.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__regex_syntax__0_5_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.5.3.crate",
        type = "tar.gz",
        sha256 = "b2550876c31dc914696a6c2e01cbce8afba79a93c8ae979d2fe051c0230b3756",
        strip_prefix = "regex-syntax-0.5.3",
        build_file = "//remote/non_cratesio/cargo/remote:regex-syntax-0.5.3.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__termcolor__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-0.3.5.crate",
        type = "tar.gz",
        sha256 = "56c456352e44f9f91f774ddeeed27c1ec60a2455ed66d692059acfb1d731bda1",
        strip_prefix = "termcolor-0.3.5",
        build_file = "//remote/non_cratesio/cargo/remote:termcolor-0.3.5.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__termion__1_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.1.crate",
        type = "tar.gz",
        sha256 = "689a3bdfaab439fd92bc87df5c4c78417d3cbe537487274e9b0b2dce76e92096",
        strip_prefix = "termion-1.5.1",
        build_file = "//remote/non_cratesio/cargo/remote:termion-1.5.1.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__thread_local__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.5.crate",
        type = "tar.gz",
        sha256 = "279ef31c19ededf577bfd12dfae728040a21f635b06a24cd670ff510edd38963",
        strip_prefix = "thread_local-0.3.5",
        build_file = "//remote/non_cratesio/cargo/remote:thread_local-0.3.5.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__ucd_util__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.1.crate",
        type = "tar.gz",
        sha256 = "fd2be2d6639d0f8fe6cdda291ad456e23629558d466e2789d2c3e9892bda285d",
        strip_prefix = "ucd-util-0.1.1",
        build_file = "//remote/non_cratesio/cargo/remote:ucd-util-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__unreachable__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unreachable/unreachable-1.0.0.crate",
        type = "tar.gz",
        sha256 = "382810877fe448991dfc7f0dd6e3ae5d58088fd0ea5e35189655f84e6814fa56",
        strip_prefix = "unreachable-1.0.0",
        build_file = "//remote/non_cratesio/cargo/remote:unreachable-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__utf8_ranges__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.0.crate",
        type = "tar.gz",
        sha256 = "662fab6525a98beff2921d7f61a39e7d59e0b425ebc7d0d9e66d316e55124122",
        strip_prefix = "utf8-ranges-1.0.0",
        build_file = "//remote/non_cratesio/cargo/remote:utf8-ranges-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__void__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/void/void-1.0.2.crate",
        type = "tar.gz",
        sha256 = "6a02e4885ed3bc0f2de90ea6dd45ebcbb66dacffe03547fadbb0eeae2770887d",
        strip_prefix = "void-1.0.2",
        build_file = "//remote/non_cratesio/cargo/remote:void-1.0.2.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__winapi__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.4.crate",
        type = "tar.gz",
        sha256 = "04e3bd221fcbe8a271359c04f21a76db7d0c6028862d1bb5512d85e1e2eb5bb3",
        strip_prefix = "winapi-0.3.4",
        build_file = "//remote/non_cratesio/cargo/remote:winapi-0.3.4.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = "//remote/non_cratesio/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = "//remote/non_cratesio/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"
    )

    _new_http_archive(
        name = "non_cratesio__wincolor__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wincolor/wincolor-0.1.6.crate",
        type = "tar.gz",
        sha256 = "eeb06499a3a4d44302791052df005d5232b927ed1a9658146d842165c4de7767",
        strip_prefix = "wincolor-0.1.6",
        build_file = "//remote/non_cratesio/cargo/remote:wincolor-0.1.6.BUILD"
    )

