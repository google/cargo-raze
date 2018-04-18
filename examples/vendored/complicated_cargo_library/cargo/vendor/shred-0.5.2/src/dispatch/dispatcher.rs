use smallvec::SmallVec;

use dispatch::stage::Stage;
use res::Resources;
use system::RunNow;

/// The dispatcher struct, allowing
/// systems to be executed in parallel.
pub struct Dispatcher<'a, 'b> {
    stages: Vec<Stage<'a>>,
    thread_local: ThreadLocal<'b>,
    #[cfg(not(target_os = "emscripten"))] thread_pool: ::std::sync::Arc<::rayon::ThreadPool>,
}

impl<'a, 'b> Dispatcher<'a, 'b> {
    /// Dispatch all the systems with given resources and context
    /// and then run thread local systems.
    ///
    /// This function automatically redirects to
    ///
    /// * [`dispatch_par`] in case it is supported
    /// * [`dispatch_seq`] otherwise
    ///
    /// and runs `dispatch_thread_local` afterwards.
    ///
    /// Please note that this method assumes that no resource
    /// is currently borrowed. If that's the case, it panics.
    ///
    /// [`dispatch_par`]: struct.Dispatcher.html#method.dispatch_par
    /// [`dispatch_seq`]: struct.Dispatcher.html#method.dispatch_seq
    pub fn dispatch(&mut self, res: &Resources) {
        #[cfg(not(target_os = "emscripten"))]
        self.dispatch_par(res);

        #[cfg(target_os = "emscripten")]
        self.dispatch_seq(res);

        self.dispatch_thread_local(res);
    }

    /// Dispatches the systems (except thread local systems)
    /// in parallel given the resources to operate on.
    ///
    /// This operation blocks the
    /// executing thread.
    ///
    /// Only available on platforms with
    /// multithreading support (so not on emscripten).
    ///
    /// Please note that this method assumes that no resource
    /// is currently borrowed. If that's the case, it panics.
    #[cfg(not(target_os = "emscripten"))]
    pub fn dispatch_par(&mut self, res: &Resources) {
        let stages = &mut self.stages;

        self.thread_pool.install(move || for stage in stages {
            stage.execute(res);
        });
    }

    /// Dispatches the systems (except thread local systems) sequentially.
    ///
    /// This is useful if parallel overhead is
    /// too big or the platform does not support multithreading.
    ///
    /// Please note that this method assumes that no resource
    /// is currently borrowed. If that's the case, it panics.
    pub fn dispatch_seq(&mut self, res: &Resources) {
        for stage in &mut self.stages {
            stage.execute_seq(res);
        }
    }

    /// Dispatch only thread local systems sequentially.
    ///
    /// Please note that this method assumes that no resource
    /// is currently borrowed. If that's the case, it panics.
    pub fn dispatch_thread_local(&mut self, res: &Resources) {
        for sys in &mut self.thread_local {
            sys.run_now(res);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystemId(pub usize);

pub type SystemExecSend<'b> = Box<for<'a> RunNow<'a> + Send + 'b>;
pub type ThreadLocal<'a> = SmallVec<[Box<for<'b> RunNow<'b> + 'a>; 4]>;

#[cfg(not(target_os = "emscripten"))]
pub fn new_dispatcher<'a, 'b>(
    stages: Vec<Stage<'a>>,
    thread_local: ThreadLocal<'b>,
    thread_pool: ::std::sync::Arc<::rayon::ThreadPool>,
) -> Dispatcher<'a, 'b> {
    Dispatcher {
        stages,
        thread_local,
        thread_pool,
    }
}

#[cfg(target_os = "emscripten")]
pub fn new_dispatcher<'a, 'b>(
    stages: Vec<Stage<'a>>,
    thread_local: ThreadLocal<'b>,
) -> Dispatcher<'a, 'b> {
    Dispatcher {
        stages,
        thread_local,
    }
}

#[cfg(test)]
mod tests {
    use dispatch::builder::DispatcherBuilder;
    use res::*;
    use system::*;

    struct Res(i32);

    struct Dummy(i32);

    impl<'a> System<'a> for Dummy {
        type SystemData = FetchMut<'a, Res>;

        fn run(&mut self, mut data: Self::SystemData) {
            if self.0 == 4 {
                // In second stage

                assert_eq!(data.0, 6);
            } else if self.0 == 5 {
                // In second stage

                assert_eq!(data.0, 10);
            }

            data.0 += self.0;
        }
    }

    struct Panic;

    impl<'a> System<'a> for Panic {
        type SystemData = ();

        fn run(&mut self, _: Self::SystemData) {
            panic!("Propagated panic");
        }
    }

    fn new_builder() -> DispatcherBuilder<'static, 'static> {
        DispatcherBuilder::new()
            .add(Dummy(0), "0", &[])
            .add(Dummy(1), "1", &[])
            .add(Dummy(2), "2", &[])
            .add(Dummy(3), "3", &["1"])
            .add_barrier()
            .add(Dummy(4), "4", &[])
            .add(Dummy(5), "5", &["4"])
    }

    fn new_resources() -> Resources {
        let mut res = Resources::new();
        res.add(Res(0));

        res
    }

    #[test]
    #[should_panic(expected = "Propagated panic")]
    fn dispatcher_panics() {
        DispatcherBuilder::new()
            .add(Panic, "p", &[])
            .build()
            .dispatch(&mut new_resources())
    }

    #[test]
    fn stages() {
        let mut d = new_builder().build();

        d.dispatch(&mut new_resources());
    }

    #[test]
    fn stages_async() {
        let mut d = new_builder().build_async(new_resources());

        d.dispatch();
    }
}
