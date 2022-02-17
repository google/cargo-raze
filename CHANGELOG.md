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

## v0.13 and below
TODO(dfreese)

