# Custom attributes
The `Eq` and `PartialEq` traits support the following attributes:

* **Container attributes**
    * [`<Trait>(bound="<where-clause or empty>")`](#custom-bound)
* **Field attributes**
    * [`<Trait>(bound="<where-clause or empty>")`](#custom-bound)

The `PartialEq` trait also supports the following attributes:

* **Container attributes**
    * [`PartialEq="feature_allow_slow_enum"`](#enumerations)
* **Field attributes**
    * [`PartialEq="ignore"`](#ignoring-a-field)
    * [`PartialEq(compare_with="<path>")`](#compare-with)

# Enumerations

Unfortunatelly, there is no way for derivative to derive `PartialEq` on
enumerations as efficiently as the built-in `derive(PartialEq)`
[yet][discriminant].

If you want to use derivative on enumerations anyway, you can add

```rust
#[derivative(PartialEq="feature_allow_slow_enum")]
```

to your enumeration. This acts as a “feature-gate”.

# Ignoring a field

You can use *derivative* to ignore a field when comparing:

```rust
#[derive(Derivative)]
#[derivative(PartialEq)]
struct Foo {
    foo: u8,
    #[derivative(PartialEq="ignore")]
    bar: u8,
}

assert!(Foo { foo: 0, bar: 42 } == Foo { foo: 0, bar: 7});
assert!(Foo { foo: 42, bar: 0 } != Foo { foo: 7, bar: 0});
```

# Compare with

Usually fields are compared using `==`. You can use an alternative comparison
function if you like:

```rust
#[derive(Derivative)]
#[derivative(PartialEq)]
struct Foo {
    foo: u32,
    #[derivative(PartialEq(compare_with="path::to::my_cmp_fn"))]
    bar: SomeTypeThatMightNotBePartialEq,
}
```

`foo` will be compared with `==` and `bar` will be compared with
`path::to::my_cmp_fn` which must have the following prototype:

```rust
fn my_cmp_fn(&T, &T) -> bool;
```

# Custom bound

Usually a `T: Eq` bound is added for each type parameter `T`. You can use
override this behaviour if the infered bound is not correct for you.

Eg. comparing raw pointers does not require the type to be `Eq`, so you could
use:

```rust
#[derive(Derivative)]
#[derivative(Eq)]
struct WithPtr<T: ?Sized> {
    #[derivative(Eq(bound=""))]
    foo: *const T
}
```

See [`Default`'s documentation](./Default.md#custom-bound) for more details.

[discriminant]: https://github.com/rust-lang/rfcs/pull/1696
