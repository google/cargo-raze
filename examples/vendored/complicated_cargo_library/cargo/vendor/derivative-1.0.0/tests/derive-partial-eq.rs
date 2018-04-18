#[macro_use]
extern crate derivative;

#[derive(Derivative)]
#[derivative(PartialEq)]
struct Foo {
    foo: u8
}

#[derive(Derivative)]
#[derivative(PartialEq="feature_allow_slow_enum")]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
struct WithPtr<T: ?Sized> {
    #[derivative(PartialEq(bound=""))]
    foo: *const T
}

#[derive(Derivative)]
#[derivative(PartialEq)]
struct Empty;

#[derive(Derivative)]
#[derivative(PartialEq)]
struct AllIgnored {
    #[derivative(PartialEq="ignore")]
    foo: u8,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
struct OneIgnored {
    #[derivative(PartialEq="ignore")]
    foo: u8,
    bar: u8,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
struct Parity(
    #[derivative(PartialEq(compare_with="same_parity"))]
    u8,
);

fn same_parity(lhs: &u8, rhs: &u8) -> bool {
    lhs % 2 == rhs % 2
}

#[derive(Derivative)]
#[derivative(PartialEq)]
struct Generic<T>(
    #[derivative(PartialEq(compare_with="dummy_cmp", bound=""))]
    T,
);

fn dummy_cmp<T>(_: &T, _: &T) -> bool {
    true
}

trait SomeTrait {}
struct SomeType {
    #[allow(dead_code)]
    foo: u8
}
impl SomeTrait for SomeType {}

#[test]
fn main() {
    assert!(Foo { foo: 7 } == Foo { foo: 7 });
    assert!(Foo { foo: 7 } != Foo { foo: 42 });

    let ptr1: *const SomeTrait = &SomeType { foo: 0 };
    let ptr2: *const SomeTrait = &SomeType { foo: 1 };
    assert!(WithPtr { foo: ptr1 } == WithPtr { foo: ptr1 });
    assert!(WithPtr { foo: ptr1 } != WithPtr { foo: ptr2 });

    assert!(Empty == Empty);
    assert!(AllIgnored { foo: 0 } == AllIgnored { foo: 42 });
    assert!(OneIgnored { foo: 0, bar: 6 } == OneIgnored { foo: 42, bar: 6 });
    assert!(OneIgnored { foo: 0, bar: 6 } != OneIgnored { foo: 42, bar: 7 });

    assert!(Option::Some(42) == Option::Some(42));
    assert!(Option::Some(0) != Option::Some(42));
    assert!(Option::Some(42) != Option::None);
    assert!(Option::None != Option::Some(42));
    assert!(Option::None::<u8> == Option::None::<u8>);

    assert!(Parity(3) == Parity(7));
    assert!(Parity(2) == Parity(42));
    assert!(Parity(3) != Parity(42));
    assert!(Parity(2) != Parity(7));

    assert!(Generic(SomeType { foo: 0 }) == Generic(SomeType{ foo: 0 }));
}
