use fnv::FnvHashMap;

use dispatch::Dispatcher;
use dispatch::dispatcher::{SystemId, ThreadLocal};
use dispatch::stage::StagesBuilder;
use system::{RunNow, System};

/// Builder for the [`Dispatcher`].
///
/// [`Dispatcher`]: struct.Dispatcher.html
///
/// ## Barriers
///
/// Barriers are a way of sequentializing parts of
/// the system execution. See `add_barrier()`.
///
/// ## Examples
///
/// This is how you create a dispatcher with
/// a shared thread pool:
///
/// ```rust
/// # #![allow(unused)]
/// #
/// # extern crate shred;
/// # #[macro_use]
/// # extern crate shred_derive;
/// # use shred::{Dispatcher, DispatcherBuilder, Fetch, System};
/// # #[derive(Debug)] struct Res;
/// # #[derive(SystemData)] #[allow(unused)] struct Data<'a> { a: Fetch<'a, Res> }
/// # struct Dummy;
/// # impl<'a> System<'a> for Dummy {
/// #   type SystemData = Data<'a>;
/// #
/// #   fn run(&mut self, _: Data<'a>) {}
/// # }
/// #
/// # fn main() {
/// # let system_a = Dummy;
/// # let system_b = Dummy;
/// # let system_c = Dummy;
/// # let system_d = Dummy;
/// # let system_e = Dummy;
/// let dispatcher: Dispatcher = DispatcherBuilder::new()
///     .add(system_a, "a", &[])
///     .add(system_b, "b", &["a"]) // b depends on a
///     .add(system_c, "c", &["a"]) // c also depends on a
///     .add(system_d, "d", &[])
///     .add(system_e, "e", &["c", "d"]) // e executes after c and d are finished
///     .build();
/// # }
/// ```
///
#[derive(Default)]
pub struct DispatcherBuilder<'a, 'b> {
    current_id: usize,
    map: FnvHashMap<String, SystemId>,
    stages_builder: StagesBuilder<'a>,
    thread_local: ThreadLocal<'b>,
    #[cfg(not(target_os = "emscripten"))]
    thread_pool:
        Option<::std::sync::Arc<::rayon::ThreadPool>>,
}

impl<'a, 'b> DispatcherBuilder<'a, 'b> {
    /// Creates a new `DispatcherBuilder` by using the `Default` implementation.
    ///
    /// The default behaviour is to create a thread pool on `finish`.
    /// If you already have a rayon `ThreadPool`, it's highly recommended to configure
    /// this builder to use it with `with_pool` instead.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a new system with a given name and a list of dependencies.
    /// Please note that the dependency should be added before
    /// you add the depending system.
    ///
    /// If you want to register systems which can not be specified as
    /// dependencies, you can use `""` as their name, which will not panic
    /// (using another name twice will).
    ///
    /// # Panics
    ///
    /// * if the specified dependency does not exist
    /// * if a system with the same name was already registered.
    pub fn add<T>(mut self, system: T, name: &str, dep: &[&str]) -> Self
    where
        T: for<'c> System<'c> + Send + 'a,
    {
        use std::collections::hash_map::Entry;

        let id = self.next_id();

        let dependencies = dep.iter()
            .map(|x| {
                *self.map
                    .get(*x)
                    .expect(&format!("No such system registered (\"{}\")", *x))
            })
            .collect();

        if name != "" {
            if let Entry::Vacant(e) = self.map.entry(name.to_owned()) {
                e.insert(id);
            } else {
                panic!(
                    "Cannot insert multiple systems with the same name (\"{}\")",
                    name
                );
            }
        }

        self.stages_builder.insert(dependencies, id, system);
        self
    }

    /// Adds a new thread local system.
    ///
    /// Please only use this if your struct is not `Send` and `Sync`.
    ///
    /// Thread-local systems are dispatched in-order.
    pub fn add_thread_local<T>(mut self, system: T) -> Self
    where
        T: for<'c> RunNow<'c> + 'b,
    {
        self.thread_local.push(Box::new(system));
        self
    }

    /// Inserts a barrier which assures that all systems
    /// added before the barrier are executed before the ones
    /// after this barrier.
    ///
    /// Does nothing if there were no systems added
    /// since the last call to `add_barrier()`.
    ///
    /// Thread-local systems are not affected by barriers;
    /// they're always executed at the end.
    pub fn add_barrier(mut self) -> Self {
        self.stages_builder.add_barrier();

        self
    }

    /// Attach a rayon thread pool to the builder
    /// and use that instead of creating one.
    #[cfg(not(target_os = "emscripten"))]
    pub fn with_pool(mut self, pool: ::std::sync::Arc<::rayon::ThreadPool>) -> Self {
        self.thread_pool = Some(pool);

        self
    }

    /// Builds the `Dispatcher`.
    ///
    /// In the future, this method will
    /// precompute useful information in
    /// order to speed up dispatching.
    pub fn build(self) -> Dispatcher<'a, 'b> {
        use dispatch::dispatcher::new_dispatcher;

        #[cfg(not(target_os = "emscripten"))]
        let d = new_dispatcher(
            self.stages_builder.build(),
            self.thread_local,
            self.thread_pool.unwrap_or_else(Self::create_thread_pool),
        );

        #[cfg(target_os = "emscripten")]
        let d = new_dispatcher(self.stages_builder.build(), self.thread_local);

        d
    }

    fn next_id(&mut self) -> SystemId {
        let id = self.current_id;
        self.current_id += 1;

        SystemId(id)
    }

    #[cfg(not(target_os = "emscripten"))]
    fn create_thread_pool() -> ::std::sync::Arc<::rayon::ThreadPool> {
        use std::sync::Arc;
        use rayon::{Configuration, ThreadPool};

        Arc::new(
            ThreadPool::new(Configuration::new()).expect("Invalid thread pool configuration"),
        )
    }
}

#[cfg(not(target_os = "emscripten"))]
impl<'b> DispatcherBuilder<'static, 'b> {
    /// Builds an async dispatcher.
    ///
    /// It does not allow non-static types and
    /// accepts a `Resource` struct.
    pub fn build_async<R>(self, res: R) -> ::dispatch::async::AsyncDispatcher<'b, R> {
        use dispatch::async::new_async;

        new_async(
            res,
            self.stages_builder.build(),
            self.thread_local,
            self.thread_pool.unwrap_or_else(Self::create_thread_pool),
        )
    }
}
