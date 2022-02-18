# Change Log

## v0.15.0
* Added all generated bazel files to an exported filegroup

## v0.14.2
* Fixed [#417](https://github.com/google/cargo-raze/issues/417) where symlinks
  were improperly generated on Windows [[#464](
  https://github.com/google/cargo-raze/pull/464)].

## v0.14.1
* Added additional flags into the build scripts [[#461](
  https://github.com/google/cargo-raze/pull/461)]
* Fixed an issue where tags were no longer sorted [[#462](
  https://github.com/google/cargo-raze/pull/462)]

## v0.14.0
* Added a `crate-name` tag to generated bazel files that holds the original
  crate name without shifting `-` to `_` [[#455](
  https://github.com/google/cargo-raze/pull/455)]

## v0.13.1
* Updated bazel-based build rules for cargo raze to use latest rules_rust and
  rules_foreign_cc, which had breaking changes.  No significant behavior changes
  [[#452](https://github.com/google/cargo-raze/pull/452)]

## v0.13.0
* Planning is now done using the resolve tree, rather than packages, which
  resolves a number of issues (
  [#144](https://github.com/google/cargo-raze/issues/144),
  [#187](https://github.com/google/cargo-raze/issues/187),
  [#241](https://github.com/google/cargo-raze/issues/241),
  [#269](https://github.com/google/cargo-raze/issues/269),
  [#270](https://github.com/google/cargo-raze/issues/270)),
  [[#425](https://github.com/google/cargo-raze/pull/425)]
* Fixed issue [#355](https://github.com/google/cargo-raze/issues/355) for dev
  dependencies [[#405](https://github.com/google/cargo-raze/pull/405)]
* Stopped panicking on unrecognized licenses
  [[#413](https://github.com/google/cargo-raze/pull/413)]
* Started reporting an error if lock file generation failed
  [[#411](https://github.com/google/cargo-raze/pull/411)]
* Fixed issue [#389](https://github.com/google/cargo-raze/issues/389) where
  default dependencies would be incorrectly duplicated in the platform-specific
  dependencies [[#437](https://github.com/google/cargo-raze/pull/437)]
* Made some structs sortable and deserializable for when cargo raze is being
  used as a library [[#415](https://github.com/google/cargo-raze/pull/415)]

## v0.12.0
* Fixed an issue ([#354](https://github.com/google/cargo-raze/issues/354)) where
  a leading slash could be placed on package names if Cargo.toml and WORKSPACE
  are in the same directory.
  [[#411](https://github.com/google/cargo-raze/pull/411)]
* Fixed an issue with a leading slash on package names for Windows
  [[#401](https://github.com/google/cargo-raze/pull/401)]
* Started setting the `links` attribute for `cargo_build_script`
  [[#400](https://github.com/google/cargo-raze/pull/400)]

## v0.11 and below
* Please check the git history.
