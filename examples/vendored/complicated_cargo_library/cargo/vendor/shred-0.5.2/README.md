# shred - *Sh*ared *re*source *d*ispatcher

[![Build Status][bi]][bl] [![Crates.io][ci]][cl] ![MIT/Apache][li] [![Docs.rs][di]][dl]

[bi]: https://travis-ci.org/slide-rs/shred.svg?branch=master
[bl]: https://travis-ci.org/slide-rs/shred

[ci]: https://img.shields.io/crates/v/shred.svg
[cl]: https://crates.io/crates/shred/

[li]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg

[di]: https://docs.rs/shred/badge.svg
[dl]: https://docs.rs/shred/

This library allows to dispatch
systems, which can have interdependencies,
shared and exclusive resource access, in parallel.

## Usage

```rust
extern crate shred;

use shred::{DispatcherBuilder, Fetch, FetchMut, Resource, Resources, System};

#[derive(Debug)]
struct ResA;

#[derive(Debug)]
struct ResB;

struct PrintSystem;

// Systems should be generic over the
// context if possible, so it's easy
// to introduce one.
impl<'a> System<'a> for PrintSystem {
    type SystemData = (Fetch<'a, ResA>, FetchMut<'a, ResB>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("{:?}", &*a);
        println!("{:?}", &*b);

        *b = ResB; // We can mutate ResB here
        // because it's `FetchMut`.
    }
}

fn main() {
    let mut resources = Resources::new();
    let mut dispatcher = DispatcherBuilder::new()
        .add(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        .build();
    resources.add(ResA);
    resources.add(ResB);

    dispatcher.dispatch(&mut resources);
}
```

Please see [the benchmark](benches/bench.rs) for a bigger (and useful) example.

### Required Rust version

`1.17 stable`

## Features

* lock-free
* no channels or similar functionality used (-> less overhead)
* allows lifetimes (opposed to `'static` only)

## Contribution

Contribution is highly welcome! If you'd like another
feature, just create an issue. You can also help
out if you want to; just pick a "help wanted" issue.
If you need any help, feel free to ask!

All contributions are assumed to be dual-licensed under
MIT/Apache-2.

## License

`shred` is distributed under the terms of both the MIT 
license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
