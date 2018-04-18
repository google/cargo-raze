#![allow(unused)]

extern crate shred;
#[macro_use]
extern crate shred_derive;

use std::fmt::Debug;

use shred::{Fetch, FetchMut, Resource};

trait Hrtb<'a> {}

#[derive(SystemData)]
struct VeryCustomDerive<'a, T: Debug + Resource + for<'b> Hrtb<'b>> {
    _b: FetchMut<'a, T>,
}

#[derive(SystemData)]
struct SomeTuple<'a, T: Debug + Resource>(Fetch<'a, T>);

#[derive(SystemData)]
struct WithWhereClause<'a, T>
where
    T: Resource,
{
    k: Fetch<'a, T>,
}

fn main() {}
