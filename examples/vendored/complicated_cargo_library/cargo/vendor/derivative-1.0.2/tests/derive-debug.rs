#[cfg(feature = "use_core")]
extern crate core;

#[macro_use]
extern crate derivative;

#[derive(Derivative)]
#[derivative(Debug)]
struct Foo {
    foo: u8,
    #[derivative(Debug="ignore")]
    bar: u8,
}

#[derive(Derivative)]
#[derivative(Debug)]
struct Bar (
    u8,
    #[derivative(Debug="ignore")]
    u8,
);

#[derive(Derivative)]
#[derivative(Debug)]
enum C {
    C1(isize),
    C2(#[derivative(Debug="ignore")] i32),
    C3(String),
}

#[derive(Derivative)]
#[derivative(Debug)]
enum D {
    D1 {
        #[derivative(Debug="ignore")]
        a: isize
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct F(#[derivative(Debug="ignore")] isize);

#[derive(Derivative)]
#[derivative(Debug)]
struct G(isize, #[derivative(Debug="ignore")] isize);

#[derive(Derivative)]
#[derivative(Debug)]
struct J(#[derivative(Debug="ignore")] NoDebug);

struct NoDebug;

trait ToDebug {
    fn to_show(&self) -> String;
}

impl<T: std::fmt::Debug> ToDebug for T {
    fn to_show(&self) -> String {
        format!("{:?}", self)
    }
}

#[test]
fn main() {
    assert_eq!(Foo { foo: 42, bar: 1 }.to_show(), "Foo { foo: 42 }".to_string());
    assert_eq!(Bar(42, 1).to_show(), "Bar(42)".to_string());
    assert_eq!(C::C1(12).to_show(), "C1(12)".to_string());
    assert_eq!(C::C2(12).to_show(), "C2".to_string());
    assert_eq!(C::C3("foo".to_string()).to_show(), "C3(\"foo\")".to_string());
    assert_eq!(D::D1 { a: 42 }.to_show(), "D1".to_string());
    assert_eq!(F(42).to_show(), "F".to_string());
    assert_eq!(G(42, 0).to_show(), "G(42)".to_string());
    assert_eq!(J(NoDebug).to_show(), "J".to_string());
}
