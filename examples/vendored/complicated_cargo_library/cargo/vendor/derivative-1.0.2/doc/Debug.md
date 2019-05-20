# Custom attributes
The `Debug` trait supports the following attributes:

* **Container attributes**
    * [`Debug(bound="<where-clause or empty>")`](#custom-bound)
    * [`Debug="transparent"`](#hiding-newtypes)
* **Variant attributes**
    * [`Debug="transparent"`](#hiding-newtypes)
* **Field attributes**
    * [`Debug(bound="<where-clause or empty>")`](#custom-bound)
    * [`Debug(format_with="<path>")`](#format-with)
    * [`Debug="ignore"`](#ignoring-a-field)

# Ignoring a field

You can use *derivative* to hide fields from a structure or enumeration `Debug`
implementation:

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

# Hiding newtypes

You can use *derivative* to automatically unwrap newtypes and enumeration
variants with only one field:

```rust
#[derive(Derivative)]
#[derivative(Debug="transparent")]
struct A(isize);

#[derive(Derivative)]
#[derivative(Debug)]
enum C {
    Foo(u8),
    #[derivative(Debug="transparent")]
    Bar(u8),
}

println!("{:?}", A(42)); // 42
println!("{:?}", C::Bar(42)); // 42

// But:
println!("{:?}", C::Foo(42)); // Foo(42)
```

# Format with

You can pass a field to a format function:

```rust
#[derive(Derivative)]
#[derivative(Debug)]
struct Foo {
    foo: u32,
    #[derivative(Debug(format_with="path::to::my_fmt_fn"))]
    bar: SomeTypeThatMightNotBeDebug,
}
```

The field `bar` will be displayed with `path::to::my_fmt_fn(&bar, &mut fmt)`
where `fmt` is the current [`Formatter`].

The function must the following prototype:

```rust
fn fmt(&T, &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>;
```

# Custom bound

Usually, *derivative* will add a `T: Debug` bound for each type parameter `T`
of the current type. If you do not want that, you can specify an explicit bound:

* Either on the type. This replaces all bounds:

```rust
#[derive(Derivative)]
#[derivative(Debug(bound="T: Debug, U:MyDebug")]
struct Foo<T, U> {
    foo: T,
    #[derivative(Debug(format_with="MyDebug::my_fmt"))]
    bar: U,
}
```

* Or on a field. This replaces the bound *derivative* guessed for that field. The example below is equivalent to the above:

```rust
#[derive(Derivative)]
#[derivative(Debug)]
struct Foo<T, U> {
    foo: T,
    #[derivative(Debug(format_with="MyDebug::my_fmt", bound="U: MyDebug"))]
    bar: U,
}
```

With `bound=""` it is possible to remove any bound for the type. This is useful
if your type contains a `Foo<T>` that is `Debug` even if `T` is not.

[`Formatter`]: https://doc.rust-lang.org/std/fmt/struct.Formatter.html
