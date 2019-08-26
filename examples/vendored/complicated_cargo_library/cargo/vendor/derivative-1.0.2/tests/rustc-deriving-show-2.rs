#![allow(dead_code)]
#![deny(unused_variables)] // We used to generate warning: unused variable: `f`

#[cfg(feature = "use_core")]
extern crate core;

#[macro_use]
extern crate derivative;

use std::fmt;

#[derive(Derivative)]
#[derivative(Debug)]
enum A {}
#[derive(Derivative)]
#[derivative(Debug)]
enum B { B1, B2, B3 }

#[derive(Derivative)]
#[derivative(Debug)]
enum C { C1(isize), C2(B), C3(String) }

#[derive(Derivative)]
#[derivative(Debug)]
enum D { D1{ a: isize } }

#[derive(Derivative)]
#[derivative(Debug)]
struct E;
#[derive(Derivative)]
#[derivative(Debug)]
struct F(isize);
#[derive(Derivative)]
#[derivative(Debug)]
struct G(isize, isize);
#[derive(Derivative)]
#[derivative(Debug)]
struct H { a: isize }
#[derive(Derivative)]
#[derivative(Debug)]
struct I { a: isize, b: isize }
#[derive(Derivative)]
#[derivative(Debug)]
struct J(Custom);

struct Custom;
impl fmt::Debug for Custom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "yay")
    }
}

trait ToDebug {
    fn to_show(&self) -> String;
}

impl<T: fmt::Debug> ToDebug for T {
    fn to_show(&self) -> String {
        format!("{:?}", self)
    }
}

#[test]
fn main() {
    assert_eq!(B::B1.to_show(), "B1".to_string());
    assert_eq!(B::B2.to_show(), "B2".to_string());
    assert_eq!(C::C1(3).to_show(), "C1(3)".to_string());
    assert_eq!(C::C2(B::B2).to_show(), "C2(B2)".to_string());
    assert_eq!(D::D1{ a: 2 }.to_show(), "D1 { a: 2 }".to_string());
    assert_eq!(E.to_show(), "E".to_string());
    assert_eq!(F(3).to_show(), "F(3)".to_string());
    assert_eq!(G(3, 4).to_show(), "G(3, 4)".to_string());
    assert_eq!(I{ a: 2, b: 4 }.to_show(), "I { a: 2, b: 4 }".to_string());
    assert_eq!(J(Custom).to_show(), "J(yay)".to_string());
}
