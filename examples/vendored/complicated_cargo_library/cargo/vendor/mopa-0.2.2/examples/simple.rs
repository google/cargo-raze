#[macro_use]
extern crate mopa;

use mopa::Any;

trait PanicAny: Any { }

mopafy!(PanicAny);

impl PanicAny for i32 { }

fn main() {
    let p: &PanicAny = &2;
    println!("{}", p.is::<i32>());
}
