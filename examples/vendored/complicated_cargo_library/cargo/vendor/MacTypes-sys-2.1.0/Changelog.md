# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) 
and this project adheres to [Semantic Versioning](http://semver.org/).

## [2.1.0] - 2019-01-09
### Fixed
- Improved docs.
- Fixed a documentation test failure caused by fencing C code without marking it.

### Added
- Added impls of `PartialEq<UInt32>` and `PartialEq<NumVersion>` for `NumVersionVariant`.

## [2.0.2] - 2017-07-22
### Fixed
- Improve docs.

## [2.0.1] - 2017-07-22
### Fixed
- `NumVersionVariant`'s fields are now pub.

## [2.0.0] - 2017-07-21
### Changed
- `NumVersionVariant` is now a Union by default, as unions have been stabilised with Rust 1.19.

### Fixed
- Improved the documentation.

### Removed
- Deleted the depreciated `StrField32` alias.

## [1.3.0] - 2017-05-11
### Fixed
- Fixed a typo: renamed `StrField32` to `Str32Field`. Note that `StrField32` is still present,
  but it is marked as deprecated and will be removed in a future release.

### Added
- Added documentation copied from the original header file.

### Changed
- This project is now dual-licensed under the terms of the Apple Public Source License,
  as the project now has a substantial amount of Apple documentation copied verbatim
  from the original header.

## [1.2.0] - 2017-05-11
### Changed
- Added a default `use_std` feature which is enabled by default. This configures
  whether the crate has the `#![no_std]` attribute at the root.
- The `Debug` impl for `VersRec` will now throw an error instead of panicking if
  `ShortVersion` and `Reserved` cannot be converted to `UTF8`.
- Removed a use of unsafe in the `Default` impl of `VersRec`.

### Added
- Added the `nightly` feature, which makes `NumVersionVariant` a proper union.

## [1.1.0] - 2016-12-07
### Changed
- Made `StrLength` always inline, and not have `extern "C"` linkage because
  it is used as an internal/inline interface (and is declared as a macro when
  compiled as `C` in the header.

### Added
- Added `Float32Point` struct.

## [1.0.4] - 2016-10-19
### Changed
- Made `kVariableLengthArray` a variable of type `ItemCount`, and `kUnknownType`
  an `OSType`.
- Shortened the various `developStage` variables to `UInt8`, from `UInt32`.

### Added
- Added documentation link to Cargo.toml and Readme.md.

## [1.0.3] - 2016-09-20
### Changed
- Do not use any features in the libc crate.
- Float32/Float64 now alias to f32/f64 instead of c_float/c_double.

## [1.0.2] - 2016-09-14
### Changed
- Remove the WinApi dependency.
- Use rust-native fixed sized int types instead of libc's stdint types.

### Fixed
- Bugfix: Mark Float80 and Float96 as #[repr(C)]

## [1.0.1] - 2016-09-08
### Added
- Added crate-level documentation comment.

### Fixed
- Fixed typos in Readme.md

## [1.0.0] - 2016-09-06
### Added
- First release.

