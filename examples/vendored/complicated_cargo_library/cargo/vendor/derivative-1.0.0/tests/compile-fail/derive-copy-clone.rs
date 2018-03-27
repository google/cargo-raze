#[macro_use]
extern crate derivative;

#[derive(Derivative)]
//~^ ERROR custom derive attribute panicked
//~| HELP `#[derivative(Copy)]` can't be used with `#[derive(Clone)]`
#[derivative(Copy)]
#[derive(Clone)]
struct OurTheir1;

fn main() {
}
