"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of normal dependencies for the Rust targets of that package.
_DEPENDENCIES = {
    "impl": {
        "anyhow": "@cargo_raze__anyhow__1_0_37//:anyhow",
        "cargo-clone-crate": "@cargo_raze__cargo_clone_crate__0_1_5//:cargo_clone_crate",
        "cargo-lock": "@cargo_raze__cargo_lock__6_0_0//:cargo_lock",
        "cargo-platform": "@cargo_raze__cargo_platform__0_1_1//:cargo_platform",
        "cargo_metadata": "@cargo_raze__cargo_metadata__0_12_1//:cargo_metadata",
        "cargo_toml": "@cargo_raze__cargo_toml__0_8_1//:cargo_toml",
        "cfg-expr": "@cargo_raze__cfg_expr__0_5_1//:cfg_expr",
        "crates-index": "@cargo_raze__crates_index__0_16_2//:crates_index",
        "docopt": "@cargo_raze__docopt__1_1_0//:docopt",
        "glob": "@cargo_raze__glob__0_3_0//:glob",
        "itertools": "@cargo_raze__itertools__0_10_0//:itertools",
        "log": "@cargo_raze__log__0_4_11//:log",
        "pathdiff": "@cargo_raze__pathdiff__0_2_0//:pathdiff",
        "regex": "@cargo_raze__regex__1_4_2//:regex",
        "rustc-serialize": "@cargo_raze__rustc_serialize__0_3_24//:rustc_serialize",
        "semver": "@cargo_raze__semver__0_11_0//:semver",
        "serde": "@cargo_raze__serde__1_0_118//:serde",
        "serde_json": "@cargo_raze__serde_json__1_0_61//:serde_json",
        "slug": "@cargo_raze__slug__0_1_4//:slug",
        "spdx": "@cargo_raze__spdx__0_3_4//:spdx",
        "tempfile": "@cargo_raze__tempfile__3_1_0//:tempfile",
        "tera": "@cargo_raze__tera__1_6_1//:tera",
        "toml": "@cargo_raze__toml__0_5_8//:toml",
        "url": "@cargo_raze__url__2_2_0//:url",
    },
}

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of proc_macro dependencies for the Rust targets of that package.
_PROC_MACRO_DEPENDENCIES = {
    "impl": {
        "serde_derive": "@cargo_raze__serde_derive__1_0_118//:serde_derive",
    },
}

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of normal dev dependencies for the Rust targets of that package.
_DEV_DEPENDENCIES = {
    "impl": {
        "flate2": "@cargo_raze__flate2__1_0_19//:flate2",
        "hamcrest2": "@cargo_raze__hamcrest2__0_3_0//:hamcrest2",
        "httpmock": "@cargo_raze__httpmock__0_5_2//:httpmock",
        "lazy_static": "@cargo_raze__lazy_static__1_4_0//:lazy_static",
        "tar": "@cargo_raze__tar__0_4_30//:tar",
    },
}

# EXPERIMENTAL -- MAY CHANGE AT ANY TIME: A mapping of package names to a set of proc_macro dev dependencies for the Rust targets of that package.
_DEV_PROC_MACRO_DEPENDENCIES = {
    "impl": {
        "indoc": "@cargo_raze__indoc__1_0_3//:indoc",
    },
}

def crate_deps(deps, package_name = None):
    """EXPERIMENTAL -- MAY CHANGE AT ANY TIME: Finds the fully qualified label of the requested crates for the package where this macro is called.

    WARNING: This macro is part of an expeirmental API and is subject to change.

    Args:
        deps (list): The desired list of crate targets.
        package_name (str, optional): The package name of the set of dependencies to look up.
            Defaults to `native.package_name()`.
    Returns:
        list: A list of labels to cargo-raze generated targets (str)
    """

    if not package_name:
        package_name = native.package_name()

    # Join both sets of dependencies
    dependencies = _flatten_dependency_maps([
        _DEPENDENCIES,
        _PROC_MACRO_DEPENDENCIES,
        _DEV_DEPENDENCIES,
        _DEV_PROC_MACRO_DEPENDENCIES,
    ])

    if not deps:
        return []

    missing_crates = []
    crate_targets = []
    for crate_target in deps:
        if crate_target not in dependencies[package_name]:
            missing_crates.append(crate_target)
        else:
            crate_targets.append(dependencies[package_name][crate_target])

    if missing_crates:
        fail("Could not find crates `{}` among dependencies of `{}`. Available dependencies were `{}`".format(
            missing_crates,
            package_name,
            dependencies[package_name],
        ))

    return crate_targets

def all_crate_deps(normal = False, normal_dev = False, proc_macro = False, proc_macro_dev = False, package_name = None):
    """EXPERIMENTAL -- MAY CHANGE AT ANY TIME: Finds the fully qualified label of all requested direct crate dependencies \
    for the package where this macro is called.

    If no parameters are set, all normal dependencies are returned. Setting any one flag will
    otherwise impact the contents of the returned list.

    Args:
        normal (bool, optional): If True, normal dependencies are included in the
            output list. Defaults to False.
        normal_dev (bool, optional): If True, normla dev dependencies will be
            included in the output list. Defaults to False.
        proc_macro (bool, optional): If True, proc_macro dependencies are included
            in the output list. Defaults to False.
        proc_macro_dev (bool, optional): If True, dev proc_macro dependencies are
            included in the output list. Defaults to False.
        package_name (str, optional): The package name of the set of dependencies to look up.
            Defaults to `native.package_name()`.

    Returns:
        list: A list of labels to cargo-raze generated targets (str)
    """

    if not package_name:
        package_name = native.package_name()

    # Determine the relevant maps to use
    all_dependency_maps = []
    if normal:
        all_dependency_maps.append(_DEPENDENCIES)
    if normal_dev:
        all_dependency_maps.append(_DEV_DEPENDENCIES)
    if proc_macro:
        all_dependency_maps.append(_PROC_MACRO_DEPENDENCIES)
    if proc_macro_dev:
        all_dependency_maps.append(_DEV_PROC_MACRO_DEPENDENCIES)

    # Default to always using normal dependencies
    if not all_dependency_maps:
        all_dependency_maps.append(_DEPENDENCIES)

    dependencies = _flatten_dependency_maps(all_dependency_maps)

    if not dependencies:
        return []

    return dependencies[package_name].values()

def _flatten_dependency_maps(all_dependency_maps):
    """Flatten a list of dependency maps into one dictionary.

    Dependency maps have the following structure:

    ```python
    DEPENDENCIES_MAP = {
        # The first key in the map is a Bazel package
        # name of the workspace this file is defined in.
        "package_name": {

            # An alias to a crate target.     # The label of the crate target the
            # Aliases are only crate names.   # alias refers to.
            "alias":                          "@full//:label",
        }
    }
    ```

    Args:
        all_dependency_maps (list): A list of dicts as described above

    Returns:
        dict: A dictionary as described above
    """
    dependencies = {}

    for dep_map in all_dependency_maps:
        for pkg_name in dep_map:
            if pkg_name not in dependencies:
                # Add a non-frozen dict to the collection of dependencies
                dependencies.setdefault(pkg_name, dict(dep_map[pkg_name].items()))
                continue

            duplicate_crate_aliases = [key for key in dependencies[pkg_name] if key in dep_map[pkg_name]]
            if duplicate_crate_aliases:
                fail("There should be no duplicate crate aliases: {}".format(duplicate_crate_aliases))

            dependencies[pkg_name].update(dep_map[pkg_name])

    return dependencies

def cargo_raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "cargo_raze__adler__0_2_3",
        url = "https://crates.io/api/v1/crates/adler/0.2.3/download",
        type = "tar.gz",
        sha256 = "ee2a4ec343196209d6594e19543ae87a39f96d5534d7174822a3ad825dd6ed7e",
        strip_prefix = "adler-0.2.3",
        build_file = Label("//third_party/cargo/remote:BUILD.adler-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__aho_corasick__0_7_15",
        url = "https://crates.io/api/v1/crates/aho-corasick/0.7.15/download",
        type = "tar.gz",
        sha256 = "7404febffaa47dac81aa44dba71523c9d069b1bdc50a77db41195149e17f68e5",
        strip_prefix = "aho-corasick-0.7.15",
        build_file = Label("//third_party/cargo/remote:BUILD.aho-corasick-0.7.15.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__anyhow__1_0_37",
        url = "https://crates.io/api/v1/crates/anyhow/1.0.37/download",
        type = "tar.gz",
        sha256 = "ee67c11feeac938fae061b232e38e0b6d94f97a9df10e6271319325ac4c56a86",
        strip_prefix = "anyhow-1.0.37",
        build_file = Label("//third_party/cargo/remote:BUILD.anyhow-1.0.37.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__arrayref__0_3_6",
        url = "https://crates.io/api/v1/crates/arrayref/0.3.6/download",
        type = "tar.gz",
        sha256 = "a4c527152e37cf757a3f78aae5a06fbeefdb07ccc535c980a3208ee3060dd544",
        strip_prefix = "arrayref-0.3.6",
        build_file = Label("//third_party/cargo/remote:BUILD.arrayref-0.3.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__arrayvec__0_5_2",
        url = "https://crates.io/api/v1/crates/arrayvec/0.5.2/download",
        type = "tar.gz",
        sha256 = "23b62fc65de8e4e7f52534fb52b0f3ed04746ae267519eef2a83941e8085068b",
        strip_prefix = "arrayvec-0.5.2",
        build_file = Label("//third_party/cargo/remote:BUILD.arrayvec-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ascii_canvas__2_0_0",
        url = "https://crates.io/api/v1/crates/ascii-canvas/2.0.0/download",
        type = "tar.gz",
        sha256 = "ff8eb72df928aafb99fe5d37b383f2fe25bd2a765e3e5f7c365916b6f2463a29",
        strip_prefix = "ascii-canvas-2.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.ascii-canvas-2.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__assert_json_diff__1_1_0",
        url = "https://crates.io/api/v1/crates/assert-json-diff/1.1.0/download",
        type = "tar.gz",
        sha256 = "4259cbe96513d2f1073027a259fc2ca917feb3026a5a8d984e3628e490255cc0",
        strip_prefix = "assert-json-diff-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.assert-json-diff-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_channel__1_5_1",
        url = "https://crates.io/api/v1/crates/async-channel/1.5.1/download",
        type = "tar.gz",
        sha256 = "59740d83946db6a5af71ae25ddf9562c2b176b2ca42cf99a455f09f4a220d6b9",
        strip_prefix = "async-channel-1.5.1",
        build_file = Label("//third_party/cargo/remote:BUILD.async-channel-1.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_executor__1_4_0",
        url = "https://crates.io/api/v1/crates/async-executor/1.4.0/download",
        type = "tar.gz",
        sha256 = "eb877970c7b440ead138f6321a3b5395d6061183af779340b65e20c0fede9146",
        strip_prefix = "async-executor-1.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.async-executor-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_global_executor__1_4_3",
        url = "https://crates.io/api/v1/crates/async-global-executor/1.4.3/download",
        type = "tar.gz",
        sha256 = "73079b49cd26b8fd5a15f68fc7707fc78698dc2a3d61430f2a7a9430230dfa04",
        strip_prefix = "async-global-executor-1.4.3",
        build_file = Label("//third_party/cargo/remote:BUILD.async-global-executor-1.4.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_io__1_3_1",
        url = "https://crates.io/api/v1/crates/async-io/1.3.1/download",
        type = "tar.gz",
        sha256 = "9315f8f07556761c3e48fec2e6b276004acf426e6dc068b2c2251854d65ee0fd",
        strip_prefix = "async-io-1.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.async-io-1.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_mutex__1_4_0",
        url = "https://crates.io/api/v1/crates/async-mutex/1.4.0/download",
        type = "tar.gz",
        sha256 = "479db852db25d9dbf6204e6cb6253698f175c15726470f78af0d918e99d6156e",
        strip_prefix = "async-mutex-1.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.async-mutex-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_process__1_0_1",
        url = "https://crates.io/api/v1/crates/async-process/1.0.1/download",
        type = "tar.gz",
        sha256 = "4c8cea09c1fb10a317d1b5af8024eeba256d6554763e85ecd90ff8df31c7bbda",
        strip_prefix = "async-process-1.0.1",
        build_file = Label("//third_party/cargo/remote:BUILD.async-process-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_std__1_8_0",
        url = "https://crates.io/api/v1/crates/async-std/1.8.0/download",
        type = "tar.gz",
        sha256 = "8f9f84f1280a2b436a2c77c2582602732b6c2f4321d5494d6e799e6c367859a8",
        strip_prefix = "async-std-1.8.0",
        build_file = Label("//third_party/cargo/remote:BUILD.async-std-1.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_task__4_0_3",
        url = "https://crates.io/api/v1/crates/async-task/4.0.3/download",
        type = "tar.gz",
        sha256 = "e91831deabf0d6d7ec49552e489aed63b7456a7a3c46cff62adad428110b0af0",
        strip_prefix = "async-task-4.0.3",
        build_file = Label("//third_party/cargo/remote:BUILD.async-task-4.0.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__async_trait__0_1_42",
        url = "https://crates.io/api/v1/crates/async-trait/0.1.42/download",
        type = "tar.gz",
        sha256 = "8d3a45e77e34375a7923b1e8febb049bb011f064714a8e17a1a616fef01da13d",
        strip_prefix = "async-trait-0.1.42",
        build_file = Label("//third_party/cargo/remote:BUILD.async-trait-0.1.42.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__atomic_waker__1_0_0",
        url = "https://crates.io/api/v1/crates/atomic-waker/1.0.0/download",
        type = "tar.gz",
        sha256 = "065374052e7df7ee4047b1160cca5e1467a12351a40b3da123c870ba0b8eda2a",
        strip_prefix = "atomic-waker-1.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.atomic-waker-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__atty__0_2_14",
        url = "https://crates.io/api/v1/crates/atty/0.2.14/download",
        type = "tar.gz",
        sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8",
        strip_prefix = "atty-0.2.14",
        build_file = Label("//third_party/cargo/remote:BUILD.atty-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__autocfg__1_0_1",
        url = "https://crates.io/api/v1/crates/autocfg/1.0.1/download",
        type = "tar.gz",
        sha256 = "cdb031dd78e28731d87d56cc8ffef4a8f36ca26c38fe2de700543e627f8a464a",
        strip_prefix = "autocfg-1.0.1",
        build_file = Label("//third_party/cargo/remote:BUILD.autocfg-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__base64__0_13_0",
        url = "https://crates.io/api/v1/crates/base64/0.13.0/download",
        type = "tar.gz",
        sha256 = "904dfeac50f3cdaba28fc6f57fdcddb75f49ed61346676a78c4ffe55877802fd",
        strip_prefix = "base64-0.13.0",
        build_file = Label("//third_party/cargo/remote:BUILD.base64-0.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__basic_cookies__0_1_4",
        url = "https://crates.io/api/v1/crates/basic-cookies/0.1.4/download",
        type = "tar.gz",
        sha256 = "cb53b6b315f924c7f113b162e53b3901c05fc9966baf84d201dfcc7432a4bb38",
        strip_prefix = "basic-cookies-0.1.4",
        build_file = Label("//third_party/cargo/remote:BUILD.basic-cookies-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__bit_set__0_5_2",
        url = "https://crates.io/api/v1/crates/bit-set/0.5.2/download",
        type = "tar.gz",
        sha256 = "6e11e16035ea35e4e5997b393eacbf6f63983188f7a2ad25bfb13465f5ad59de",
        strip_prefix = "bit-set-0.5.2",
        build_file = Label("//third_party/cargo/remote:BUILD.bit-set-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__bit_vec__0_6_3",
        url = "https://crates.io/api/v1/crates/bit-vec/0.6.3/download",
        type = "tar.gz",
        sha256 = "349f9b6a179ed607305526ca489b34ad0a41aed5f7980fa90eb03160b69598fb",
        strip_prefix = "bit-vec-0.6.3",
        build_file = Label("//third_party/cargo/remote:BUILD.bit-vec-0.6.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__bitflags__1_2_1",
        url = "https://crates.io/api/v1/crates/bitflags/1.2.1/download",
        type = "tar.gz",
        sha256 = "cf1de2fe8c75bc145a2f577add951f8134889b4795d47466a54a5c846d691693",
        strip_prefix = "bitflags-1.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.bitflags-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__blake2b_simd__0_5_11",
        url = "https://crates.io/api/v1/crates/blake2b_simd/0.5.11/download",
        type = "tar.gz",
        sha256 = "afa748e348ad3be8263be728124b24a24f268266f6f5d58af9d75f6a40b5c587",
        strip_prefix = "blake2b_simd-0.5.11",
        build_file = Label("//third_party/cargo/remote:BUILD.blake2b_simd-0.5.11.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__block_buffer__0_7_3",
        url = "https://crates.io/api/v1/crates/block-buffer/0.7.3/download",
        type = "tar.gz",
        sha256 = "c0940dc441f31689269e10ac70eb1002a3a1d3ad1390e030043662eb7fe4688b",
        strip_prefix = "block-buffer-0.7.3",
        build_file = Label("//third_party/cargo/remote:BUILD.block-buffer-0.7.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__block_padding__0_1_5",
        url = "https://crates.io/api/v1/crates/block-padding/0.1.5/download",
        type = "tar.gz",
        sha256 = "fa79dedbb091f449f1f39e53edf88d5dbe95f895dae6135a8d7b881fb5af73f5",
        strip_prefix = "block-padding-0.1.5",
        build_file = Label("//third_party/cargo/remote:BUILD.block-padding-0.1.5.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__blocking__1_0_2",
        url = "https://crates.io/api/v1/crates/blocking/1.0.2/download",
        type = "tar.gz",
        sha256 = "c5e170dbede1f740736619b776d7251cb1b9095c435c34d8ca9f57fcd2f335e9",
        strip_prefix = "blocking-1.0.2",
        build_file = Label("//third_party/cargo/remote:BUILD.blocking-1.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__bstr__0_2_14",
        url = "https://crates.io/api/v1/crates/bstr/0.2.14/download",
        type = "tar.gz",
        sha256 = "473fc6b38233f9af7baa94fb5852dca389e3d95b8e21c8e3719301462c5d9faf",
        strip_prefix = "bstr-0.2.14",
        build_file = Label("//third_party/cargo/remote:BUILD.bstr-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__bumpalo__3_4_0",
        url = "https://crates.io/api/v1/crates/bumpalo/3.4.0/download",
        type = "tar.gz",
        sha256 = "2e8c087f005730276d1096a652e92a8bacee2e2472bcc9715a74d2bec38b5820",
        strip_prefix = "bumpalo-3.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.bumpalo-3.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__byte_tools__0_3_1",
        url = "https://crates.io/api/v1/crates/byte-tools/0.3.1/download",
        type = "tar.gz",
        sha256 = "e3b5ca7a04898ad4bcd41c90c5285445ff5b791899bb1b0abdd2a2aa791211d7",
        strip_prefix = "byte-tools-0.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.byte-tools-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__byteorder__1_3_4",
        url = "https://crates.io/api/v1/crates/byteorder/1.3.4/download",
        type = "tar.gz",
        sha256 = "08c48aae112d48ed9f069b33538ea9e3e90aa263cfa3d1c24309612b1f7472de",
        strip_prefix = "byteorder-1.3.4",
        build_file = Label("//third_party/cargo/remote:BUILD.byteorder-1.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__bytes__0_5_6",
        url = "https://crates.io/api/v1/crates/bytes/0.5.6/download",
        type = "tar.gz",
        sha256 = "0e4cec68f03f32e44924783795810fa50a7035d8c8ebe78580ad7e6c703fba38",
        strip_prefix = "bytes-0.5.6",
        build_file = Label("//third_party/cargo/remote:BUILD.bytes-0.5.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cache_padded__1_1_1",
        url = "https://crates.io/api/v1/crates/cache-padded/1.1.1/download",
        type = "tar.gz",
        sha256 = "631ae5198c9be5e753e5cc215e1bd73c2b466a3565173db433f52bb9d3e66dba",
        strip_prefix = "cache-padded-1.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.cache-padded-1.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cargo_clone_crate__0_1_5",
        url = "https://crates.io/api/v1/crates/cargo-clone-crate/0.1.5/download",
        type = "tar.gz",
        sha256 = "063043e6a2cbca4f5505731e83e6cd61500934cd4df310c19040eb8fd1ec08d5",
        strip_prefix = "cargo-clone-crate-0.1.5",
        build_file = Label("//third_party/cargo/remote:BUILD.cargo-clone-crate-0.1.5.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cargo_lock__6_0_0",
        url = "https://crates.io/api/v1/crates/cargo-lock/6.0.0/download",
        type = "tar.gz",
        sha256 = "bad00408e56f778335802ea240b8d70bebf6ea6c43c7508ebb6259431b5f16c2",
        strip_prefix = "cargo-lock-6.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.cargo-lock-6.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cargo_platform__0_1_1",
        url = "https://crates.io/api/v1/crates/cargo-platform/0.1.1/download",
        type = "tar.gz",
        sha256 = "0226944a63d1bf35a3b5f948dd7c59e263db83695c9e8bffc4037de02e30f1d7",
        strip_prefix = "cargo-platform-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.cargo-platform-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cargo_metadata__0_12_1",
        url = "https://crates.io/api/v1/crates/cargo_metadata/0.12.1/download",
        type = "tar.gz",
        sha256 = "83f95cf4bf0dda0ac2e65371ae7215d0dce3c187613a9dbf23aaa9374186f97a",
        strip_prefix = "cargo_metadata-0.12.1",
        build_file = Label("//third_party/cargo/remote:BUILD.cargo_metadata-0.12.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cargo_toml__0_8_1",
        url = "https://crates.io/api/v1/crates/cargo_toml/0.8.1/download",
        type = "tar.gz",
        sha256 = "513d17226888c7b8283ac02a1c1b0d8a9d4cbf6db65dfadb79f598f5d7966fe9",
        strip_prefix = "cargo_toml-0.8.1",
        build_file = Label("//third_party/cargo/remote:BUILD.cargo_toml-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cc__1_0_66",
        url = "https://crates.io/api/v1/crates/cc/1.0.66/download",
        type = "tar.gz",
        sha256 = "4c0496836a84f8d0495758516b8621a622beb77c0fed418570e50764093ced48",
        strip_prefix = "cc-1.0.66",
        build_file = Label("//third_party/cargo/remote:BUILD.cc-1.0.66.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cfg_expr__0_5_1",
        url = "https://crates.io/api/v1/crates/cfg-expr/0.5.1/download",
        type = "tar.gz",
        sha256 = "cf3029692edb3cd77a7fb51dbbbdc4b550c958f7b2147cb7691e38b070f96b5d",
        strip_prefix = "cfg-expr-0.5.1",
        build_file = Label("//third_party/cargo/remote:BUILD.cfg-expr-0.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cfg_if__0_1_10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//third_party/cargo/remote:BUILD.cfg-if-0.1.10.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__cfg_if__1_0_0",
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download",
        type = "tar.gz",
        sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd",
        strip_prefix = "cfg-if-1.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.cfg-if-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__chrono__0_4_19",
        url = "https://crates.io/api/v1/crates/chrono/0.4.19/download",
        type = "tar.gz",
        sha256 = "670ad68c9088c2a963aaa298cb369688cf3f9465ce5e2d4ca10e6e0098a1ce73",
        strip_prefix = "chrono-0.4.19",
        build_file = Label("//third_party/cargo/remote:BUILD.chrono-0.4.19.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__chrono_tz__0_5_3",
        url = "https://crates.io/api/v1/crates/chrono-tz/0.5.3/download",
        type = "tar.gz",
        sha256 = "2554a3155fec064362507487171dcc4edc3df60cb10f3a1fb10ed8094822b120",
        strip_prefix = "chrono-tz-0.5.3",
        build_file = Label("//third_party/cargo/remote:BUILD.chrono-tz-0.5.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__concurrent_queue__1_2_2",
        url = "https://crates.io/api/v1/crates/concurrent-queue/1.2.2/download",
        type = "tar.gz",
        sha256 = "30ed07550be01594c6026cff2a1d7fe9c8f683caa798e12b68694ac9e88286a3",
        strip_prefix = "concurrent-queue-1.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.concurrent-queue-1.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__constant_time_eq__0_1_5",
        url = "https://crates.io/api/v1/crates/constant_time_eq/0.1.5/download",
        type = "tar.gz",
        sha256 = "245097e9a4535ee1e3e3931fcfcd55a796a44c643e8596ff6566d68f09b87bbc",
        strip_prefix = "constant_time_eq-0.1.5",
        build_file = Label("//third_party/cargo/remote:BUILD.constant_time_eq-0.1.5.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__core_foundation__0_9_1",
        url = "https://crates.io/api/v1/crates/core-foundation/0.9.1/download",
        type = "tar.gz",
        sha256 = "0a89e2ae426ea83155dccf10c0fa6b1463ef6d5fcb44cee0b224a408fa640a62",
        strip_prefix = "core-foundation-0.9.1",
        build_file = Label("//third_party/cargo/remote:BUILD.core-foundation-0.9.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__core_foundation_sys__0_8_2",
        url = "https://crates.io/api/v1/crates/core-foundation-sys/0.8.2/download",
        type = "tar.gz",
        sha256 = "ea221b5284a47e40033bf9b66f35f984ec0ea2931eb03505246cd27a963f981b",
        strip_prefix = "core-foundation-sys-0.8.2",
        build_file = Label("//third_party/cargo/remote:BUILD.core-foundation-sys-0.8.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__crates_index__0_16_2",
        url = "https://crates.io/api/v1/crates/crates-index/0.16.2/download",
        type = "tar.gz",
        sha256 = "7f24823d553339d125040d989d2a593a01b034fe5ac17714423bcd2c3d168878",
        strip_prefix = "crates-index-0.16.2",
        build_file = Label("//third_party/cargo/remote:BUILD.crates-index-0.16.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__crc32fast__1_2_1",
        url = "https://crates.io/api/v1/crates/crc32fast/1.2.1/download",
        type = "tar.gz",
        sha256 = "81156fece84ab6a9f2afdb109ce3ae577e42b1228441eded99bd77f627953b1a",
        strip_prefix = "crc32fast-1.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.crc32fast-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__crossbeam_utils__0_8_1",
        url = "https://crates.io/api/v1/crates/crossbeam-utils/0.8.1/download",
        type = "tar.gz",
        sha256 = "02d96d1e189ef58269ebe5b97953da3274d83a93af647c2ddd6f9dab28cedb8d",
        strip_prefix = "crossbeam-utils-0.8.1",
        build_file = Label("//third_party/cargo/remote:BUILD.crossbeam-utils-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__crunchy__0_2_2",
        url = "https://crates.io/api/v1/crates/crunchy/0.2.2/download",
        type = "tar.gz",
        sha256 = "7a81dae078cea95a014a339291cec439d2f232ebe854a9d672b796c6afafa9b7",
        strip_prefix = "crunchy-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.crunchy-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__curl__0_4_34",
        url = "https://crates.io/api/v1/crates/curl/0.4.34/download",
        type = "tar.gz",
        sha256 = "e268162af1a5fe89917ae25ba3b0a77c8da752bdc58e7dbb4f15b91fbd33756e",
        strip_prefix = "curl-0.4.34",
        build_file = Label("//third_party/cargo/remote:BUILD.curl-0.4.34.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__curl_sys__0_4_39_curl_7_74_0",
        url = "https://crates.io/api/v1/crates/curl-sys/0.4.39+curl-7.74.0/download",
        type = "tar.gz",
        sha256 = "07a8ce861e7b68a0b394e814d7ee9f1b2750ff8bd10372c6ad3bacc10e86f874",
        strip_prefix = "curl-sys-0.4.39+curl-7.74.0",
        build_file = Label("//third_party/cargo/remote:BUILD.curl-sys-0.4.39+curl-7.74.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__deunicode__0_4_3",
        url = "https://crates.io/api/v1/crates/deunicode/0.4.3/download",
        type = "tar.gz",
        sha256 = "850878694b7933ca4c9569d30a34b55031b9b139ee1fc7b94a527c4ef960d690",
        strip_prefix = "deunicode-0.4.3",
        build_file = Label("//third_party/cargo/remote:BUILD.deunicode-0.4.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__diff__0_1_12",
        url = "https://crates.io/api/v1/crates/diff/0.1.12/download",
        type = "tar.gz",
        sha256 = "0e25ea47919b1560c4e3b7fe0aaab9becf5b84a10325ddf7db0f0ba5e1026499",
        strip_prefix = "diff-0.1.12",
        build_file = Label("//third_party/cargo/remote:BUILD.diff-0.1.12.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__difference__2_0_0",
        url = "https://crates.io/api/v1/crates/difference/2.0.0/download",
        type = "tar.gz",
        sha256 = "524cbf6897b527295dff137cec09ecf3a05f4fddffd7dfcd1585403449e74198",
        strip_prefix = "difference-2.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.difference-2.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__digest__0_8_1",
        url = "https://crates.io/api/v1/crates/digest/0.8.1/download",
        type = "tar.gz",
        sha256 = "f3d0c8c8752312f9713efd397ff63acb9f85585afbf179282e720e7704954dd5",
        strip_prefix = "digest-0.8.1",
        build_file = Label("//third_party/cargo/remote:BUILD.digest-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__dirs__1_0_5",
        url = "https://crates.io/api/v1/crates/dirs/1.0.5/download",
        type = "tar.gz",
        sha256 = "3fd78930633bd1c6e35c4b42b1df7b0cbc6bc191146e512bb3bedf243fcc3901",
        strip_prefix = "dirs-1.0.5",
        build_file = Label("//third_party/cargo/remote:BUILD.dirs-1.0.5.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__docopt__1_1_0",
        url = "https://crates.io/api/v1/crates/docopt/1.1.0/download",
        type = "tar.gz",
        sha256 = "7f525a586d310c87df72ebcd98009e57f1cc030c8c268305287a476beb653969",
        strip_prefix = "docopt-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.docopt-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__either__1_6_1",
        url = "https://crates.io/api/v1/crates/either/1.6.1/download",
        type = "tar.gz",
        sha256 = "e78d4f1cc4ae33bbfc157ed5d5a5ef3bc29227303d595861deb238fcec4e9457",
        strip_prefix = "either-1.6.1",
        build_file = Label("//third_party/cargo/remote:BUILD.either-1.6.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ena__0_14_0",
        url = "https://crates.io/api/v1/crates/ena/0.14.0/download",
        type = "tar.gz",
        sha256 = "d7402b94a93c24e742487327a7cd839dc9d36fec9de9fb25b09f2dae459f36c3",
        strip_prefix = "ena-0.14.0",
        build_file = Label("//third_party/cargo/remote:BUILD.ena-0.14.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__encoding_rs__0_8_26",
        url = "https://crates.io/api/v1/crates/encoding_rs/0.8.26/download",
        type = "tar.gz",
        sha256 = "801bbab217d7f79c0062f4f7205b5d4427c6d1a7bd7aafdd1475f7c59d62b283",
        strip_prefix = "encoding_rs-0.8.26",
        build_file = Label("//third_party/cargo/remote:BUILD.encoding_rs-0.8.26.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__event_listener__2_5_1",
        url = "https://crates.io/api/v1/crates/event-listener/2.5.1/download",
        type = "tar.gz",
        sha256 = "f7531096570974c3a9dcf9e4b8e1cede1ec26cf5046219fb3b9d897503b9be59",
        strip_prefix = "event-listener-2.5.1",
        build_file = Label("//third_party/cargo/remote:BUILD.event-listener-2.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__extend__0_1_2",
        url = "https://crates.io/api/v1/crates/extend/0.1.2/download",
        type = "tar.gz",
        sha256 = "f47da3a72ec598d9c8937a7ebca8962a5c7a1f28444e38c2b33c771ba3f55f05",
        strip_prefix = "extend-0.1.2",
        build_file = Label("//third_party/cargo/remote:BUILD.extend-0.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__fake_simd__0_1_2",
        url = "https://crates.io/api/v1/crates/fake-simd/0.1.2/download",
        type = "tar.gz",
        sha256 = "e88a8acf291dafb59c2d96e8f59828f3838bb1a70398823ade51a84de6a6deed",
        strip_prefix = "fake-simd-0.1.2",
        build_file = Label("//third_party/cargo/remote:BUILD.fake-simd-0.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__fastrand__1_4_0",
        url = "https://crates.io/api/v1/crates/fastrand/1.4.0/download",
        type = "tar.gz",
        sha256 = "ca5faf057445ce5c9d4329e382b2ce7ca38550ef3b73a5348362d5f24e0c7fe3",
        strip_prefix = "fastrand-1.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.fastrand-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__filetime__0_2_13",
        url = "https://crates.io/api/v1/crates/filetime/0.2.13/download",
        type = "tar.gz",
        sha256 = "0c122a393ea57648015bf06fbd3d372378992e86b9ff5a7a497b076a28c79efe",
        strip_prefix = "filetime-0.2.13",
        build_file = Label("//third_party/cargo/remote:BUILD.filetime-0.2.13.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__fixedbitset__0_2_0",
        url = "https://crates.io/api/v1/crates/fixedbitset/0.2.0/download",
        type = "tar.gz",
        sha256 = "37ab347416e802de484e4d03c7316c48f1ecb56574dfd4a46a80f173ce1de04d",
        strip_prefix = "fixedbitset-0.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.fixedbitset-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__flate2__1_0_19",
        url = "https://crates.io/api/v1/crates/flate2/1.0.19/download",
        type = "tar.gz",
        sha256 = "7411863d55df97a419aa64cb4d2f167103ea9d767e2c54a1868b7ac3f6b47129",
        strip_prefix = "flate2-1.0.19",
        build_file = Label("//third_party/cargo/remote:BUILD.flate2-1.0.19.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__flume__0_9_2",
        url = "https://crates.io/api/v1/crates/flume/0.9.2/download",
        type = "tar.gz",
        sha256 = "1bebadab126f8120d410b677ed95eee4ba6eb7c6dd8e34a5ec88a08050e26132",
        strip_prefix = "flume-0.9.2",
        build_file = Label("//third_party/cargo/remote:BUILD.flume-0.9.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__fnv__1_0_7",
        url = "https://crates.io/api/v1/crates/fnv/1.0.7/download",
        type = "tar.gz",
        sha256 = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1",
        strip_prefix = "fnv-1.0.7",
        build_file = Label("//third_party/cargo/remote:BUILD.fnv-1.0.7.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__foreign_types__0_3_2",
        url = "https://crates.io/api/v1/crates/foreign-types/0.3.2/download",
        type = "tar.gz",
        sha256 = "f6f339eb8adc052cd2ca78910fda869aefa38d22d5cb648e6485e4d3fc06f3b1",
        strip_prefix = "foreign-types-0.3.2",
        build_file = Label("//third_party/cargo/remote:BUILD.foreign-types-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__foreign_types_shared__0_1_1",
        url = "https://crates.io/api/v1/crates/foreign-types-shared/0.1.1/download",
        type = "tar.gz",
        sha256 = "00b0228411908ca8685dba7fc2cdd70ec9990a6e753e89b6ac91a84c40fbaf4b",
        strip_prefix = "foreign-types-shared-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.foreign-types-shared-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__form_urlencoded__1_0_0",
        url = "https://crates.io/api/v1/crates/form_urlencoded/1.0.0/download",
        type = "tar.gz",
        sha256 = "ece68d15c92e84fa4f19d3780f1294e5ca82a78a6d515f1efaabcc144688be00",
        strip_prefix = "form_urlencoded-1.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.form_urlencoded-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__fuchsia_zircon__0_3_3",
        url = "https://crates.io/api/v1/crates/fuchsia-zircon/0.3.3/download",
        type = "tar.gz",
        sha256 = "2e9763c69ebaae630ba35f74888db465e49e259ba1bc0eda7d06f4a067615d82",
        strip_prefix = "fuchsia-zircon-0.3.3",
        build_file = Label("//third_party/cargo/remote:BUILD.fuchsia-zircon-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__fuchsia_zircon_sys__0_3_3",
        url = "https://crates.io/api/v1/crates/fuchsia-zircon-sys/0.3.3/download",
        type = "tar.gz",
        sha256 = "3dcaa9ae7725d12cdb85b3ad99a434db70b468c09ded17e012d86b5c1010f7a7",
        strip_prefix = "fuchsia-zircon-sys-0.3.3",
        build_file = Label("//third_party/cargo/remote:BUILD.fuchsia-zircon-sys-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_channel__0_3_8",
        url = "https://crates.io/api/v1/crates/futures-channel/0.3.8/download",
        type = "tar.gz",
        sha256 = "4b7109687aa4e177ef6fe84553af6280ef2778bdb7783ba44c9dc3399110fe64",
        strip_prefix = "futures-channel-0.3.8",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-channel-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_core__0_3_8",
        url = "https://crates.io/api/v1/crates/futures-core/0.3.8/download",
        type = "tar.gz",
        sha256 = "847ce131b72ffb13b6109a221da9ad97a64cbe48feb1028356b836b47b8f1748",
        strip_prefix = "futures-core-0.3.8",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-core-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_io__0_3_8",
        url = "https://crates.io/api/v1/crates/futures-io/0.3.8/download",
        type = "tar.gz",
        sha256 = "611834ce18aaa1bd13c4b374f5d653e1027cf99b6b502584ff8c9a64413b30bb",
        strip_prefix = "futures-io-0.3.8",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-io-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_lite__1_11_3",
        url = "https://crates.io/api/v1/crates/futures-lite/1.11.3/download",
        type = "tar.gz",
        sha256 = "b4481d0cd0de1d204a4fa55e7d45f07b1d958abcb06714b3446438e2eff695fb",
        strip_prefix = "futures-lite-1.11.3",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-lite-1.11.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_macro__0_3_8",
        url = "https://crates.io/api/v1/crates/futures-macro/0.3.8/download",
        type = "tar.gz",
        sha256 = "77408a692f1f97bcc61dc001d752e00643408fbc922e4d634c655df50d595556",
        strip_prefix = "futures-macro-0.3.8",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-macro-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_sink__0_3_8",
        url = "https://crates.io/api/v1/crates/futures-sink/0.3.8/download",
        type = "tar.gz",
        sha256 = "f878195a49cee50e006b02b93cf7e0a95a38ac7b776b4c4d9cc1207cd20fcb3d",
        strip_prefix = "futures-sink-0.3.8",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-sink-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_task__0_3_8",
        url = "https://crates.io/api/v1/crates/futures-task/0.3.8/download",
        type = "tar.gz",
        sha256 = "7c554eb5bf48b2426c4771ab68c6b14468b6e76cc90996f528c3338d761a4d0d",
        strip_prefix = "futures-task-0.3.8",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-task-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__futures_util__0_3_8",
        url = "https://crates.io/api/v1/crates/futures-util/0.3.8/download",
        type = "tar.gz",
        sha256 = "d304cff4a7b99cfb7986f7d43fbe93d175e72e704a8860787cc95e9ffd85cbd2",
        strip_prefix = "futures-util-0.3.8",
        build_file = Label("//third_party/cargo/remote:BUILD.futures-util-0.3.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__generic_array__0_12_3",
        url = "https://crates.io/api/v1/crates/generic-array/0.12.3/download",
        type = "tar.gz",
        sha256 = "c68f0274ae0e023facc3c97b2e00f076be70e254bc851d972503b328db79b2ec",
        strip_prefix = "generic-array-0.12.3",
        build_file = Label("//third_party/cargo/remote:BUILD.generic-array-0.12.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__getrandom__0_1_16",
        url = "https://crates.io/api/v1/crates/getrandom/0.1.16/download",
        type = "tar.gz",
        sha256 = "8fc3cb4d91f53b50155bdcfd23f6a4c39ae1969c2ae85982b135750cccaf5fce",
        strip_prefix = "getrandom-0.1.16",
        build_file = Label("//third_party/cargo/remote:BUILD.getrandom-0.1.16.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__getrandom__0_2_1",
        url = "https://crates.io/api/v1/crates/getrandom/0.2.1/download",
        type = "tar.gz",
        sha256 = "4060f4657be78b8e766215b02b18a2e862d83745545de804638e2b545e81aee6",
        strip_prefix = "getrandom-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.getrandom-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__git2__0_13_15",
        url = "https://crates.io/api/v1/crates/git2/0.13.15/download",
        type = "tar.gz",
        sha256 = "44f267c9da8a4de3c615b59e23606c75f164f84896e97f4dd6c15a4294de4359",
        strip_prefix = "git2-0.13.15",
        build_file = Label("//third_party/cargo/remote:BUILD.git2-0.13.15.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__glob__0_3_0",
        url = "https://crates.io/api/v1/crates/glob/0.3.0/download",
        type = "tar.gz",
        sha256 = "9b919933a397b79c37e33b77bb2aa3dc8eb6e165ad809e58ff75bc7db2e34574",
        strip_prefix = "glob-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.glob-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__globset__0_4_6",
        url = "https://crates.io/api/v1/crates/globset/0.4.6/download",
        type = "tar.gz",
        sha256 = "c152169ef1e421390738366d2f796655fec62621dabbd0fd476f905934061e4a",
        strip_prefix = "globset-0.4.6",
        build_file = Label("//third_party/cargo/remote:BUILD.globset-0.4.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__globwalk__0_8_1",
        url = "https://crates.io/api/v1/crates/globwalk/0.8.1/download",
        type = "tar.gz",
        sha256 = "93e3af942408868f6934a7b85134a3230832b9977cf66125df2f9edcfce4ddcc",
        strip_prefix = "globwalk-0.8.1",
        build_file = Label("//third_party/cargo/remote:BUILD.globwalk-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__gloo_timers__0_2_1",
        url = "https://crates.io/api/v1/crates/gloo-timers/0.2.1/download",
        type = "tar.gz",
        sha256 = "47204a46aaff920a1ea58b11d03dec6f704287d27561724a4631e450654a891f",
        strip_prefix = "gloo-timers-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.gloo-timers-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__h2__0_2_7",
        url = "https://crates.io/api/v1/crates/h2/0.2.7/download",
        type = "tar.gz",
        sha256 = "5e4728fd124914ad25e99e3d15a9361a879f6620f63cb56bbb08f95abb97a535",
        strip_prefix = "h2-0.2.7",
        build_file = Label("//third_party/cargo/remote:BUILD.h2-0.2.7.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__hamcrest2__0_3_0",
        url = "https://crates.io/api/v1/crates/hamcrest2/0.3.0/download",
        type = "tar.gz",
        sha256 = "49f837c62de05dc9cc71ff6486cd85de8856a330395ae338a04bfcefe5e91075",
        strip_prefix = "hamcrest2-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.hamcrest2-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__hashbrown__0_9_1",
        url = "https://crates.io/api/v1/crates/hashbrown/0.9.1/download",
        type = "tar.gz",
        sha256 = "d7afe4a420e3fe79967a00898cc1f4db7c8a49a9333a29f8a4bd76a253d5cd04",
        strip_prefix = "hashbrown-0.9.1",
        build_file = Label("//third_party/cargo/remote:BUILD.hashbrown-0.9.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__hermit_abi__0_1_17",
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.17/download",
        type = "tar.gz",
        sha256 = "5aca5565f760fb5b220e499d72710ed156fdb74e631659e99377d9ebfbd13ae8",
        strip_prefix = "hermit-abi-0.1.17",
        build_file = Label("//third_party/cargo/remote:BUILD.hermit-abi-0.1.17.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__hex__0_4_2",
        url = "https://crates.io/api/v1/crates/hex/0.4.2/download",
        type = "tar.gz",
        sha256 = "644f9158b2f133fd50f5fb3242878846d9eb792e445c893805ff0e3824006e35",
        strip_prefix = "hex-0.4.2",
        build_file = Label("//third_party/cargo/remote:BUILD.hex-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__home__0_5_3",
        url = "https://crates.io/api/v1/crates/home/0.5.3/download",
        type = "tar.gz",
        sha256 = "2456aef2e6b6a9784192ae780c0f15bc57df0e918585282325e8c8ac27737654",
        strip_prefix = "home-0.5.3",
        build_file = Label("//third_party/cargo/remote:BUILD.home-0.5.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__http__0_2_2",
        url = "https://crates.io/api/v1/crates/http/0.2.2/download",
        type = "tar.gz",
        sha256 = "84129d298a6d57d246960ff8eb831ca4af3f96d29e2e28848dae275408658e26",
        strip_prefix = "http-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.http-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__http_body__0_3_1",
        url = "https://crates.io/api/v1/crates/http-body/0.3.1/download",
        type = "tar.gz",
        sha256 = "13d5ff830006f7646652e057693569bfe0d51760c0085a071769d142a205111b",
        strip_prefix = "http-body-0.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.http-body-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__httparse__1_3_4",
        url = "https://crates.io/api/v1/crates/httparse/1.3.4/download",
        type = "tar.gz",
        sha256 = "cd179ae861f0c2e53da70d892f5f3029f9594be0c41dc5269cd371691b1dc2f9",
        strip_prefix = "httparse-1.3.4",
        build_file = Label("//third_party/cargo/remote:BUILD.httparse-1.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__httpdate__0_3_2",
        url = "https://crates.io/api/v1/crates/httpdate/0.3.2/download",
        type = "tar.gz",
        sha256 = "494b4d60369511e7dea41cf646832512a94e542f68bb9c49e54518e0f468eb47",
        strip_prefix = "httpdate-0.3.2",
        build_file = Label("//third_party/cargo/remote:BUILD.httpdate-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__httpmock__0_5_2",
        url = "https://crates.io/api/v1/crates/httpmock/0.5.2/download",
        type = "tar.gz",
        sha256 = "f1e098238acaa95e0bb3fd357317817fde35efb9cdedf74b18b24698499eea9d",
        strip_prefix = "httpmock-0.5.2",
        build_file = Label("//third_party/cargo/remote:BUILD.httpmock-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__humansize__1_1_0",
        url = "https://crates.io/api/v1/crates/humansize/1.1.0/download",
        type = "tar.gz",
        sha256 = "b6cab2627acfc432780848602f3f558f7e9dd427352224b0d9324025796d2a5e",
        strip_prefix = "humansize-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.humansize-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__hyper__0_13_9",
        url = "https://crates.io/api/v1/crates/hyper/0.13.9/download",
        type = "tar.gz",
        sha256 = "f6ad767baac13b44d4529fcf58ba2cd0995e36e7b435bc5b039de6f47e880dbf",
        strip_prefix = "hyper-0.13.9",
        build_file = Label("//third_party/cargo/remote:BUILD.hyper-0.13.9.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__hyper_tls__0_4_3",
        url = "https://crates.io/api/v1/crates/hyper-tls/0.4.3/download",
        type = "tar.gz",
        sha256 = "d979acc56dcb5b8dddba3917601745e877576475aa046df3226eabdecef78eed",
        strip_prefix = "hyper-tls-0.4.3",
        build_file = Label("//third_party/cargo/remote:BUILD.hyper-tls-0.4.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__idna__0_2_0",
        url = "https://crates.io/api/v1/crates/idna/0.2.0/download",
        type = "tar.gz",
        sha256 = "02e2673c30ee86b5b96a9cb52ad15718aa1f966f5ab9ad54a8b95d5ca33120a9",
        strip_prefix = "idna-0.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.idna-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ignore__0_4_17",
        url = "https://crates.io/api/v1/crates/ignore/0.4.17/download",
        type = "tar.gz",
        sha256 = "b287fb45c60bb826a0dc68ff08742b9d88a2fea13d6e0c286b3172065aaf878c",
        strip_prefix = "ignore-0.4.17",
        build_file = Label("//third_party/cargo/remote:BUILD.ignore-0.4.17.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__indexmap__1_6_1",
        url = "https://crates.io/api/v1/crates/indexmap/1.6.1/download",
        type = "tar.gz",
        sha256 = "4fb1fa934250de4de8aef298d81c729a7d33d8c239daa3a7575e6b92bfc7313b",
        strip_prefix = "indexmap-1.6.1",
        build_file = Label("//third_party/cargo/remote:BUILD.indexmap-1.6.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__indoc__1_0_3",
        url = "https://crates.io/api/v1/crates/indoc/1.0.3/download",
        type = "tar.gz",
        sha256 = "e5a75aeaaef0ce18b58056d306c27b07436fbb34b8816c53094b76dd81803136",
        strip_prefix = "indoc-1.0.3",
        build_file = Label("//third_party/cargo/remote:BUILD.indoc-1.0.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__instant__0_1_9",
        url = "https://crates.io/api/v1/crates/instant/0.1.9/download",
        type = "tar.gz",
        sha256 = "61124eeebbd69b8190558df225adf7e4caafce0d743919e5d6b19652314ec5ec",
        strip_prefix = "instant-0.1.9",
        build_file = Label("//third_party/cargo/remote:BUILD.instant-0.1.9.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__iovec__0_1_4",
        url = "https://crates.io/api/v1/crates/iovec/0.1.4/download",
        type = "tar.gz",
        sha256 = "b2b3ea6ff95e175473f8ffe6a7eb7c00d054240321b84c57051175fe3c1e075e",
        strip_prefix = "iovec-0.1.4",
        build_file = Label("//third_party/cargo/remote:BUILD.iovec-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ipnet__2_3_0",
        url = "https://crates.io/api/v1/crates/ipnet/2.3.0/download",
        type = "tar.gz",
        sha256 = "47be2f14c678be2fdcab04ab1171db51b2762ce6f0a8ee87c8dd4a04ed216135",
        strip_prefix = "ipnet-2.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.ipnet-2.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__isahc__0_9_14",
        url = "https://crates.io/api/v1/crates/isahc/0.9.14/download",
        type = "tar.gz",
        sha256 = "e2948a0ce43e2c2ef11d7edf6816508998d99e13badd1150be0914205df9388a",
        strip_prefix = "isahc-0.9.14",
        build_file = Label("//third_party/cargo/remote:BUILD.isahc-0.9.14.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__itertools__0_10_0",
        url = "https://crates.io/api/v1/crates/itertools/0.10.0/download",
        type = "tar.gz",
        sha256 = "37d572918e350e82412fe766d24b15e6682fb2ed2bbe018280caa810397cb319",
        strip_prefix = "itertools-0.10.0",
        build_file = Label("//third_party/cargo/remote:BUILD.itertools-0.10.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__itertools__0_9_0",
        url = "https://crates.io/api/v1/crates/itertools/0.9.0/download",
        type = "tar.gz",
        sha256 = "284f18f85651fe11e8a991b2adb42cb078325c996ed026d994719efcfca1d54b",
        strip_prefix = "itertools-0.9.0",
        build_file = Label("//third_party/cargo/remote:BUILD.itertools-0.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__itoa__0_4_7",
        url = "https://crates.io/api/v1/crates/itoa/0.4.7/download",
        type = "tar.gz",
        sha256 = "dd25036021b0de88a0aff6b850051563c6516d0bf53f8638938edbb9de732736",
        strip_prefix = "itoa-0.4.7",
        build_file = Label("//third_party/cargo/remote:BUILD.itoa-0.4.7.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__jobserver__0_1_21",
        url = "https://crates.io/api/v1/crates/jobserver/0.1.21/download",
        type = "tar.gz",
        sha256 = "5c71313ebb9439f74b00d9d2dcec36440beaf57a6aa0623068441dd7cd81a7f2",
        strip_prefix = "jobserver-0.1.21",
        build_file = Label("//third_party/cargo/remote:BUILD.jobserver-0.1.21.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__js_sys__0_3_46",
        url = "https://crates.io/api/v1/crates/js-sys/0.3.46/download",
        type = "tar.gz",
        sha256 = "cf3d7383929f7c9c7c2d0fa596f325832df98c3704f2c60553080f7127a58175",
        strip_prefix = "js-sys-0.3.46",
        build_file = Label("//third_party/cargo/remote:BUILD.js-sys-0.3.46.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__kernel32_sys__0_2_2",
        url = "https://crates.io/api/v1/crates/kernel32-sys/0.2.2/download",
        type = "tar.gz",
        sha256 = "7507624b29483431c0ba2d82aece8ca6cdba9382bff4ddd0f7490560c056098d",
        strip_prefix = "kernel32-sys-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.kernel32-sys-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__kv_log_macro__1_0_7",
        url = "https://crates.io/api/v1/crates/kv-log-macro/1.0.7/download",
        type = "tar.gz",
        sha256 = "0de8b303297635ad57c9f5059fd9cee7a47f8e8daa09df0fcd07dd39fb22977f",
        strip_prefix = "kv-log-macro-1.0.7",
        build_file = Label("//third_party/cargo/remote:BUILD.kv-log-macro-1.0.7.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__lalrpop__0_19_3",
        url = "https://crates.io/api/v1/crates/lalrpop/0.19.3/download",
        type = "tar.gz",
        sha256 = "5c64e04d8ea9c95c2135dfc4298088eafaf956bc90ba372eb1bea4f715634587",
        strip_prefix = "lalrpop-0.19.3",
        build_file = Label("//third_party/cargo/remote:BUILD.lalrpop-0.19.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__lalrpop_util__0_19_3",
        url = "https://crates.io/api/v1/crates/lalrpop-util/0.19.3/download",
        type = "tar.gz",
        sha256 = "f9de203e2fa3e883364fcc778a1293ab4d936f6cff400433013c20105df178c5",
        strip_prefix = "lalrpop-util-0.19.3",
        build_file = Label("//third_party/cargo/remote:BUILD.lalrpop-util-0.19.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.lazy_static-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__levenshtein__1_0_4",
        url = "https://crates.io/api/v1/crates/levenshtein/1.0.4/download",
        type = "tar.gz",
        sha256 = "66189c12161c65c0023ceb53e2fccc0013311bcb36a7cbd0f9c5e938b408ac96",
        strip_prefix = "levenshtein-1.0.4",
        build_file = Label("//third_party/cargo/remote:BUILD.levenshtein-1.0.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__libc__0_2_81",
        url = "https://crates.io/api/v1/crates/libc/0.2.81/download",
        type = "tar.gz",
        sha256 = "1482821306169ec4d07f6aca392a4681f66c75c9918aa49641a2595db64053cb",
        strip_prefix = "libc-0.2.81",
        build_file = Label("//third_party/cargo/remote:BUILD.libc-0.2.81.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__libgit2_sys__0_12_17_1_1_0",
        url = "https://crates.io/api/v1/crates/libgit2-sys/0.12.17+1.1.0/download",
        type = "tar.gz",
        sha256 = "f4ebdf65ca745126df8824688637aa0535a88900b83362d8ca63893bcf4e8841",
        strip_prefix = "libgit2-sys-0.12.17+1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.libgit2-sys-0.12.17+1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__libnghttp2_sys__0_1_5_1_42_0",
        url = "https://crates.io/api/v1/crates/libnghttp2-sys/0.1.5+1.42.0/download",
        type = "tar.gz",
        sha256 = "9657455ff47889b70ffd37c3e118e8cdd23fd1f9f3293a285f141070621c4c79",
        strip_prefix = "libnghttp2-sys-0.1.5+1.42.0",
        build_file = Label("//third_party/cargo/remote:BUILD.libnghttp2-sys-0.1.5+1.42.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__libssh2_sys__0_2_20",
        url = "https://crates.io/api/v1/crates/libssh2-sys/0.2.20/download",
        type = "tar.gz",
        sha256 = "df40b13fe7ea1be9b9dffa365a51273816c345fc1811478b57ed7d964fbfc4ce",
        strip_prefix = "libssh2-sys-0.2.20",
        build_file = Label("//third_party/cargo/remote:BUILD.libssh2-sys-0.2.20.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__libz_sys__1_1_2",
        url = "https://crates.io/api/v1/crates/libz-sys/1.1.2/download",
        type = "tar.gz",
        sha256 = "602113192b08db8f38796c4e85c39e960c145965140e918018bcde1952429655",
        strip_prefix = "libz-sys-1.1.2",
        build_file = Label("//third_party/cargo/remote:BUILD.libz-sys-1.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__lock_api__0_4_2",
        url = "https://crates.io/api/v1/crates/lock_api/0.4.2/download",
        type = "tar.gz",
        sha256 = "dd96ffd135b2fd7b973ac026d28085defbe8983df057ced3eb4f2130b0831312",
        strip_prefix = "lock_api-0.4.2",
        build_file = Label("//third_party/cargo/remote:BUILD.lock_api-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__log__0_4_11",
        url = "https://crates.io/api/v1/crates/log/0.4.11/download",
        type = "tar.gz",
        sha256 = "4fabed175da42fed1fa0746b0ea71f412aa9d35e76e95e59b192c64b9dc2bf8b",
        strip_prefix = "log-0.4.11",
        build_file = Label("//third_party/cargo/remote:BUILD.log-0.4.11.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__maplit__1_0_2",
        url = "https://crates.io/api/v1/crates/maplit/1.0.2/download",
        type = "tar.gz",
        sha256 = "3e2e65a1a2e43cfcb47a895c4c8b10d1f4a61097f9f254f183aee60cad9c651d",
        strip_prefix = "maplit-1.0.2",
        build_file = Label("//third_party/cargo/remote:BUILD.maplit-1.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__matches__0_1_8",
        url = "https://crates.io/api/v1/crates/matches/0.1.8/download",
        type = "tar.gz",
        sha256 = "7ffc5c5338469d4d3ea17d269fa8ea3512ad247247c30bd2df69e68309ed0a08",
        strip_prefix = "matches-0.1.8",
        build_file = Label("//third_party/cargo/remote:BUILD.matches-0.1.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__memchr__2_3_4",
        url = "https://crates.io/api/v1/crates/memchr/2.3.4/download",
        type = "tar.gz",
        sha256 = "0ee1c47aaa256ecabcaea351eae4a9b01ef39ed810004e298d2511ed284b1525",
        strip_prefix = "memchr-2.3.4",
        build_file = Label("//third_party/cargo/remote:BUILD.memchr-2.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__mime__0_3_16",
        url = "https://crates.io/api/v1/crates/mime/0.3.16/download",
        type = "tar.gz",
        sha256 = "2a60c7ce501c71e03a9c9c0d35b861413ae925bd979cc7a4e30d060069aaac8d",
        strip_prefix = "mime-0.3.16",
        build_file = Label("//third_party/cargo/remote:BUILD.mime-0.3.16.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__mime_guess__2_0_3",
        url = "https://crates.io/api/v1/crates/mime_guess/2.0.3/download",
        type = "tar.gz",
        sha256 = "2684d4c2e97d99848d30b324b00c8fcc7e5c897b7cbb5819b09e7c90e8baf212",
        strip_prefix = "mime_guess-2.0.3",
        build_file = Label("//third_party/cargo/remote:BUILD.mime_guess-2.0.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__miniz_oxide__0_4_3",
        url = "https://crates.io/api/v1/crates/miniz_oxide/0.4.3/download",
        type = "tar.gz",
        sha256 = "0f2d26ec3309788e423cfbf68ad1800f061638098d76a83681af979dc4eda19d",
        strip_prefix = "miniz_oxide-0.4.3",
        build_file = Label("//third_party/cargo/remote:BUILD.miniz_oxide-0.4.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__mio__0_6_23",
        url = "https://crates.io/api/v1/crates/mio/0.6.23/download",
        type = "tar.gz",
        sha256 = "4afd66f5b91bf2a3bc13fad0e21caedac168ca4c707504e75585648ae80e4cc4",
        strip_prefix = "mio-0.6.23",
        build_file = Label("//third_party/cargo/remote:BUILD.mio-0.6.23.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__miow__0_2_2",
        url = "https://crates.io/api/v1/crates/miow/0.2.2/download",
        type = "tar.gz",
        sha256 = "ebd808424166322d4a38da87083bfddd3ac4c131334ed55856112eb06d46944d",
        strip_prefix = "miow-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.miow-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__native_tls__0_2_7",
        url = "https://crates.io/api/v1/crates/native-tls/0.2.7/download",
        type = "tar.gz",
        sha256 = "b8d96b2e1c8da3957d58100b09f102c6d9cfdfced01b7ec5a8974044bb09dbd4",
        strip_prefix = "native-tls-0.2.7",
        build_file = Label("//third_party/cargo/remote:BUILD.native-tls-0.2.7.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__nb_connect__1_0_2",
        url = "https://crates.io/api/v1/crates/nb-connect/1.0.2/download",
        type = "tar.gz",
        sha256 = "8123a81538e457d44b933a02faf885d3fe8408806b23fa700e8f01c6c3a98998",
        strip_prefix = "nb-connect-1.0.2",
        build_file = Label("//third_party/cargo/remote:BUILD.nb-connect-1.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__net2__0_2_37",
        url = "https://crates.io/api/v1/crates/net2/0.2.37/download",
        type = "tar.gz",
        sha256 = "391630d12b68002ae1e25e8f974306474966550ad82dac6886fb8910c19568ae",
        strip_prefix = "net2-0.2.37",
        build_file = Label("//third_party/cargo/remote:BUILD.net2-0.2.37.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__new_debug_unreachable__1_0_4",
        url = "https://crates.io/api/v1/crates/new_debug_unreachable/1.0.4/download",
        type = "tar.gz",
        sha256 = "e4a24736216ec316047a1fc4252e27dabb04218aa4a3f37c6e7ddbf1f9782b54",
        strip_prefix = "new_debug_unreachable-1.0.4",
        build_file = Label("//third_party/cargo/remote:BUILD.new_debug_unreachable-1.0.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num__0_2_1",
        url = "https://crates.io/api/v1/crates/num/0.2.1/download",
        type = "tar.gz",
        sha256 = "b8536030f9fea7127f841b45bb6243b27255787fb4eb83958aa1ef9d2fdc0c36",
        strip_prefix = "num-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.num-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num_bigint__0_2_6",
        url = "https://crates.io/api/v1/crates/num-bigint/0.2.6/download",
        type = "tar.gz",
        sha256 = "090c7f9998ee0ff65aa5b723e4009f7b217707f1fb5ea551329cc4d6231fb304",
        strip_prefix = "num-bigint-0.2.6",
        build_file = Label("//third_party/cargo/remote:BUILD.num-bigint-0.2.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num_complex__0_2_4",
        url = "https://crates.io/api/v1/crates/num-complex/0.2.4/download",
        type = "tar.gz",
        sha256 = "b6b19411a9719e753aff12e5187b74d60d3dc449ec3f4dc21e3989c3f554bc95",
        strip_prefix = "num-complex-0.2.4",
        build_file = Label("//third_party/cargo/remote:BUILD.num-complex-0.2.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num_integer__0_1_44",
        url = "https://crates.io/api/v1/crates/num-integer/0.1.44/download",
        type = "tar.gz",
        sha256 = "d2cc698a63b549a70bc047073d2949cce27cd1c7b0a4a862d08a8031bc2801db",
        strip_prefix = "num-integer-0.1.44",
        build_file = Label("//third_party/cargo/remote:BUILD.num-integer-0.1.44.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num_iter__0_1_42",
        url = "https://crates.io/api/v1/crates/num-iter/0.1.42/download",
        type = "tar.gz",
        sha256 = "b2021c8337a54d21aca0d59a92577a029af9431cb59b909b03252b9c164fad59",
        strip_prefix = "num-iter-0.1.42",
        build_file = Label("//third_party/cargo/remote:BUILD.num-iter-0.1.42.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num_rational__0_2_4",
        url = "https://crates.io/api/v1/crates/num-rational/0.2.4/download",
        type = "tar.gz",
        sha256 = "5c000134b5dbf44adc5cb772486d335293351644b801551abe8f75c84cfa4aef",
        strip_prefix = "num-rational-0.2.4",
        build_file = Label("//third_party/cargo/remote:BUILD.num-rational-0.2.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num_traits__0_2_14",
        url = "https://crates.io/api/v1/crates/num-traits/0.2.14/download",
        type = "tar.gz",
        sha256 = "9a64b1ec5cda2586e284722486d802acf1f7dbdc623e2bfc57e65ca1cd099290",
        strip_prefix = "num-traits-0.2.14",
        build_file = Label("//third_party/cargo/remote:BUILD.num-traits-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__num_cpus__1_13_0",
        url = "https://crates.io/api/v1/crates/num_cpus/1.13.0/download",
        type = "tar.gz",
        sha256 = "05499f3756671c15885fee9034446956fff3f243d6077b91e5767df161f766b3",
        strip_prefix = "num_cpus-1.13.0",
        build_file = Label("//third_party/cargo/remote:BUILD.num_cpus-1.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__once_cell__1_5_2",
        url = "https://crates.io/api/v1/crates/once_cell/1.5.2/download",
        type = "tar.gz",
        sha256 = "13bd41f508810a131401606d54ac32a467c97172d74ba7662562ebba5ad07fa0",
        strip_prefix = "once_cell-1.5.2",
        build_file = Label("//third_party/cargo/remote:BUILD.once_cell-1.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__opaque_debug__0_2_3",
        url = "https://crates.io/api/v1/crates/opaque-debug/0.2.3/download",
        type = "tar.gz",
        sha256 = "2839e79665f131bdb5782e51f2c6c9599c133c6098982a54c794358bf432529c",
        strip_prefix = "opaque-debug-0.2.3",
        build_file = Label("//third_party/cargo/remote:BUILD.opaque-debug-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__openssl__0_10_32",
        url = "https://crates.io/api/v1/crates/openssl/0.10.32/download",
        type = "tar.gz",
        sha256 = "038d43985d1ddca7a9900630d8cd031b56e4794eecc2e9ea39dd17aa04399a70",
        strip_prefix = "openssl-0.10.32",
        build_file = Label("//third_party/cargo/remote:BUILD.openssl-0.10.32.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__openssl_probe__0_1_2",
        url = "https://crates.io/api/v1/crates/openssl-probe/0.1.2/download",
        type = "tar.gz",
        sha256 = "77af24da69f9d9341038eba93a073b1fdaaa1b788221b00a69bce9e762cb32de",
        strip_prefix = "openssl-probe-0.1.2",
        build_file = Label("//third_party/cargo/remote:BUILD.openssl-probe-0.1.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__openssl_sys__0_9_60",
        url = "https://crates.io/api/v1/crates/openssl-sys/0.9.60/download",
        type = "tar.gz",
        sha256 = "921fc71883267538946025deffb622905ecad223c28efbfdef9bb59a0175f3e6",
        strip_prefix = "openssl-sys-0.9.60",
        build_file = Label("//third_party/cargo/remote:BUILD.openssl-sys-0.9.60.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__parking__2_0_0",
        url = "https://crates.io/api/v1/crates/parking/2.0.0/download",
        type = "tar.gz",
        sha256 = "427c3892f9e783d91cc128285287e70a59e206ca452770ece88a76f7a3eddd72",
        strip_prefix = "parking-2.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.parking-2.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__parse_zoneinfo__0_3_0",
        url = "https://crates.io/api/v1/crates/parse-zoneinfo/0.3.0/download",
        type = "tar.gz",
        sha256 = "c705f256449c60da65e11ff6626e0c16a0a0b96aaa348de61376b249bc340f41",
        strip_prefix = "parse-zoneinfo-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.parse-zoneinfo-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pathdiff__0_2_0",
        url = "https://crates.io/api/v1/crates/pathdiff/0.2.0/download",
        type = "tar.gz",
        sha256 = "877630b3de15c0b64cc52f659345724fbf6bdad9bd9566699fc53688f3c34a34",
        strip_prefix = "pathdiff-0.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.pathdiff-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__percent_encoding__2_1_0",
        url = "https://crates.io/api/v1/crates/percent-encoding/2.1.0/download",
        type = "tar.gz",
        sha256 = "d4fd5641d01c8f18a23da7b6fe29298ff4b55afcccdf78973b24cf3175fee32e",
        strip_prefix = "percent-encoding-2.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.percent-encoding-2.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pest__2_1_3",
        url = "https://crates.io/api/v1/crates/pest/2.1.3/download",
        type = "tar.gz",
        sha256 = "10f4872ae94d7b90ae48754df22fd42ad52ce740b8f370b03da4835417403e53",
        strip_prefix = "pest-2.1.3",
        build_file = Label("//third_party/cargo/remote:BUILD.pest-2.1.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pest_derive__2_1_0",
        url = "https://crates.io/api/v1/crates/pest_derive/2.1.0/download",
        type = "tar.gz",
        sha256 = "833d1ae558dc601e9a60366421196a8d94bc0ac980476d0b67e1d0988d72b2d0",
        strip_prefix = "pest_derive-2.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.pest_derive-2.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pest_generator__2_1_3",
        url = "https://crates.io/api/v1/crates/pest_generator/2.1.3/download",
        type = "tar.gz",
        sha256 = "99b8db626e31e5b81787b9783425769681b347011cc59471e33ea46d2ea0cf55",
        strip_prefix = "pest_generator-2.1.3",
        build_file = Label("//third_party/cargo/remote:BUILD.pest_generator-2.1.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pest_meta__2_1_3",
        url = "https://crates.io/api/v1/crates/pest_meta/2.1.3/download",
        type = "tar.gz",
        sha256 = "54be6e404f5317079812fc8f9f5279de376d8856929e21c184ecf6bbd692a11d",
        strip_prefix = "pest_meta-2.1.3",
        build_file = Label("//third_party/cargo/remote:BUILD.pest_meta-2.1.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__petgraph__0_5_1",
        url = "https://crates.io/api/v1/crates/petgraph/0.5.1/download",
        type = "tar.gz",
        sha256 = "467d164a6de56270bd7c4d070df81d07beace25012d5103ced4e9ff08d6afdb7",
        strip_prefix = "petgraph-0.5.1",
        build_file = Label("//third_party/cargo/remote:BUILD.petgraph-0.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__phf_shared__0_8_0",
        url = "https://crates.io/api/v1/crates/phf_shared/0.8.0/download",
        type = "tar.gz",
        sha256 = "c00cf8b9eafe68dde5e9eaa2cef8ee84a9336a47d566ec55ca16589633b65af7",
        strip_prefix = "phf_shared-0.8.0",
        build_file = Label("//third_party/cargo/remote:BUILD.phf_shared-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pico_args__0_3_4",
        url = "https://crates.io/api/v1/crates/pico-args/0.3.4/download",
        type = "tar.gz",
        sha256 = "28b9b4df73455c861d7cbf8be42f01d3b373ed7f02e378d55fa84eafc6f638b1",
        strip_prefix = "pico-args-0.3.4",
        build_file = Label("//third_party/cargo/remote:BUILD.pico-args-0.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pin_project__0_4_27",
        url = "https://crates.io/api/v1/crates/pin-project/0.4.27/download",
        type = "tar.gz",
        sha256 = "2ffbc8e94b38ea3d2d8ba92aea2983b503cd75d0888d75b86bb37970b5698e15",
        strip_prefix = "pin-project-0.4.27",
        build_file = Label("//third_party/cargo/remote:BUILD.pin-project-0.4.27.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pin_project__1_0_3",
        url = "https://crates.io/api/v1/crates/pin-project/1.0.3/download",
        type = "tar.gz",
        sha256 = "5a83804639aad6ba65345661744708855f9fbcb71176ea8d28d05aeb11d975e7",
        strip_prefix = "pin-project-1.0.3",
        build_file = Label("//third_party/cargo/remote:BUILD.pin-project-1.0.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pin_project_internal__0_4_27",
        url = "https://crates.io/api/v1/crates/pin-project-internal/0.4.27/download",
        type = "tar.gz",
        sha256 = "65ad2ae56b6abe3a1ee25f15ee605bacadb9a764edaba9c2bf4103800d4a1895",
        strip_prefix = "pin-project-internal-0.4.27",
        build_file = Label("//third_party/cargo/remote:BUILD.pin-project-internal-0.4.27.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pin_project_internal__1_0_3",
        url = "https://crates.io/api/v1/crates/pin-project-internal/1.0.3/download",
        type = "tar.gz",
        sha256 = "b7bcc46b8f73443d15bc1c5fecbb315718491fa9187fa483f0e359323cde8b3a",
        strip_prefix = "pin-project-internal-1.0.3",
        build_file = Label("//third_party/cargo/remote:BUILD.pin-project-internal-1.0.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pin_project_lite__0_1_11",
        url = "https://crates.io/api/v1/crates/pin-project-lite/0.1.11/download",
        type = "tar.gz",
        sha256 = "c917123afa01924fc84bb20c4c03f004d9c38e5127e3c039bbf7f4b9c76a2f6b",
        strip_prefix = "pin-project-lite-0.1.11",
        build_file = Label("//third_party/cargo/remote:BUILD.pin-project-lite-0.1.11.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pin_project_lite__0_2_1",
        url = "https://crates.io/api/v1/crates/pin-project-lite/0.2.1/download",
        type = "tar.gz",
        sha256 = "e36743d754ccdf9954c2e352ce2d4b106e024c814f6499c2dadff80da9a442d8",
        strip_prefix = "pin-project-lite-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.pin-project-lite-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pin_utils__0_1_0",
        url = "https://crates.io/api/v1/crates/pin-utils/0.1.0/download",
        type = "tar.gz",
        sha256 = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184",
        strip_prefix = "pin-utils-0.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.pin-utils-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__pkg_config__0_3_19",
        url = "https://crates.io/api/v1/crates/pkg-config/0.3.19/download",
        type = "tar.gz",
        sha256 = "3831453b3449ceb48b6d9c7ad7c96d5ea673e9b470a1dc578c2ce6521230884c",
        strip_prefix = "pkg-config-0.3.19",
        build_file = Label("//third_party/cargo/remote:BUILD.pkg-config-0.3.19.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__polling__2_0_2",
        url = "https://crates.io/api/v1/crates/polling/2.0.2/download",
        type = "tar.gz",
        sha256 = "a2a7bc6b2a29e632e45451c941832803a18cce6781db04de8a04696cdca8bde4",
        strip_prefix = "polling-2.0.2",
        build_file = Label("//third_party/cargo/remote:BUILD.polling-2.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ppv_lite86__0_2_10",
        url = "https://crates.io/api/v1/crates/ppv-lite86/0.2.10/download",
        type = "tar.gz",
        sha256 = "ac74c624d6b2d21f425f752262f42188365d7b8ff1aff74c82e45136510a4857",
        strip_prefix = "ppv-lite86-0.2.10",
        build_file = Label("//third_party/cargo/remote:BUILD.ppv-lite86-0.2.10.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__precomputed_hash__0_1_1",
        url = "https://crates.io/api/v1/crates/precomputed-hash/0.1.1/download",
        type = "tar.gz",
        sha256 = "925383efa346730478fb4838dbe9137d2a47675ad789c546d150a6e1dd4ab31c",
        strip_prefix = "precomputed-hash-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.precomputed-hash-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__proc_macro_error__1_0_4",
        url = "https://crates.io/api/v1/crates/proc-macro-error/1.0.4/download",
        type = "tar.gz",
        sha256 = "da25490ff9892aab3fcf7c36f08cfb902dd3e71ca0f9f9517bea02a73a5ce38c",
        strip_prefix = "proc-macro-error-1.0.4",
        build_file = Label("//third_party/cargo/remote:BUILD.proc-macro-error-1.0.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__proc_macro_error_attr__1_0_4",
        url = "https://crates.io/api/v1/crates/proc-macro-error-attr/1.0.4/download",
        type = "tar.gz",
        sha256 = "a1be40180e52ecc98ad80b184934baf3d0d29f979574e439af5a55274b35f869",
        strip_prefix = "proc-macro-error-attr-1.0.4",
        build_file = Label("//third_party/cargo/remote:BUILD.proc-macro-error-attr-1.0.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__proc_macro_hack__0_5_19",
        url = "https://crates.io/api/v1/crates/proc-macro-hack/0.5.19/download",
        type = "tar.gz",
        sha256 = "dbf0c48bc1d91375ae5c3cd81e3722dff1abcf81a30960240640d223f59fe0e5",
        strip_prefix = "proc-macro-hack-0.5.19",
        build_file = Label("//third_party/cargo/remote:BUILD.proc-macro-hack-0.5.19.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__proc_macro_nested__0_1_6",
        url = "https://crates.io/api/v1/crates/proc-macro-nested/0.1.6/download",
        type = "tar.gz",
        sha256 = "eba180dafb9038b050a4c280019bbedf9f2467b61e5d892dcad585bb57aadc5a",
        strip_prefix = "proc-macro-nested-0.1.6",
        build_file = Label("//third_party/cargo/remote:BUILD.proc-macro-nested-0.1.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__proc_macro2__1_0_24",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.24/download",
        type = "tar.gz",
        sha256 = "1e0704ee1a7e00d7bb417d0770ea303c1bccbabf0ef1667dae92b5967f5f8a71",
        strip_prefix = "proc-macro2-1.0.24",
        build_file = Label("//third_party/cargo/remote:BUILD.proc-macro2-1.0.24.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__puddle__0_1_0",
        url = "https://crates.io/api/v1/crates/puddle/0.1.0/download",
        type = "tar.gz",
        sha256 = "0cf84452e80b28e2b05e53964d6f5a44a57978ce19b4920be49e1a61079a24d7",
        strip_prefix = "puddle-0.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.puddle-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__qstring__0_7_2",
        url = "https://crates.io/api/v1/crates/qstring/0.7.2/download",
        type = "tar.gz",
        sha256 = "d464fae65fff2680baf48019211ce37aaec0c78e9264c84a3e484717f965104e",
        strip_prefix = "qstring-0.7.2",
        build_file = Label("//third_party/cargo/remote:BUILD.qstring-0.7.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__quote__1_0_8",
        url = "https://crates.io/api/v1/crates/quote/1.0.8/download",
        type = "tar.gz",
        sha256 = "991431c3519a3f36861882da93630ce66b52918dcf1b8e2fd66b397fc96f28df",
        strip_prefix = "quote-1.0.8",
        build_file = Label("//third_party/cargo/remote:BUILD.quote-1.0.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand__0_7_3",
        url = "https://crates.io/api/v1/crates/rand/0.7.3/download",
        type = "tar.gz",
        sha256 = "6a6b1679d49b24bbfe0c803429aa1874472f50d9b363131f0e89fc356b544d03",
        strip_prefix = "rand-0.7.3",
        build_file = Label("//third_party/cargo/remote:BUILD.rand-0.7.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand__0_8_1",
        url = "https://crates.io/api/v1/crates/rand/0.8.1/download",
        type = "tar.gz",
        sha256 = "c24fcd450d3fa2b592732565aa4f17a27a61c65ece4726353e000939b0edee34",
        strip_prefix = "rand-0.8.1",
        build_file = Label("//third_party/cargo/remote:BUILD.rand-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand_chacha__0_2_2",
        url = "https://crates.io/api/v1/crates/rand_chacha/0.2.2/download",
        type = "tar.gz",
        sha256 = "f4c8ed856279c9737206bf725bf36935d8666ead7aa69b52be55af369d193402",
        strip_prefix = "rand_chacha-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_chacha-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand_chacha__0_3_0",
        url = "https://crates.io/api/v1/crates/rand_chacha/0.3.0/download",
        type = "tar.gz",
        sha256 = "e12735cf05c9e10bf21534da50a147b924d555dc7a547c42e6bb2d5b6017ae0d",
        strip_prefix = "rand_chacha-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_chacha-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand_core__0_5_1",
        url = "https://crates.io/api/v1/crates/rand_core/0.5.1/download",
        type = "tar.gz",
        sha256 = "90bde5296fc891b0cef12a6d03ddccc162ce7b2aff54160af9338f8d40df6d19",
        strip_prefix = "rand_core-0.5.1",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_core-0.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand_core__0_6_1",
        url = "https://crates.io/api/v1/crates/rand_core/0.6.1/download",
        type = "tar.gz",
        sha256 = "c026d7df8b298d90ccbbc5190bd04d85e159eaf5576caeacf8741da93ccbd2e5",
        strip_prefix = "rand_core-0.6.1",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_core-0.6.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand_hc__0_2_0",
        url = "https://crates.io/api/v1/crates/rand_hc/0.2.0/download",
        type = "tar.gz",
        sha256 = "ca3129af7b92a17112d59ad498c6f81eaf463253766b90396d39ea7a39d6613c",
        strip_prefix = "rand_hc-0.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_hc-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rand_hc__0_3_0",
        url = "https://crates.io/api/v1/crates/rand_hc/0.3.0/download",
        type = "tar.gz",
        sha256 = "3190ef7066a446f2e7f42e239d161e905420ccab01eb967c9eb27d21b2322a73",
        strip_prefix = "rand_hc-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.rand_hc-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__redox_syscall__0_1_57",
        url = "https://crates.io/api/v1/crates/redox_syscall/0.1.57/download",
        type = "tar.gz",
        sha256 = "41cc0f7e4d5d4544e8861606a285bb08d3e70712ccc7d2b84d7c0ccfaf4b05ce",
        strip_prefix = "redox_syscall-0.1.57",
        build_file = Label("//third_party/cargo/remote:BUILD.redox_syscall-0.1.57.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__redox_users__0_3_5",
        url = "https://crates.io/api/v1/crates/redox_users/0.3.5/download",
        type = "tar.gz",
        sha256 = "de0737333e7a9502c789a36d7c7fa6092a49895d4faa31ca5df163857ded2e9d",
        strip_prefix = "redox_users-0.3.5",
        build_file = Label("//third_party/cargo/remote:BUILD.redox_users-0.3.5.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__regex__1_4_2",
        url = "https://crates.io/api/v1/crates/regex/1.4.2/download",
        type = "tar.gz",
        sha256 = "38cf2c13ed4745de91a5eb834e11c00bcc3709e773173b2ce4c56c9fbde04b9c",
        strip_prefix = "regex-1.4.2",
        build_file = Label("//third_party/cargo/remote:BUILD.regex-1.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__regex_syntax__0_6_21",
        url = "https://crates.io/api/v1/crates/regex-syntax/0.6.21/download",
        type = "tar.gz",
        sha256 = "3b181ba2dcf07aaccad5448e8ead58db5b742cf85dfe035e2227f137a539a189",
        strip_prefix = "regex-syntax-0.6.21",
        build_file = Label("//third_party/cargo/remote:BUILD.regex-syntax-0.6.21.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__remove_dir_all__0_5_3",
        url = "https://crates.io/api/v1/crates/remove_dir_all/0.5.3/download",
        type = "tar.gz",
        sha256 = "3acd125665422973a33ac9d3dd2df85edad0f4ae9b00dafb1a05e43a9f5ef8e7",
        strip_prefix = "remove_dir_all-0.5.3",
        build_file = Label("//third_party/cargo/remote:BUILD.remove_dir_all-0.5.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__reqwest__0_10_10",
        url = "https://crates.io/api/v1/crates/reqwest/0.10.10/download",
        type = "tar.gz",
        sha256 = "0718f81a8e14c4dbb3b34cf23dc6aaf9ab8a0dfec160c534b3dbca1aaa21f47c",
        strip_prefix = "reqwest-0.10.10",
        build_file = Label("//third_party/cargo/remote:BUILD.reqwest-0.10.10.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rust_argon2__0_8_3",
        url = "https://crates.io/api/v1/crates/rust-argon2/0.8.3/download",
        type = "tar.gz",
        sha256 = "4b18820d944b33caa75a71378964ac46f58517c92b6ae5f762636247c09e78fb",
        strip_prefix = "rust-argon2-0.8.3",
        build_file = Label("//third_party/cargo/remote:BUILD.rust-argon2-0.8.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__rustc_serialize__0_3_24",
        url = "https://crates.io/api/v1/crates/rustc-serialize/0.3.24/download",
        type = "tar.gz",
        sha256 = "dcf128d1287d2ea9d80910b5f1120d0b8eede3fbf1abe91c40d39ea7d51e6fda",
        strip_prefix = "rustc-serialize-0.3.24",
        build_file = Label("//third_party/cargo/remote:BUILD.rustc-serialize-0.3.24.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ryu__1_0_5",
        url = "https://crates.io/api/v1/crates/ryu/1.0.5/download",
        type = "tar.gz",
        sha256 = "71d301d4193d031abdd79ff7e3dd721168a9572ef3fe51a1517aba235bd8f86e",
        strip_prefix = "ryu-1.0.5",
        build_file = Label("//third_party/cargo/remote:BUILD.ryu-1.0.5.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__same_file__1_0_6",
        url = "https://crates.io/api/v1/crates/same-file/1.0.6/download",
        type = "tar.gz",
        sha256 = "93fc1dc3aaa9bfed95e02e6eadabb4baf7e3078b0bd1b4d7b6b0b68378900502",
        strip_prefix = "same-file-1.0.6",
        build_file = Label("//third_party/cargo/remote:BUILD.same-file-1.0.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__schannel__0_1_19",
        url = "https://crates.io/api/v1/crates/schannel/0.1.19/download",
        type = "tar.gz",
        sha256 = "8f05ba609c234e60bee0d547fe94a4c7e9da733d1c962cf6e59efa4cd9c8bc75",
        strip_prefix = "schannel-0.1.19",
        build_file = Label("//third_party/cargo/remote:BUILD.schannel-0.1.19.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__scopeguard__1_1_0",
        url = "https://crates.io/api/v1/crates/scopeguard/1.1.0/download",
        type = "tar.gz",
        sha256 = "d29ab0c6d3fc0ee92fe66e2d99f700eab17a8d57d1c1d3b748380fb20baa78cd",
        strip_prefix = "scopeguard-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.scopeguard-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__security_framework__2_0_0",
        url = "https://crates.io/api/v1/crates/security-framework/2.0.0/download",
        type = "tar.gz",
        sha256 = "c1759c2e3c8580017a484a7ac56d3abc5a6c1feadf88db2f3633f12ae4268c69",
        strip_prefix = "security-framework-2.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.security-framework-2.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__security_framework_sys__2_0_0",
        url = "https://crates.io/api/v1/crates/security-framework-sys/2.0.0/download",
        type = "tar.gz",
        sha256 = "f99b9d5e26d2a71633cc4f2ebae7cc9f874044e0c351a27e17892d76dce5678b",
        strip_prefix = "security-framework-sys-2.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.security-framework-sys-2.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__semver__0_11_0",
        url = "https://crates.io/api/v1/crates/semver/0.11.0/download",
        type = "tar.gz",
        sha256 = "f301af10236f6df4160f7c3f04eec6dbc70ace82d23326abad5edee88801c6b6",
        strip_prefix = "semver-0.11.0",
        build_file = Label("//third_party/cargo/remote:BUILD.semver-0.11.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__semver_parser__0_10_0",
        url = "https://crates.io/api/v1/crates/semver-parser/0.10.0/download",
        type = "tar.gz",
        sha256 = "0e012c6c5380fb91897ba7b9261a0f565e624e869d42fe1a1d03fa0d68a083d5",
        strip_prefix = "semver-parser-0.10.0",
        build_file = Label("//third_party/cargo/remote:BUILD.semver-parser-0.10.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__serde__1_0_118",
        url = "https://crates.io/api/v1/crates/serde/1.0.118/download",
        type = "tar.gz",
        sha256 = "06c64263859d87aa2eb554587e2d23183398d617427327cf2b3d0ed8c69e4800",
        strip_prefix = "serde-1.0.118",
        build_file = Label("//third_party/cargo/remote:BUILD.serde-1.0.118.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__serde_derive__1_0_118",
        url = "https://crates.io/api/v1/crates/serde_derive/1.0.118/download",
        type = "tar.gz",
        sha256 = "c84d3526699cd55261af4b941e4e725444df67aa4f9e6a3564f18030d12672df",
        strip_prefix = "serde_derive-1.0.118",
        build_file = Label("//third_party/cargo/remote:BUILD.serde_derive-1.0.118.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__serde_json__1_0_61",
        url = "https://crates.io/api/v1/crates/serde_json/1.0.61/download",
        type = "tar.gz",
        sha256 = "4fceb2595057b6891a4ee808f70054bd2d12f0e97f1cbb78689b59f676df325a",
        strip_prefix = "serde_json-1.0.61",
        build_file = Label("//third_party/cargo/remote:BUILD.serde_json-1.0.61.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__serde_regex__1_1_0",
        url = "https://crates.io/api/v1/crates/serde_regex/1.1.0/download",
        type = "tar.gz",
        sha256 = "a8136f1a4ea815d7eac4101cfd0b16dc0cb5e1fe1b8609dfd728058656b7badf",
        strip_prefix = "serde_regex-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.serde_regex-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__serde_urlencoded__0_7_0",
        url = "https://crates.io/api/v1/crates/serde_urlencoded/0.7.0/download",
        type = "tar.gz",
        sha256 = "edfa57a7f8d9c1d260a549e7224100f6c43d43f9103e06dd8b4095a9b2b43ce9",
        strip_prefix = "serde_urlencoded-0.7.0",
        build_file = Label("//third_party/cargo/remote:BUILD.serde_urlencoded-0.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__sha_1__0_8_2",
        url = "https://crates.io/api/v1/crates/sha-1/0.8.2/download",
        type = "tar.gz",
        sha256 = "f7d94d0bede923b3cea61f3f1ff57ff8cdfd77b400fb8f9998949e0cf04163df",
        strip_prefix = "sha-1-0.8.2",
        build_file = Label("//third_party/cargo/remote:BUILD.sha-1-0.8.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__signal_hook__0_1_17",
        url = "https://crates.io/api/v1/crates/signal-hook/0.1.17/download",
        type = "tar.gz",
        sha256 = "7e31d442c16f047a671b5a71e2161d6e68814012b7f5379d269ebd915fac2729",
        strip_prefix = "signal-hook-0.1.17",
        build_file = Label("//third_party/cargo/remote:BUILD.signal-hook-0.1.17.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__signal_hook_registry__1_3_0",
        url = "https://crates.io/api/v1/crates/signal-hook-registry/1.3.0/download",
        type = "tar.gz",
        sha256 = "16f1d0fef1604ba8f7a073c7e701f213e056707210e9020af4528e0101ce11a6",
        strip_prefix = "signal-hook-registry-1.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.signal-hook-registry-1.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__siphasher__0_3_3",
        url = "https://crates.io/api/v1/crates/siphasher/0.3.3/download",
        type = "tar.gz",
        sha256 = "fa8f3741c7372e75519bd9346068370c9cdaabcc1f9599cbcf2a2719352286b7",
        strip_prefix = "siphasher-0.3.3",
        build_file = Label("//third_party/cargo/remote:BUILD.siphasher-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__slab__0_4_2",
        url = "https://crates.io/api/v1/crates/slab/0.4.2/download",
        type = "tar.gz",
        sha256 = "c111b5bd5695e56cffe5129854aa230b39c93a305372fdbb2668ca2394eea9f8",
        strip_prefix = "slab-0.4.2",
        build_file = Label("//third_party/cargo/remote:BUILD.slab-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__slug__0_1_4",
        url = "https://crates.io/api/v1/crates/slug/0.1.4/download",
        type = "tar.gz",
        sha256 = "b3bc762e6a4b6c6fcaade73e77f9ebc6991b676f88bb2358bddb56560f073373",
        strip_prefix = "slug-0.1.4",
        build_file = Label("//third_party/cargo/remote:BUILD.slug-0.1.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__sluice__0_5_3",
        url = "https://crates.io/api/v1/crates/sluice/0.5.3/download",
        type = "tar.gz",
        sha256 = "8e24ed1edc8e774f2ec098b0650eec82bfc7c59ddd16cd0e17797bdc92ce2bf1",
        strip_prefix = "sluice-0.5.3",
        build_file = Label("//third_party/cargo/remote:BUILD.sluice-0.5.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__smallvec__1_6_0",
        url = "https://crates.io/api/v1/crates/smallvec/1.6.0/download",
        type = "tar.gz",
        sha256 = "1a55ca5f3b68e41c979bf8c46a6f1da892ca4db8f94023ce0bd32407573b1ac0",
        strip_prefix = "smallvec-1.6.0",
        build_file = Label("//third_party/cargo/remote:BUILD.smallvec-1.6.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__smartstring__0_2_6",
        url = "https://crates.io/api/v1/crates/smartstring/0.2.6/download",
        type = "tar.gz",
        sha256 = "1ada87540bf8ef4cf8a1789deb175626829bb59b1fefd816cf7f7f55efcdbae9",
        strip_prefix = "smartstring-0.2.6",
        build_file = Label("//third_party/cargo/remote:BUILD.smartstring-0.2.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__socket2__0_3_19",
        url = "https://crates.io/api/v1/crates/socket2/0.3.19/download",
        type = "tar.gz",
        sha256 = "122e570113d28d773067fab24266b66753f6ea915758651696b6e35e49f88d6e",
        strip_prefix = "socket2-0.3.19",
        build_file = Label("//third_party/cargo/remote:BUILD.socket2-0.3.19.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__spdx__0_3_4",
        url = "https://crates.io/api/v1/crates/spdx/0.3.4/download",
        type = "tar.gz",
        sha256 = "1a68f874c9aa7762aa10401e2ae004d977e7b6156074668eb4ce78dd0cb28255",
        strip_prefix = "spdx-0.3.4",
        build_file = Label("//third_party/cargo/remote:BUILD.spdx-0.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__spinning_top__0_2_2",
        url = "https://crates.io/api/v1/crates/spinning_top/0.2.2/download",
        type = "tar.gz",
        sha256 = "7e529d73e80d64b5f2631f9035113347c578a1c9c7774b83a2b880788459ab36",
        strip_prefix = "spinning_top-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.spinning_top-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__static_assertions__1_1_0",
        url = "https://crates.io/api/v1/crates/static_assertions/1.1.0/download",
        type = "tar.gz",
        sha256 = "a2eb9349b6444b326872e140eb1cf5e7c522154d69e7a0ffb0fb81c06b37543f",
        strip_prefix = "static_assertions-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.static_assertions-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__string_cache__0_8_1",
        url = "https://crates.io/api/v1/crates/string_cache/0.8.1/download",
        type = "tar.gz",
        sha256 = "8ddb1139b5353f96e429e1a5e19fbaf663bddedaa06d1dbd49f82e352601209a",
        strip_prefix = "string_cache-0.8.1",
        build_file = Label("//third_party/cargo/remote:BUILD.string_cache-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__strsim__0_9_3",
        url = "https://crates.io/api/v1/crates/strsim/0.9.3/download",
        type = "tar.gz",
        sha256 = "6446ced80d6c486436db5c078dde11a9f73d42b57fb273121e160b84f63d894c",
        strip_prefix = "strsim-0.9.3",
        build_file = Label("//third_party/cargo/remote:BUILD.strsim-0.9.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__syn__1_0_57",
        url = "https://crates.io/api/v1/crates/syn/1.0.57/download",
        type = "tar.gz",
        sha256 = "4211ce9909eb971f111059df92c45640aad50a619cf55cd76476be803c4c68e6",
        strip_prefix = "syn-1.0.57",
        build_file = Label("//third_party/cargo/remote:BUILD.syn-1.0.57.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tar__0_4_30",
        url = "https://crates.io/api/v1/crates/tar/0.4.30/download",
        type = "tar.gz",
        sha256 = "489997b7557e9a43e192c527face4feacc78bfbe6eed67fd55c4c9e381cba290",
        strip_prefix = "tar-0.4.30",
        build_file = Label("//third_party/cargo/remote:BUILD.tar-0.4.30.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tempfile__3_1_0",
        url = "https://crates.io/api/v1/crates/tempfile/3.1.0/download",
        type = "tar.gz",
        sha256 = "7a6e24d9338a0a5be79593e2fa15a648add6138caa803e2d5bc782c371732ca9",
        strip_prefix = "tempfile-3.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.tempfile-3.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tera__1_6_1",
        url = "https://crates.io/api/v1/crates/tera/1.6.1/download",
        type = "tar.gz",
        sha256 = "eac6ab7eacf40937241959d540670f06209c38ceadb62116999db4a950fbf8dc",
        strip_prefix = "tera-1.6.1",
        build_file = Label("//third_party/cargo/remote:BUILD.tera-1.6.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__term__0_5_2",
        url = "https://crates.io/api/v1/crates/term/0.5.2/download",
        type = "tar.gz",
        sha256 = "edd106a334b7657c10b7c540a0106114feadeb4dc314513e97df481d5d966f42",
        strip_prefix = "term-0.5.2",
        build_file = Label("//third_party/cargo/remote:BUILD.term-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__thread_local__1_0_1",
        url = "https://crates.io/api/v1/crates/thread_local/1.0.1/download",
        type = "tar.gz",
        sha256 = "d40c6d1b69745a6ec6fb1ca717914848da4b44ae29d9b3080cbee91d72a69b14",
        strip_prefix = "thread_local-1.0.1",
        build_file = Label("//third_party/cargo/remote:BUILD.thread_local-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__time__0_1_44",
        url = "https://crates.io/api/v1/crates/time/0.1.44/download",
        type = "tar.gz",
        sha256 = "6db9e6914ab8b1ae1c260a4ae7a49b6c5611b40328a735b21862567685e73255",
        strip_prefix = "time-0.1.44",
        build_file = Label("//third_party/cargo/remote:BUILD.time-0.1.44.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tiny_keccak__2_0_2",
        url = "https://crates.io/api/v1/crates/tiny-keccak/2.0.2/download",
        type = "tar.gz",
        sha256 = "2c9d3793400a45f954c52e73d068316d76b6f4e36977e3fcebb13a2721e80237",
        strip_prefix = "tiny-keccak-2.0.2",
        build_file = Label("//third_party/cargo/remote:BUILD.tiny-keccak-2.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tinyvec__1_1_0",
        url = "https://crates.io/api/v1/crates/tinyvec/1.1.0/download",
        type = "tar.gz",
        sha256 = "ccf8dbc19eb42fba10e8feaaec282fb50e2c14b2726d6301dbfeed0f73306a6f",
        strip_prefix = "tinyvec-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.tinyvec-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tinyvec_macros__0_1_0",
        url = "https://crates.io/api/v1/crates/tinyvec_macros/0.1.0/download",
        type = "tar.gz",
        sha256 = "cda74da7e1a664f795bb1f8a87ec406fb89a02522cf6e50620d016add6dbbf5c",
        strip_prefix = "tinyvec_macros-0.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.tinyvec_macros-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tokio__0_2_24",
        url = "https://crates.io/api/v1/crates/tokio/0.2.24/download",
        type = "tar.gz",
        sha256 = "099837d3464c16a808060bb3f02263b412f6fafcb5d01c533d309985fbeebe48",
        strip_prefix = "tokio-0.2.24",
        build_file = Label("//third_party/cargo/remote:BUILD.tokio-0.2.24.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tokio_macros__0_2_6",
        url = "https://crates.io/api/v1/crates/tokio-macros/0.2.6/download",
        type = "tar.gz",
        sha256 = "e44da00bfc73a25f814cd8d7e57a68a5c31b74b3152a0a1d1f590c97ed06265a",
        strip_prefix = "tokio-macros-0.2.6",
        build_file = Label("//third_party/cargo/remote:BUILD.tokio-macros-0.2.6.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tokio_tls__0_3_1",
        url = "https://crates.io/api/v1/crates/tokio-tls/0.3.1/download",
        type = "tar.gz",
        sha256 = "9a70f4fcd7b3b24fb194f837560168208f669ca8cb70d0c4b862944452396343",
        strip_prefix = "tokio-tls-0.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.tokio-tls-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tokio_util__0_3_1",
        url = "https://crates.io/api/v1/crates/tokio-util/0.3.1/download",
        type = "tar.gz",
        sha256 = "be8242891f2b6cbef26a2d7e8605133c2c554cd35b3e4948ea892d6d68436499",
        strip_prefix = "tokio-util-0.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.tokio-util-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__toml__0_5_8",
        url = "https://crates.io/api/v1/crates/toml/0.5.8/download",
        type = "tar.gz",
        sha256 = "a31142970826733df8241ef35dc040ef98c679ab14d7c3e54d827099b3acecaa",
        strip_prefix = "toml-0.5.8",
        build_file = Label("//third_party/cargo/remote:BUILD.toml-0.5.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tower_service__0_3_0",
        url = "https://crates.io/api/v1/crates/tower-service/0.3.0/download",
        type = "tar.gz",
        sha256 = "e987b6bf443f4b5b3b6f38704195592cca41c5bb7aedd3c3693c7081f8289860",
        strip_prefix = "tower-service-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.tower-service-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tracing__0_1_22",
        url = "https://crates.io/api/v1/crates/tracing/0.1.22/download",
        type = "tar.gz",
        sha256 = "9f47026cdc4080c07e49b37087de021820269d996f581aac150ef9e5583eefe3",
        strip_prefix = "tracing-0.1.22",
        build_file = Label("//third_party/cargo/remote:BUILD.tracing-0.1.22.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tracing_attributes__0_1_11",
        url = "https://crates.io/api/v1/crates/tracing-attributes/0.1.11/download",
        type = "tar.gz",
        sha256 = "80e0ccfc3378da0cce270c946b676a376943f5cd16aeba64568e7939806f4ada",
        strip_prefix = "tracing-attributes-0.1.11",
        build_file = Label("//third_party/cargo/remote:BUILD.tracing-attributes-0.1.11.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tracing_core__0_1_17",
        url = "https://crates.io/api/v1/crates/tracing-core/0.1.17/download",
        type = "tar.gz",
        sha256 = "f50de3927f93d202783f4513cda820ab47ef17f624b03c096e86ef00c67e6b5f",
        strip_prefix = "tracing-core-0.1.17",
        build_file = Label("//third_party/cargo/remote:BUILD.tracing-core-0.1.17.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__tracing_futures__0_2_4",
        url = "https://crates.io/api/v1/crates/tracing-futures/0.2.4/download",
        type = "tar.gz",
        sha256 = "ab7bb6f14721aa00656086e9335d363c5c8747bae02ebe32ea2c7dece5689b4c",
        strip_prefix = "tracing-futures-0.2.4",
        build_file = Label("//third_party/cargo/remote:BUILD.tracing-futures-0.2.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__try_lock__0_2_3",
        url = "https://crates.io/api/v1/crates/try-lock/0.2.3/download",
        type = "tar.gz",
        sha256 = "59547bce71d9c38b83d9c0e92b6066c4253371f15005def0c30d9657f50c7642",
        strip_prefix = "try-lock-0.2.3",
        build_file = Label("//third_party/cargo/remote:BUILD.try-lock-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__typenum__1_12_0",
        url = "https://crates.io/api/v1/crates/typenum/1.12.0/download",
        type = "tar.gz",
        sha256 = "373c8a200f9e67a0c95e62a4f52fbf80c23b4381c05a17845531982fa99e6b33",
        strip_prefix = "typenum-1.12.0",
        build_file = Label("//third_party/cargo/remote:BUILD.typenum-1.12.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ucd_trie__0_1_3",
        url = "https://crates.io/api/v1/crates/ucd-trie/0.1.3/download",
        type = "tar.gz",
        sha256 = "56dee185309b50d1f11bfedef0fe6d036842e3fb77413abef29f8f8d1c5d4c1c",
        strip_prefix = "ucd-trie-0.1.3",
        build_file = Label("//third_party/cargo/remote:BUILD.ucd-trie-0.1.3.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unic_char_property__0_9_0",
        url = "https://crates.io/api/v1/crates/unic-char-property/0.9.0/download",
        type = "tar.gz",
        sha256 = "a8c57a407d9b6fa02b4795eb81c5b6652060a15a7903ea981f3d723e6c0be221",
        strip_prefix = "unic-char-property-0.9.0",
        build_file = Label("//third_party/cargo/remote:BUILD.unic-char-property-0.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unic_char_range__0_9_0",
        url = "https://crates.io/api/v1/crates/unic-char-range/0.9.0/download",
        type = "tar.gz",
        sha256 = "0398022d5f700414f6b899e10b8348231abf9173fa93144cbc1a43b9793c1fbc",
        strip_prefix = "unic-char-range-0.9.0",
        build_file = Label("//third_party/cargo/remote:BUILD.unic-char-range-0.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unic_common__0_9_0",
        url = "https://crates.io/api/v1/crates/unic-common/0.9.0/download",
        type = "tar.gz",
        sha256 = "80d7ff825a6a654ee85a63e80f92f054f904f21e7d12da4e22f9834a4aaa35bc",
        strip_prefix = "unic-common-0.9.0",
        build_file = Label("//third_party/cargo/remote:BUILD.unic-common-0.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unic_segment__0_9_0",
        url = "https://crates.io/api/v1/crates/unic-segment/0.9.0/download",
        type = "tar.gz",
        sha256 = "e4ed5d26be57f84f176157270c112ef57b86debac9cd21daaabbe56db0f88f23",
        strip_prefix = "unic-segment-0.9.0",
        build_file = Label("//third_party/cargo/remote:BUILD.unic-segment-0.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unic_ucd_segment__0_9_0",
        url = "https://crates.io/api/v1/crates/unic-ucd-segment/0.9.0/download",
        type = "tar.gz",
        sha256 = "2079c122a62205b421f499da10f3ee0f7697f012f55b675e002483c73ea34700",
        strip_prefix = "unic-ucd-segment-0.9.0",
        build_file = Label("//third_party/cargo/remote:BUILD.unic-ucd-segment-0.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unic_ucd_version__0_9_0",
        url = "https://crates.io/api/v1/crates/unic-ucd-version/0.9.0/download",
        type = "tar.gz",
        sha256 = "96bd2f2237fe450fcd0a1d2f5f4e91711124f7857ba2e964247776ebeeb7b0c4",
        strip_prefix = "unic-ucd-version-0.9.0",
        build_file = Label("//third_party/cargo/remote:BUILD.unic-ucd-version-0.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unicase__2_6_0",
        url = "https://crates.io/api/v1/crates/unicase/2.6.0/download",
        type = "tar.gz",
        sha256 = "50f37be617794602aabbeee0be4f259dc1778fabe05e2d67ee8f79326d5cb4f6",
        strip_prefix = "unicase-2.6.0",
        build_file = Label("//third_party/cargo/remote:BUILD.unicase-2.6.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unicode_bidi__0_3_4",
        url = "https://crates.io/api/v1/crates/unicode-bidi/0.3.4/download",
        type = "tar.gz",
        sha256 = "49f2bd0c6468a8230e1db229cff8029217cf623c767ea5d60bfbd42729ea54d5",
        strip_prefix = "unicode-bidi-0.3.4",
        build_file = Label("//third_party/cargo/remote:BUILD.unicode-bidi-0.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unicode_normalization__0_1_16",
        url = "https://crates.io/api/v1/crates/unicode-normalization/0.1.16/download",
        type = "tar.gz",
        sha256 = "a13e63ab62dbe32aeee58d1c5408d35c36c392bba5d9d3142287219721afe606",
        strip_prefix = "unicode-normalization-0.1.16",
        build_file = Label("//third_party/cargo/remote:BUILD.unicode-normalization-0.1.16.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unicode_xid__0_2_1",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.1/download",
        type = "tar.gz",
        sha256 = "f7fe0bb3479651439c9112f72b6c505038574c9fbb575ed1bf3b797fa39dd564",
        strip_prefix = "unicode-xid-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.unicode-xid-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__unindent__0_1_7",
        url = "https://crates.io/api/v1/crates/unindent/0.1.7/download",
        type = "tar.gz",
        sha256 = "f14ee04d9415b52b3aeab06258a3f07093182b88ba0f9b8d203f211a7a7d41c7",
        strip_prefix = "unindent-0.1.7",
        build_file = Label("//third_party/cargo/remote:BUILD.unindent-0.1.7.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__url__2_2_0",
        url = "https://crates.io/api/v1/crates/url/2.2.0/download",
        type = "tar.gz",
        sha256 = "5909f2b0817350449ed73e8bcd81c8c3c8d9a7a5d8acba4b27db277f1868976e",
        strip_prefix = "url-2.2.0",
        build_file = Label("//third_party/cargo/remote:BUILD.url-2.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__vcpkg__0_2_11",
        url = "https://crates.io/api/v1/crates/vcpkg/0.2.11/download",
        type = "tar.gz",
        sha256 = "b00bca6106a5e23f3eee943593759b7fcddb00554332e856d990c893966879fb",
        strip_prefix = "vcpkg-0.2.11",
        build_file = Label("//third_party/cargo/remote:BUILD.vcpkg-0.2.11.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__vec_arena__1_0_0",
        url = "https://crates.io/api/v1/crates/vec-arena/1.0.0/download",
        type = "tar.gz",
        sha256 = "eafc1b9b2dfc6f5529177b62cf806484db55b32dc7c9658a118e11bbeb33061d",
        strip_prefix = "vec-arena-1.0.0",
        build_file = Label("//third_party/cargo/remote:BUILD.vec-arena-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__version_check__0_9_2",
        url = "https://crates.io/api/v1/crates/version_check/0.9.2/download",
        type = "tar.gz",
        sha256 = "b5a972e5669d67ba988ce3dc826706fb0a8b01471c088cb0b6110b805cc36aed",
        strip_prefix = "version_check-0.9.2",
        build_file = Label("//third_party/cargo/remote:BUILD.version_check-0.9.2.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__waker_fn__1_1_0",
        url = "https://crates.io/api/v1/crates/waker-fn/1.1.0/download",
        type = "tar.gz",
        sha256 = "9d5b2c62b4012a3e1eca5a7e077d13b3bf498c4073e33ccd58626607748ceeca",
        strip_prefix = "waker-fn-1.1.0",
        build_file = Label("//third_party/cargo/remote:BUILD.waker-fn-1.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__walkdir__2_3_1",
        url = "https://crates.io/api/v1/crates/walkdir/2.3.1/download",
        type = "tar.gz",
        sha256 = "777182bc735b6424e1a57516d35ed72cb8019d85c8c9bf536dccb3445c1a2f7d",
        strip_prefix = "walkdir-2.3.1",
        build_file = Label("//third_party/cargo/remote:BUILD.walkdir-2.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__want__0_3_0",
        url = "https://crates.io/api/v1/crates/want/0.3.0/download",
        type = "tar.gz",
        sha256 = "1ce8a968cb1cd110d136ff8b819a556d6fb6d919363c61534f6860c7eb172ba0",
        strip_prefix = "want-0.3.0",
        build_file = Label("//third_party/cargo/remote:BUILD.want-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasi__0_10_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.10.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "1a143597ca7c7793eff794def352d41792a93c481eb1042423ff7ff72ba2c31f",
        strip_prefix = "wasi-0.10.0+wasi-snapshot-preview1",
        build_file = Label("//third_party/cargo/remote:BUILD.wasi-0.10.0+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasi__0_9_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.9.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "cccddf32554fecc6acb585f82a32a72e28b48f8c4c1883ddfeeeaa96f7d8e519",
        strip_prefix = "wasi-0.9.0+wasi-snapshot-preview1",
        build_file = Label("//third_party/cargo/remote:BUILD.wasi-0.9.0+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasm_bindgen__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen/0.2.69/download",
        type = "tar.gz",
        sha256 = "3cd364751395ca0f68cafb17666eee36b63077fb5ecd972bbcd74c90c4bf736e",
        strip_prefix = "wasm-bindgen-0.2.69",
        build_file = Label("//third_party/cargo/remote:BUILD.wasm-bindgen-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasm_bindgen_backend__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-backend/0.2.69/download",
        type = "tar.gz",
        sha256 = "1114f89ab1f4106e5b55e688b828c0ab0ea593a1ea7c094b141b14cbaaec2d62",
        strip_prefix = "wasm-bindgen-backend-0.2.69",
        build_file = Label("//third_party/cargo/remote:BUILD.wasm-bindgen-backend-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasm_bindgen_futures__0_4_19",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-futures/0.4.19/download",
        type = "tar.gz",
        sha256 = "1fe9756085a84584ee9457a002b7cdfe0bfff169f45d2591d8be1345a6780e35",
        strip_prefix = "wasm-bindgen-futures-0.4.19",
        build_file = Label("//third_party/cargo/remote:BUILD.wasm-bindgen-futures-0.4.19.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasm_bindgen_macro__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.69/download",
        type = "tar.gz",
        sha256 = "7a6ac8995ead1f084a8dea1e65f194d0973800c7f571f6edd70adf06ecf77084",
        strip_prefix = "wasm-bindgen-macro-0.2.69",
        build_file = Label("//third_party/cargo/remote:BUILD.wasm-bindgen-macro-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasm_bindgen_macro_support__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.69/download",
        type = "tar.gz",
        sha256 = "b5a48c72f299d80557c7c62e37e7225369ecc0c963964059509fbafe917c7549",
        strip_prefix = "wasm-bindgen-macro-support-0.2.69",
        build_file = Label("//third_party/cargo/remote:BUILD.wasm-bindgen-macro-support-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wasm_bindgen_shared__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-shared/0.2.69/download",
        type = "tar.gz",
        sha256 = "7e7811dd7f9398f14cc76efd356f98f03aa30419dea46aa810d71e819fc97158",
        strip_prefix = "wasm-bindgen-shared-0.2.69",
        build_file = Label("//third_party/cargo/remote:BUILD.wasm-bindgen-shared-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__web_sys__0_3_46",
        url = "https://crates.io/api/v1/crates/web-sys/0.3.46/download",
        type = "tar.gz",
        sha256 = "222b1ef9334f92a21d3fb53dc3fd80f30836959a90f9274a626d7e06315ba3c3",
        strip_prefix = "web-sys-0.3.46",
        build_file = Label("//third_party/cargo/remote:BUILD.web-sys-0.3.46.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__wepoll_sys__3_0_1",
        url = "https://crates.io/api/v1/crates/wepoll-sys/3.0.1/download",
        type = "tar.gz",
        sha256 = "0fcb14dea929042224824779fbc82d9fab8d2e6d3cbc0ac404de8edf489e77ff",
        strip_prefix = "wepoll-sys-3.0.1",
        build_file = Label("//third_party/cargo/remote:BUILD.wepoll-sys-3.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__winapi__0_2_8",
        url = "https://crates.io/api/v1/crates/winapi/0.2.8/download",
        type = "tar.gz",
        sha256 = "167dc9d6949a9b857f3451275e911c3f44255842c1f7a76f33c55103a909087a",
        strip_prefix = "winapi-0.2.8",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-0.2.8.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__winapi__0_3_9",
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-0.3.9.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__winapi_build__0_1_1",
        url = "https://crates.io/api/v1/crates/winapi-build/0.1.1/download",
        type = "tar.gz",
        sha256 = "2d315eee3b34aca4797b2da6b13ed88266e6d612562a0c46390af8299fc699bc",
        strip_prefix = "winapi-build-0.1.1",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-build-0.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-i686-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__winapi_util__0_1_5",
        url = "https://crates.io/api/v1/crates/winapi-util/0.1.5/download",
        type = "tar.gz",
        sha256 = "70ec6ce85bb158151cae5e5c87f95a8e97d2c0c4b001223f33a334e3ce5de178",
        strip_prefix = "winapi-util-0.1.5",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-util-0.1.5.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//third_party/cargo/remote:BUILD.winapi-x86_64-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__winreg__0_7_0",
        url = "https://crates.io/api/v1/crates/winreg/0.7.0/download",
        type = "tar.gz",
        sha256 = "0120db82e8a1e0b9fb3345a539c478767c0048d842860994d96113d5b667bd69",
        strip_prefix = "winreg-0.7.0",
        build_file = Label("//third_party/cargo/remote:BUILD.winreg-0.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__ws2_32_sys__0_2_1",
        url = "https://crates.io/api/v1/crates/ws2_32-sys/0.2.1/download",
        type = "tar.gz",
        sha256 = "d59cefebd0c892fa2dd6de581e937301d8552cb44489cdff035c6187cb63fa5e",
        strip_prefix = "ws2_32-sys-0.2.1",
        build_file = Label("//third_party/cargo/remote:BUILD.ws2_32-sys-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "cargo_raze__xattr__0_2_2",
        url = "https://crates.io/api/v1/crates/xattr/0.2.2/download",
        type = "tar.gz",
        sha256 = "244c3741f4240ef46274860397c7c74e50eb23624996930e484c16679633a54c",
        strip_prefix = "xattr-0.2.2",
        build_file = Label("//third_party/cargo/remote:BUILD.xattr-0.2.2.bazel"),
    )
