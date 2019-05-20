#[cfg(feature = "use_core")]
extern crate core;

#[macro_use]
extern crate derivative;

use std::marker::PhantomData;

#[derive(Derivative)]
#[derivative(Debug)]
struct Foo<T, U> {
    foo: T,
    #[derivative(Debug="ignore")]
    bar: U,
}

#[derive(Derivative)]
#[derivative(Debug)]
struct Bar<T, U> (
    T,
    #[derivative(Debug="ignore")]
    U,
);

#[derive(Derivative)]
#[derivative(Debug)]
enum C<T, U> {
    C1(T),
    C2(#[derivative(Debug="ignore")] U),
    C3(String),
}

#[derive(Derivative)]
#[derivative(Debug)]
enum D<U> {
    D1{
        #[derivative(Debug="ignore")]
        a: U
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct F<U>(#[derivative(Debug="ignore")] U);

#[derive(Derivative)]
#[derivative(Debug)]
struct G<U>(isize, #[derivative(Debug="ignore")] U);

#[derive(Derivative)]
#[derivative(Debug)]
struct J<U>(#[derivative(Debug="ignore")] U);

struct NoDebug;

trait ToDebug {
    fn to_show(&self) -> String;
}

impl<T: std::fmt::Debug> ToDebug for T {
    fn to_show(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct PhantomField<T> {
    foo: PhantomData<T>,
}

#[derive(Derivative)]
#[derivative(Debug)]
struct PhantomTuple<T> {
    foo: PhantomData<(T,)>,
}

#[test]
fn main() {
    assert_eq!(Foo { foo: 42, bar: NoDebug }.to_show(), "Foo { foo: 42 }".to_string());
    assert_eq!(Bar(42, NoDebug).to_show(), "Bar(42)".to_string());
    assert_eq!(C::C1::<i32, NoDebug>(12).to_show(), "C1(12)".to_string());
    assert_eq!(C::C2::<i32, NoDebug>(NoDebug).to_show(), "C2".to_string());
    assert_eq!(C::C3::<i32, NoDebug>("foo".to_string()).to_show(), "C3(\"foo\")".to_string());
    assert_eq!(D::D1 { a: NoDebug }.to_show(), "D1".to_string());
    assert_eq!(F(NoDebug).to_show(), "F".to_string());
    assert_eq!(G(42, NoDebug).to_show(), "G(42)".to_string());
    assert_eq!(J(NoDebug).to_show(), "J".to_string());
    assert_eq!(&format!("{:?}", PhantomField::<NoDebug> { foo: Default::default() }), "PhantomField { foo: PhantomData }");
    assert_eq!(&format!("{:?}", PhantomTuple::<NoDebug> { foo: Default::default() }), "PhantomTuple { foo: PhantomData }");
}
