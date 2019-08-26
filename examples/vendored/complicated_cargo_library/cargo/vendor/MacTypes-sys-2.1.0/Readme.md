# MacTypes-sys

## Description:

 The `MacTypes-sys` library provides bindings to the MacTypes.h header on macOS.
This library defines base types used in both Carbon and legacy Cocoa APIs.

## Usage:

Add `MacTypes-sys` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
MacTypes-sys = "*"
```

Then, you can import the `MacTypes-sys` in your crate root, and use the functions defined:

```rust
extern crate MacTypes_sys;
```

Full documentation can be found on [docs.rs](https://docs.rs/MacTypes-sys).

## Contributors:

- [George Burton](https://github.com/burtonageo)

## License
Copyright © 2016 George Burton

Distributed under the [MIT License](License.md).
