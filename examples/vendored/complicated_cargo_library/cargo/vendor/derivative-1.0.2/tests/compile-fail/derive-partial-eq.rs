#[macro_use]
extern crate derivative;

#[derive(Derivative)]
//~^ ERROR 4:10: 4:20: proc-macro derive panicked
//~| HELP can't use `#[derivative(PartialEq)]` on an enumeration without `feature_allow_slow_enum`
#[derivative(PartialEq)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {}
