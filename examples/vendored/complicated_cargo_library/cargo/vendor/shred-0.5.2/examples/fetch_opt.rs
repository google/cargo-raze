extern crate shred;

use shred::{DispatcherBuilder, Fetch, FetchMut, Resources, System};

#[derive(Debug)]
struct ResA;

#[derive(Debug)]
struct ResB;

struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    // We can simply use `Option<Fetch>` or `Option<FetchMut>` if a resource
    // isn't strictly required.
    type SystemData = (Fetch<'a, ResA>, Option<FetchMut<'a, ResB>>);

    fn run(&mut self, data: Self::SystemData) {
        let (a, mut b) = data;

        println!("{:?}", &*a);

        if let Some(ref mut x) = b {
            println!("{:?}", &**x);

            **x = ResB;
        }
    }
}

fn main() {
    let mut resources = Resources::new();
    let mut dispatcher = DispatcherBuilder::new()
        .add(PrintSystem, "print", &[]) // Adds a system "print" without dependencies
        .build();
    resources.add(ResA);

    // `ResB` is not in resources, but `PrintSystem` still works.
    dispatcher.dispatch(&resources);

    resources.add(ResB);

    // Now `ResB` can be printed, too.
    dispatcher.dispatch(&resources);
}
