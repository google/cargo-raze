# Derivative

This crate provides a set of alternative `#[derive]` attributes for Rust.

## Examples

*derivative* uses attributes to make it possible to derive more implementations
than the built-in `derive(Trait)`. Here are a few examples of stuffs you cannot
just `derive`.

You can derive `Default` on enumerations:

| With *derivative* | [Original][default-enum-source] |
|-------------------|---------------------------------|
| {% codesnippet "default-enum.rs" %}{% endcodesnippet %} | {% codesnippet "default-enum-orig.rs" %}{% endcodesnippet %} |


You can use different default values for some fields:

| With *derivative* | [Original][default-value-source] |
|-------------------|---------------------------------|
| {% codesnippet "default-value.rs" %}{% endcodesnippet %} | {% codesnippet "default-value-orig.rs" %}{% endcodesnippet %} |


Want a transparent `Debug` implementation for your wrapper? We got that:

| With *derivative* | [Original][transparent-source] |
|-------------------|---------------------------------|
| {% codesnippet "debug-transparent.rs" %}{% endcodesnippet %} | {% codesnippet "debug-transparent-orig.rs" %}{% endcodesnippet %} |


Need to ignore a field? We got that too:

| With *derivative* | [Original][eq-ignore-source] |
|-------------------|---------------------------------|
| {% codesnippet "eq-ignore.rs" %}{% endcodesnippet %} | {% codesnippet "eq-ignore-orig.rs" %}{% endcodesnippet %} |


[default-value-source]: https://github.com/rust-lang-nursery/regex/blob/3cfef1e79d135a3e8a670aff53e7fabef453a3e1/src/re_builder.rs#L12-L39
[default-enum-source]: https://github.com/rust-lang/rust/blob/16eeeac783d2ede28e09f2a433c612dea309fe33/src/libcore/option.rs#L714-L718
[transparent-source]: https://github.com/rust-lang/rust/blob/5457c35ece57bbc4a65baff239a02d6abb81c8a2/src/libcore/num/mod.rs#L46-L54
[eq-ignore-source]: https://github.com/steveklabnik/semver/blob/baa0fbb57c80a7fb344fbeedac24a28439ddf5b5/src/version.rs#L196-L205
