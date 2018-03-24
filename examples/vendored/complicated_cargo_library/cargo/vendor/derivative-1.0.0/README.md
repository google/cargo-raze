# Derivative [![Travis](https://img.shields.io/travis/mcarton/rust-derivative.svg?maxAge=3600)](https://travis-ci.org/mcarton/rust-derivative) [![Build status](https://img.shields.io/appveyor/ci/mcarton/rust-derivative.svg?maxAge=3600)](https://ci.appveyor.com/project/mcarton/rust-derivative) [![Crates.io](https://img.shields.io/crates/v/derivative.svg?maxAge=2592000)](https://crates.io/crates/derivative) [![Crates.io](https://img.shields.io/crates/l/derivative.svg?maxAge=2592000)](https://github.com/mcarton/rust-derivative#license)

This crate provides a set of alternative `#[derive]` attributes for Rust.

## [Documentation][documentation]
## Stability

This crate is now stable and usable on rustc stable too!

Note that you need *rustc 1.15 or later*.

## What it does

```rust
#[derive(Derivative)]
#[derivative(Debug)]
struct Foo {
    foo: u8,
    #[derivative(Debug="ignore")]
    bar: u8,
}

println!("{:?}", Foo { foo: 42, bar: 1 }); // Foo { foo: 42 }
```

Check the [documentation] for more!

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Acknowledgements

This is inspired from how [`serde`] wonderfully handles attributes.
This also takes some code and ideas from `serde` itself.

Some tests are directly adapted from `rustc`'s tests.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`serde`]: https://crates.io/crates/serde
[documentation]: https://mcarton.github.io/rust-derivative/
[rustc]: https://github.com/rust-lang/rust
