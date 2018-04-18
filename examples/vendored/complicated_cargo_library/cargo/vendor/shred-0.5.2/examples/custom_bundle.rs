extern crate shred;

use shred::{Fetch, FetchMut, ResourceId, Resources, SystemData};

#[derive(Debug)]
struct ResA;

#[derive(Debug)]
struct ResB;

struct ExampleBundle<'a> {
    a: Fetch<'a, ResA>,
    b: FetchMut<'a, ResB>,
}

impl<'a> SystemData<'a> for ExampleBundle<'a> {
    fn fetch(res: &'a Resources, id: usize) -> Self {
        ExampleBundle {
            a: res.fetch(id),
            b: res.fetch_mut(id),
        }
    }

    fn reads(id: usize) -> Vec<ResourceId> {
        vec![ResourceId::new_with_id::<ResA>(id)]
    }

    fn writes(id: usize) -> Vec<ResourceId> {
        vec![ResourceId::new_with_id::<ResB>(id)]
    }
}

fn main() {
    let mut res = Resources::new();
    res.add(ResA);
    res.add(ResB);


    let mut bundle = ExampleBundle::fetch(&res, 0);
    *bundle.b = ResB;

    println!("{:?}", *bundle.a);
    println!("{:?}", *bundle.b);
}
