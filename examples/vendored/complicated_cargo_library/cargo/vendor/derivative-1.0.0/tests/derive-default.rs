#[macro_use]
extern crate derivative;

#[derive(Debug, Derivative, PartialEq)]
#[derivative(Default="new")]
struct Foo {
    foo: u8,
    #[derivative(Default(value="42"))]
    bar: u8,
}

#[derive(Debug, Derivative, PartialEq)]
#[derivative(Default(new="true"))]
struct Bar (
    u8,
    #[derivative(Default(value="42"))]
    u8,
);

#[derive(Debug, Derivative, PartialEq)]
#[derivative(Default)]
enum Enum1 {
    #[allow(dead_code)]
    A,
    #[derivative(Default)]
    B,
}

#[derive(Debug, Derivative, PartialEq)]
#[derivative(Default)]
enum Enum2 {
    #[derivative(Default)]
    A,
    #[allow(dead_code)]
    B,
}

#[derive(Debug, Derivative, PartialEq)]
#[derivative(Default)]
struct A(#[derivative(Default(value="NoDefault"))] NoDefault);

#[derive(Debug, PartialEq)]
struct NoDefault;

#[test]
fn main() {
    assert_eq!(Foo::default(), Foo { foo: 0, bar: 42 });
    assert_eq!(Foo::new(), Foo { foo: 0, bar: 42 });
    assert_eq!(Bar::default(), Bar(0, 42));
    assert_eq!(Bar::new(), Bar(0, 42));
    assert_eq!(A::default(), A(NoDefault));
    assert_eq!(Enum1::default(), Enum1::B);
    assert_eq!(Enum2::default(), Enum2::A);
}
