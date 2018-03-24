#[cfg(not(target_os = "emscripten"))]
pub use self::async::AsyncDispatcher;
pub use self::builder::DispatcherBuilder;
pub use self::dispatcher::Dispatcher;
#[cfg(not(target_os = "emscripten"))]
pub use self::par_seq::{Par, ParSeq, Seq};

#[cfg(not(target_os = "emscripten"))]
mod async;
mod builder;
mod dispatcher;
#[cfg(not(target_os = "emscripten"))]
mod par_seq;
mod stage;
mod util;
