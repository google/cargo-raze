#[macro_use]
extern crate log;
extern crate futures;
extern crate env_logger;

use futures::future::ok;
use futures::executor::block_on;
use futures::Future;

fn return_a_future() -> Box<Future<Item=u64, Error=()>> {

    Box::new(ok(42))
}

pub fn main() {
  env_logger::init();
  trace!("Getting prepared to call a future");

  let f = return_a_future();
  let result = block_on(f).unwrap();

  info!("Got result from future: {:?}", result);
}
