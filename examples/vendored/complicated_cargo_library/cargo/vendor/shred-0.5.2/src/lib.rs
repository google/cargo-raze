//! **Sh**ared **re**source **d**ispatcher
//!
//! This library allows to dispatch
//! systems, which can have interdependencies,
//! shared and exclusive resource access, in parallel.
//!
//! # Examples
//!
//! ```rust
//! extern crate shred;
//! #[macro_use]
//! extern crate shred_derive;
//!
//! use shred::{DispatcherBuilder, Fetch, FetchMut, Resource, Resources, System};
//!
//! #[derive(Debug)]
//! struct ResA;
//!
//! #[derive(Debug)]
//! struct ResB;
//!
//! #[derive(SystemData)]
//! struct Data<'a> {
//!     a: Fetch<'a, ResA>,
//!     b: FetchMut<'a, ResB>,
//! }
//!
//! struct EmptySystem;
//!
//! impl<'a> System<'a> for EmptySystem {
//!     type SystemData = Data<'a>;
//!
//!     fn run(&mut self, bundle: Data<'a>) {
//!         println!("{:?}", &*bundle.a);
//!         println!("{:?}", &*bundle.b);
//!     }
//! }
//!
//!
//! fn main() {
//!     let mut resources = Resources::new();
//!     let mut dispatcher = DispatcherBuilder::new()
//!         .add(EmptySystem, "empty", &[])
//!         .build();
//!     resources.add(ResA);
//!     resources.add(ResB);
//!
//!     dispatcher.dispatch(&mut resources);
//! }
//! ```
//!
//! Once you are more familiar with how system data and parallelization works,
//! you can take look at a more flexible and performant way to dispatch: `ParSeq`.
//! Using it is bit trickier, but it allows dispatching without any virtual function calls.
//!

#![deny(unused_must_use)]
#![warn(missing_docs)]

extern crate arrayvec;
extern crate fnv;
#[macro_use]
extern crate mopa;
#[cfg(not(target_os = "emscripten"))]
extern crate pulse;
#[cfg(not(target_os = "emscripten"))]
extern crate rayon;
extern crate smallvec;

mod cell;
mod dispatch;
mod res;
mod system;

pub use dispatch::{Dispatcher, DispatcherBuilder};
#[cfg(not(target_os = "emscripten"))]
pub use dispatch::{Par, ParSeq, Seq};
#[cfg(not(target_os = "emscripten"))]
pub use dispatch::AsyncDispatcher;
pub use res::{Fetch, FetchId, FetchIdMut, FetchMut, Resource, ResourceId, Resources};
pub use system::{RunNow, RunningTime, System, SystemData};
