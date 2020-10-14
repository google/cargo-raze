# cargo-raze: Bazel BUILD generation for Rust Crates

[![Build Status](https://travis-ci.org/google/cargo-raze.svg?branch=master)](https://travis-ci.org/google/cargo-raze)

An experimental support Cargo plugin for distilling a workspace-level
Cargo.toml into BUILD targets that code using [rules_rust](https://github.com/bazelbuild/rules_rust)
can depend on directly.

### Disclaimer

This is not an official Google product (experimental or otherwise), it is just code that happens to be owned by Google.

## Overview

This project synthesizes the dependency resolution logic and some of the
functionality of Cargo such as features and build scripts into executable
rules that Bazel can run to compile Rust crates. Though the standard rules_rust
rules can be used to compile Rust code from scratch, the fine granularity of the
dependency ecosystem makes transforming dependency trees based on that ecosystem
onerous, even for code with few dependencies.

## Usage

cargo-raze can generate buildable targets in one of two modes: Vendoring, or
Non-Vendoring. In the vendoring mode, developers use the common `cargo vendor`
subcommand to retrieve the dependencies indicated by their workspace Cargo.toml into
directories that cargo-raze then populates with BUILD files. In the
non-vendoring mode, cargo-raze generates a flat list of BUILD files, and a
workspace-level macro that can be invoked in the WORKSPACE file to pull down the
dependencies automatically in similar fashion to Cargo itself.

In both cases, the first step is to decide where to situate the Cargo
dependencies in the workspace. This library was designed with monorepos in mind,
where an organization decides upon a set of dependencies that everyone points
at. It is intended that stakeholders in the dependencies collaborate to upgrade
dependencies atomically, and fix breakages across their codebase simultaneously.
In the event that this isn't feasible, it is still possible to use cargo-raze in
a decentralized scenario, but it's unlikely that such decoupled repositories
would interact well together with the current implementation.

Regardless of the approach chosen, the rust_rules should be brought in to the
WORKSPACE. Here is an example:

```python
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "io_bazel_rules_rust",
    sha256 = "5ed804fcd10a506a5b8e9e59bc6b3b7f43bc30c87ce4670e6f78df43604894fd",
    strip_prefix = "rules_rust-fdf9655ba95616e0314b4e0ebab40bb0c5fe005c",
    urls = [
        # Master branch as of 2020-07-27
        "https://github.com/bazelbuild/rules_rust/archive/fdf9655ba95616e0314b4e0ebab40bb0c5fe005c.tar.gz",
    ],
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")
```

### Vendoring Mode

In Vendoring mode, a root directly is selected that will house the vendored
dependencies and become the gateway to those build rules. "//cargo" is
conventional, but "//third_party/cargo" may be desirable to satisfy
organizational needs. Vendoring directly into root isn't well supported due to
implementation-specific idiosyncracies, but it may be supported in the future.
From here forward, "//cargo" will be the assumed directory.

#### Generate a Cargo.toml

First, generate a standard Cargo.toml with the dependencies of interest. Take
care to include a `[lib]` directive so that Cargo does not complain about
missing source files for this mock crate. Here is an example:

```toml
[package]
name = "compile_with_bazel"
version = "0.0.0"

# Mandatory (or Cargo tooling is unhappy)
[lib]
path = "fake_lib.rs"

[dependencies]
log = "=0.3.6"

[raze]
# The WORKSPACE relative path to the Cargo.toml working directory.
workspace_path = "//cargo"

# The target to generate BUILD rules for.
target = "x86_64-unknown-linux-gnu"
```

#### Generate buildable targets

First, install the required tools for vendoring and generating BUILDable
targets.

```bash
$ cargo install cargo-raze
```

Following that, vendor your dependencies from within the cargo/ directory. This
will also update your `Cargo.lock` file.

```bash
$ cargo vendor --versioned-dirs
```

Finally, generate your BUILD files, again from within the cargo/ directory

```bash
$ cargo raze
```

You can now depend on any _explicit_ dependencies in any Rust rule by depending on
`//cargo:your_dependency_name`.

### Remote Dependency Mode

In Remote mode, a directory similar to the vendoring mode is selected. In this
case, though, it contains only BUILD files, a vendoring instruction for the
WORKSPACE, and aliases to the explicit dependencies. Slightly different plumbing
is required.

#### Generate a Cargo.toml

Generate a Cargo.toml, similar to Vendoring mode but add a new `genmode` directive in the
`[raze]` section

```toml
[raze]
# The WORKSPACE relative path to the Cargo.toml working directory.
workspace_path = "//cargo"

# The target to generate BUILD rules for.
target = "x86_64-unknown-linux-gnu"

genmode = "Remote"
```

This tells Raze not to expect the dependencies to be vendored and to generate
different files.

#### Generate buildable targets

First, install cargo-raze.

```bash
$ cargo install cargo-raze
```

Next, execute cargo raze from within the cargo directory

```bash
$ cargo raze
```

Finally, invoke the remote library fetching function within your WORKSPACE:

```python
load("//cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()
```

This tells Bazel where to get the dependencies from, and how to build them:
using the files generated into //cargo.

_Note that this method's name depends on your `gen_workspace_prefix` setting_.

You can depend on any _explicit_ dependencies in any Rust rule by depending on
`//cargo:your_dependency_name`.

### Handling Unconventional Crates

Some crates execute a "build script", which, while technically unrestricted in
what it can do, usually does one of a few common things.

All options noted below are enumerated in the
[src/settings.rs](./impl/src/settings.rs) file.

#### Crates that generate files using locally known information

In some cases, a crate uses only basic information in order to generate a Rust
source file. These build-scripts rules can actually be executed and used within
Bazel by including a directive in your Cargo.toml prior to generation:

```toml
[raze.crates.clang-sys.'0.21.1']
gen_buildrs = true
```

This setting tells cargo-raze to generate a rust_binary target for the build
script and to direct its generated (OUT_DIR-style) outputs to the parent crate.

#### Crates that depend on certain flags being determined by a build script

Some build scripts conditionally emit directives to stdout that Cargo knows how
to propagate. Unfortunately, its not so simple to manage build-time generated
dependency information, so if the flags are statically known (perhaps, since the
compilation target is statically known), they can be provided from within the
Cargo.toml, in the following manner

```toml
[raze.crates.unicase.'2.1.0']
additional_flags = [
  # Rustc is 1.15, enable all optional settings
  "--cfg=__unicase__iter_cmp",
  "--cfg=__unicase__defauler_hasher",
]
```

Flags provided in this manner are directly handed to rustc. It may be helpful to
refer to the build-script section of the documentation to interpret build
scripts and stdout directives that are encountered, available here:
https://doc.rust-lang.org/cargo/reference/build-scripts.html

#### Crates that need system libraries

There are two ways to provide system libraries that a crate needs for
compilation. The first is to vendor the system library directly, craft a BUILD
rule for it, and add the dependency to the corresponding `-sys` crate. For
openssl, this may in part look like:

```toml
[raze.crates.openssl-sys.'0.9.24']
additional_flags = [
  # Vendored openssl is 1.0.2m
  "--cfg=ossl102",
  "--cfg=version=102",
]
additional_deps = [
  "@//third_party/openssl:crypto",
  "@//third_party/openssl:ssl",
]

[raze.crates.openssl.'0.10.2']
additional_flags = [
  # Vendored openssl is 1.0.2m
  "--cfg=ossl102",
  "--cfg=version=102",
  "--cfg=ossl10x",
]
```

In some cases, directly wiring up a local system dependency may be preferable.
To do this, refer to the `new_local_repository` section of the Bazel
documentation. For a precompiled version of llvm in a WORKSPACE, this may look
something like:

```python
new_local_repository(
    name = "llvm",
    build_file = "llvm.BUILD",
    path = "/usr/lib/llvm-3.9",
)
```

In a few cases, the sys crate may need to be overridden entirely. This can be
facilitated by removing and supplementing dependencies in the Cargo.toml,
pre-generation:

```toml
[raze.crates.sdl2.'0.31.0']
skipped_deps = [
  "sdl2-sys-0.31.0"
]
additional_deps = [
  "@//cargo/overrides/sdl2-sys:sdl2_sys"
]
```

#### Crates that supply useful binaries

Some crates provide useful binaries that themselves can be used as part of a
compilation process: Bindgen is a great example. Bindgen produces Rust source
files by processing C or C++ files. A directive can be added to the Cargo.toml
to tell Bazel to expose such binaries for you:

```toml
[raze.crates.bindgen.'0.32.2']
gen_buildrs = true # needed to build bindgen
extra_aliased_targets = [
  "cargo_bin_bindgen"
]
```

Cargo-raze prefixes binary targets with `cargo_bin_`, as although Cargo permits
binaries and libraries to share the same target name, Bazel disallows this.

#### Crates that only provide binaries

Currently, cargo does not gather metadata about crates that do not provide any
libraries. This means that these specifying them in the `[dependencies]` section
of your `Cargo.toml` file will not result in generated Bazel targets. Cargo-raze
has a special field to handle these crates when using `genmode = "Remote"`:

```toml
[raze.binary_deps]
wasm-bindgen-cli = "0.2.68"
```

In the snippet above, the `wasm-bindgen-cli` crate is defined as binary dependency
and Cargo-raze will ensure metadata for this and any other crate defined here are
included in the resulting output directory. Lockfiles for targets specified under
`[raze.binary_deps]` will be generated into a `lockfiles` directory inside the path
specified by `workspace_path`.

## FAQ

## Why choose Bazel to build a Rust project?

Bazel ("fast", "correct", choose two) is a battle-tested build system used by
Google to compile incredibly large, multilingual projects without duplicating
effort, and without compromising on correctness. It accomplishes this in part by
limiting what mechanisms a given compilation object can use to discover
dependencies and by forcing buildable units to express the complete set of
their dependencies. It expects two identical sets of build target inputs to produce a
byte-for-byte equivalent final result.

In exchange, users are rewarded with a customizable and extensible build system
that compiles any kind of compilable target and allows expressing "unconventional
dependencies", such as Protobuf objects, precompiled graphics shaders, or
generated code, while remaining fast and correct.

Its also probable (though not yet demonstrated with benchmarks) that large
applications built with Bazel's strengths in mind: highly granular build units,
will compile significantly faster as they are able to cache more aggressively
and avoid recompilation of as much code while iterating.

## Why try to integrate Cargo's dependencies into this build tool?

For better or worse, the Rust ecosystem heavily depends on Cargo crates in order
to provide functionality that is often present in standard libraries. This is
actually a fantastic thing for the evolution of the language, as it describes a
structured process to stabilization (experimental crate -> 1.0 crate -> RFC ->
inclusion in stdlib), but it means that people who lack access to this ecosystem
must reinvent many wheels.

Putting that aside there are also fantastic crates that help Rust developers
interact with industry standard systems and libraries which can greatly
accelerate development in the language.

## Why not build directly with Cargo / Why generate rustc invocations?

Though the burden of emulating Cargo's functionality (where possible at all!) is
high, it appears to be the only way to maintain the guarantees (correctness,
reproducibility) that Bazel depends on to stay performant. It is possible and
likely with inflight RFCs that Cargo will become sufficiently flexible to
allow it to be used directly for compilation but at this point in time it
appears that maintaining a semblance of feature parity is actually easier than
avoiding all of the sharp edges introduced by treating Cargo like the Rust
compiler.

## What is buildable right now with Bazel, and what is not?

With a little bit of elbow grease it is possible to build nearly everything,
including projects that depend on openssl-sys. Many sys crates will require
identifying the system library that they wrap, and either vendoring it into the
project, or telling Bazel where it lives on your system. Some may require minor
source tweaks, such as eliminating hardcoded cargo environment variable
requirements. Fixes can be non-trivial in a few cases, but a good number of
the most popular crates have been built in an example repo, available at
https://github.com/acmcarther/cargo-raze-crater

## Example Repos

See these examples of providing crate configuration:

**Using vendored mode**:

- [hello-cargo-library](https://github.com/google/cargo-raze/tree/master/examples/vendored/hello_cargo_library)
- [complicated-cargo-library](https://github.com/google/cargo-raze/tree/master/examples/vendored/complicated_cargo_library)
- [non-cratesio](https://github.com/google/cargo-raze/tree/master/examples/vendored/non_cratesio_library)

**Using remote mode**:

- [complicated-example](https://github.com/google/cargo-raze/tree/master/examples/remote/complicated_cargo_library)
- [non-cratesio](https://github.com/google/cargo-raze/tree/master/examples/remote/non_cratesio)

**Compiling OpenSSL**:

- [openssl](https://github.com/acmcarther/compile_openssl)

The [raze] section is derived from a struct declared in [impl/src/settings.rs](./impl/src/settings.rs).
