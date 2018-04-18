extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{DispatcherBuilder, Fetch, FetchMut, Resources, System};

#[derive(Debug)]
struct ResA;

#[derive(Debug)]
struct ResB;

#[derive(SystemData)]
struct Data<'a> {
    a: Fetch<'a, ResA>,
    b: FetchMut<'a, ResB>,
}

struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = Data<'a>;

    fn run(&mut self, bundle: Data<'a>) {
        println!("{:?}", &*bundle.a);
        println!("{:?}", &*bundle.b);
    }
}

fn main() {
    let mut resources = Resources::new();
    let mut dispatcher = DispatcherBuilder::new()
        .add(EmptySystem, "empty", &[])
        .build();
    resources.add(ResA);
    resources.add(ResB);

    dispatcher.dispatch_seq(&resources);
}
