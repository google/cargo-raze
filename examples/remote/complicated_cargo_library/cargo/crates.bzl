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

def complicated_cargo_library_fetch_remote_crates():

    _new_http_archive(
        name = "complicated_cargo_library__aho_corasick__0_6_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.4.crate",
        type = "tar.gz",
        sha256 = "d6531d44de723825aa81398a6415283229725a00fa30713812ab9323faa82fc4",
        strip_prefix = "aho-corasick-0.6.4",
        build_file = "//remote/complicated_cargo_library/cargo/remote:aho-corasick-0.6.4.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__arrayvec__0_3_25",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.3.25.crate",
        type = "tar.gz",
        sha256 = "06f59fe10306bb78facd90d28c2038ad23ffaaefa85bac43c8a434cde383334f",
        strip_prefix = "arrayvec-0.3.25",
        build_file = "//remote/complicated_cargo_library/cargo/remote:arrayvec-0.3.25.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__arrayvec__0_4_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/arrayvec/arrayvec-0.4.7.crate",
        type = "tar.gz",
        sha256 = "a1e964f9e24d588183fcb43503abda40d288c8657dfc27311516ce2f05675aef",
        strip_prefix = "arrayvec-0.4.7",
        build_file = "//remote/complicated_cargo_library/cargo/remote:arrayvec-0.4.7.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__atom__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atom/atom-0.3.4.crate",
        type = "tar.gz",
        sha256 = "4cd7b80cba09d9c6679f5ac66af2e5eb9c17fa1b914f142d690b069ba51eacaf",
        strip_prefix = "atom-0.3.4",
        build_file = "//remote/complicated_cargo_library/cargo/remote:atom-0.3.4.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__bitflags__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.0.1.crate",
        type = "tar.gz",
        sha256 = "b3c30d3802dfb7281680d6285f2ccdaa8c2d8fee41f93805dba5c4cf50dc23cf",
        strip_prefix = "bitflags-1.0.1",
        build_file = "//remote/complicated_cargo_library/cargo/remote:bitflags-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__cfg_if__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.2.crate",
        type = "tar.gz",
        sha256 = "d4c819a1287eb618df47cc647173c5c4c66ba19d888a6e50d605672aed3140de",
        strip_prefix = "cfg-if-0.1.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:cfg-if-0.1.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__crossbeam__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam/crossbeam-0.3.2.crate",
        type = "tar.gz",
        sha256 = "24ce9782d4d5c53674646a6a4c1863a21a8fc0cb649b3c94dfc16e45071dea19",
        strip_prefix = "crossbeam-0.3.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:crossbeam-0.3.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__crossbeam_deque__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-deque/crossbeam-deque-0.2.0.crate",
        type = "tar.gz",
        sha256 = "f739f8c5363aca78cfb059edf753d8f0d36908c348f3d8d1503f03d8b75d9cf3",
        strip_prefix = "crossbeam-deque-0.2.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:crossbeam-deque-0.2.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__crossbeam_epoch__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-epoch/crossbeam-epoch-0.3.0.crate",
        type = "tar.gz",
        sha256 = "59796cc6cbbdc6bb319161349db0c3250ec73ec7fcb763a51065ec4e2e158552",
        strip_prefix = "crossbeam-epoch-0.3.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:crossbeam-epoch-0.3.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__crossbeam_utils__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/crossbeam-utils/crossbeam-utils-0.2.2.crate",
        type = "tar.gz",
        sha256 = "2760899e32a1d58d5abb31129f8fae5de75220bc2176e77ff7c627ae45c918d9",
        strip_prefix = "crossbeam-utils-0.2.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:crossbeam-utils-0.2.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__derivative__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/derivative/derivative-1.0.0.crate",
        type = "tar.gz",
        sha256 = "67b3d6d0e84e53a5bdc263cc59340541877bb541706a191d762bfac6a481bdde",
        strip_prefix = "derivative-1.0.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:derivative-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__either__1_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/either/either-1.4.0.crate",
        type = "tar.gz",
        sha256 = "740178ddf48b1a9e878e6d6509a1442a2d42fd2928aae8e7a6f8a36fb01981b3",
        strip_prefix = "either-1.4.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:either-1.4.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__fnv__1_0_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fnv/fnv-1.0.6.crate",
        type = "tar.gz",
        sha256 = "2fad85553e09a6f881f739c29f0b00b0f01357c743266d478b68951ce23285f3",
        strip_prefix = "fnv-1.0.6",
        build_file = "//remote/complicated_cargo_library/cargo/remote:fnv-1.0.6.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__fuchsia_zircon__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon/fuchsia-zircon-0.3.3.crate",
        type = "tar.gz",
        sha256 = "2e9763c69ebaae630ba35f74888db465e49e259ba1bc0eda7d06f4a067615d82",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = "//remote/complicated_cargo_library/cargo/remote:fuchsia-zircon-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__fuchsia_zircon_sys__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-zircon-sys/fuchsia-zircon-sys-0.3.3.crate",
        type = "tar.gz",
        sha256 = "3dcaa9ae7725d12cdb85b3ad99a434db70b468c09ded17e012d86b5c1010f7a7",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = "//remote/complicated_cargo_library/cargo/remote:fuchsia-zircon-sys-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__hibitset__0_3_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hibitset/hibitset-0.3.2.crate",
        type = "tar.gz",
        sha256 = "b78998e3c243d71525596e8f373dfc4b82703f25907b9e4d260383cff8307d84",
        strip_prefix = "hibitset-0.3.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:hibitset-0.3.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__itertools__0_5_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/itertools/itertools-0.5.10.crate",
        type = "tar.gz",
        sha256 = "4833d6978da405305126af4ac88569b5d71ff758581ce5a987dbfa3755f694fc",
        strip_prefix = "itertools-0.5.10",
        build_file = "//remote/complicated_cargo_library/cargo/remote:itertools-0.5.10.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__lazy_static__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-0.2.11.crate",
        type = "tar.gz",
        sha256 = "76f033c7ad61445c5b347c7382dd1237847eb1bce590fe50365dcb33d546be73",
        strip_prefix = "lazy_static-0.2.11",
        build_file = "//remote/complicated_cargo_library/cargo/remote:lazy_static-0.2.11.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__lazy_static__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.0.0.crate",
        type = "tar.gz",
        sha256 = "c8f31047daa365f19be14b47c29df4f7c3b581832407daabe6ae77397619237d",
        strip_prefix = "lazy_static-1.0.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:lazy_static-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__libc__0_2_36",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.36.crate",
        type = "tar.gz",
        sha256 = "1e5d97d6708edaa407429faa671b942dc0f2727222fb6b6539bf1db936e4b121",
        strip_prefix = "libc-0.2.36",
        build_file = "//remote/complicated_cargo_library/cargo/remote:libc-0.2.36.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__memchr__2_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.0.1.crate",
        type = "tar.gz",
        sha256 = "796fba70e76612589ed2ce7f45282f5af869e0fdd7cc6199fa1aa1f1d591ba9d",
        strip_prefix = "memchr-2.0.1",
        build_file = "//remote/complicated_cargo_library/cargo/remote:memchr-2.0.1.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__memoffset__0_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memoffset/memoffset-0.2.1.crate",
        type = "tar.gz",
        sha256 = "0f9dc261e2b62d7a622bf416ea3c5245cdd5d9a7fcc428c0d06804dfce1775b3",
        strip_prefix = "memoffset-0.2.1",
        build_file = "//remote/complicated_cargo_library/cargo/remote:memoffset-0.2.1.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__mopa__0_2_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/mopa/mopa-0.2.2.crate",
        type = "tar.gz",
        sha256 = "a785740271256c230f57462d3b83e52f998433a7062fc18f96d5999474a9f915",
        strip_prefix = "mopa-0.2.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:mopa-0.2.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__nodrop__0_1_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nodrop/nodrop-0.1.12.crate",
        type = "tar.gz",
        sha256 = "9a2228dca57108069a5262f2ed8bd2e82496d2e074a06d1ccc7ce1687b6ae0a2",
        strip_prefix = "nodrop-0.1.12",
        build_file = "//remote/complicated_cargo_library/cargo/remote:nodrop-0.1.12.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__num_cpus__1_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num_cpus/num_cpus-1.8.0.crate",
        type = "tar.gz",
        sha256 = "c51a3322e4bca9d212ad9a158a02abc6934d005490c054a2778df73a70aa0a30",
        strip_prefix = "num_cpus-1.8.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:num_cpus-1.8.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__odds__0_2_26",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/odds/odds-0.2.26.crate",
        type = "tar.gz",
        sha256 = "4eae0151b9dacf24fcc170d9995e511669a082856a91f958a2fe380bfab3fb22",
        strip_prefix = "odds-0.2.26",
        build_file = "//remote/complicated_cargo_library/cargo/remote:odds-0.2.26.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__pulse__0_5_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/pulse/pulse-0.5.3.crate",
        type = "tar.gz",
        sha256 = "655612b6c8d96a8a02f331fe296cb4f925b68e87c1d195544675abca2d9b9af0",
        strip_prefix = "pulse-0.5.3",
        build_file = "//remote/complicated_cargo_library/cargo/remote:pulse-0.5.3.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__quote__0_3_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.3.15.crate",
        type = "tar.gz",
        sha256 = "7a6e920b65c65f10b2ae65c831a81a073a89edd28c7cce89475bff467ab4167a",
        strip_prefix = "quote-0.3.15",
        build_file = "//remote/complicated_cargo_library/cargo/remote:quote-0.3.15.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__rand__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.4.2.crate",
        type = "tar.gz",
        sha256 = "eba5f8cb59cc50ed56be8880a5c7b496bfd9bd26394e176bc67884094145c2c5",
        strip_prefix = "rand-0.4.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:rand-0.4.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__rayon__0_8_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon/rayon-0.8.2.crate",
        type = "tar.gz",
        sha256 = "b614fe08b6665cb9a231d07ac1364b0ef3cb3698f1239ee0c4c3a88a524f54c8",
        strip_prefix = "rayon-0.8.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:rayon-0.8.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__rayon_core__1_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rayon-core/rayon-core-1.4.0.crate",
        type = "tar.gz",
        sha256 = "9d24ad214285a7729b174ed6d3bcfcb80177807f959d95fafd5bfc5c4f201ac8",
        strip_prefix = "rayon-core-1.4.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:rayon-core-1.4.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__redox_syscall__0_1_37",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.37.crate",
        type = "tar.gz",
        sha256 = "0d92eecebad22b767915e4d529f89f28ee96dbbf5a4810d2b844373f136417fd",
        strip_prefix = "redox_syscall-0.1.37",
        build_file = "//remote/complicated_cargo_library/cargo/remote:redox_syscall-0.1.37.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__regex__0_2_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.6.crate",
        type = "tar.gz",
        sha256 = "5be5347bde0c48cfd8c3fdc0766cdfe9d8a755ef84d620d6794c778c91de8b2b",
        strip_prefix = "regex-0.2.6",
        build_file = "//remote/complicated_cargo_library/cargo/remote:regex-0.2.6.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__regex_syntax__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.4.2.crate",
        type = "tar.gz",
        sha256 = "8e931c58b93d86f080c734bfd2bce7dd0079ae2331235818133c8be7f422e20e",
        strip_prefix = "regex-syntax-0.4.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:regex-syntax-0.4.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__scopeguard__0_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/scopeguard/scopeguard-0.3.3.crate",
        type = "tar.gz",
        sha256 = "94258f53601af11e6a49f722422f6e3425c52b06245a5cf9bc09908b174f5e27",
        strip_prefix = "scopeguard-0.3.3",
        build_file = "//remote/complicated_cargo_library/cargo/remote:scopeguard-0.3.3.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__shred__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred/shred-0.5.2.crate",
        type = "tar.gz",
        sha256 = "7d3abceaa9d0a9b47ab84b53c6029c21bcad7d7dd63e14db51ea0680faee2159",
        strip_prefix = "shred-0.5.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:shred-0.5.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__shred_derive__0_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/shred-derive/shred-derive-0.3.0.crate",
        type = "tar.gz",
        sha256 = "a4a894913b6e93fe2cd712a3bc955ec6f6b01c675c1c58b02fdfa13f77868049",
        strip_prefix = "shred-derive-0.3.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:shred-derive-0.3.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__smallvec__0_4_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/smallvec/smallvec-0.4.4.crate",
        type = "tar.gz",
        sha256 = "ee4f357e8cd37bf8822e1b964e96fd39e2cb5a0424f8aaa284ccaccc2162411c",
        strip_prefix = "smallvec-0.4.4",
        build_file = "//remote/complicated_cargo_library/cargo/remote:smallvec-0.4.4.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__specs__0_10_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/specs/specs-0.10.0.crate",
        type = "tar.gz",
        sha256 = "a210dc96ea065cb88391aa6956ed1b2a14051c668b5bc18bac66a95c215b639f",
        strip_prefix = "specs-0.10.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:specs-0.10.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__syn__0_10_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.10.8.crate",
        type = "tar.gz",
        sha256 = "58fd09df59565db3399efbba34ba8a2fec1307511ebd245d0061ff9d42691673",
        strip_prefix = "syn-0.10.8",
        build_file = "//remote/complicated_cargo_library/cargo/remote:syn-0.10.8.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__syn__0_11_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.11.11.crate",
        type = "tar.gz",
        sha256 = "d3b891b9015c88c576343b9b3e41c2c11a51c219ef067b264bd9c8aa9b441dad",
        strip_prefix = "syn-0.11.11",
        build_file = "//remote/complicated_cargo_library/cargo/remote:syn-0.11.11.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__synom__0_11_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/synom/synom-0.11.3.crate",
        type = "tar.gz",
        sha256 = "a393066ed9010ebaed60b9eafa373d4b1baac186dd7e008555b0f702b51945b6",
        strip_prefix = "synom-0.11.3",
        build_file = "//remote/complicated_cargo_library/cargo/remote:synom-0.11.3.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__thread_local__0_3_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.5.crate",
        type = "tar.gz",
        sha256 = "279ef31c19ededf577bfd12dfae728040a21f635b06a24cd670ff510edd38963",
        strip_prefix = "thread_local-0.3.5",
        build_file = "//remote/complicated_cargo_library/cargo/remote:thread_local-0.3.5.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__time__0_1_39",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.39.crate",
        type = "tar.gz",
        sha256 = "a15375f1df02096fb3317256ce2cee6a1f42fc84ea5ad5fc8c421cfe40c73098",
        strip_prefix = "time-0.1.39",
        build_file = "//remote/complicated_cargo_library/cargo/remote:time-0.1.39.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__tuple_utils__0_2_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/tuple_utils/tuple_utils-0.2.0.crate",
        type = "tar.gz",
        sha256 = "cbfecd7bb8f0a3e96b3b31c46af2677a55a588767c0091f484601424fcb20e7e",
        strip_prefix = "tuple_utils-0.2.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:tuple_utils-0.2.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__unicode_xid__0_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.0.4.crate",
        type = "tar.gz",
        sha256 = "8c1f860d7d29cf02cb2f3f359fd35991af3d30bac52c57d265a3c461074cb4dc",
        strip_prefix = "unicode-xid-0.0.4",
        build_file = "//remote/complicated_cargo_library/cargo/remote:unicode-xid-0.0.4.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__unreachable__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unreachable/unreachable-1.0.0.crate",
        type = "tar.gz",
        sha256 = "382810877fe448991dfc7f0dd6e3ae5d58088fd0ea5e35189655f84e6814fa56",
        strip_prefix = "unreachable-1.0.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:unreachable-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__utf8_ranges__1_0_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.0.crate",
        type = "tar.gz",
        sha256 = "662fab6525a98beff2921d7f61a39e7d59e0b425ebc7d0d9e66d316e55124122",
        strip_prefix = "utf8-ranges-1.0.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:utf8-ranges-1.0.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__void__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/void/void-1.0.2.crate",
        type = "tar.gz",
        sha256 = "6a02e4885ed3bc0f2de90ea6dd45ebcbb66dacffe03547fadbb0eeae2770887d",
        strip_prefix = "void-1.0.2",
        build_file = "//remote/complicated_cargo_library/cargo/remote:void-1.0.2.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__winapi__0_3_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.4.crate",
        type = "tar.gz",
        sha256 = "04e3bd221fcbe8a271359c04f21a76db7d0c6028862d1bb5512d85e1e2eb5bb3",
        strip_prefix = "winapi-0.3.4",
        build_file = "//remote/complicated_cargo_library/cargo/remote:winapi-0.3.4.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"
    )

    _new_http_archive(
        name = "complicated_cargo_library__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = "//remote/complicated_cargo_library/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"
    )

