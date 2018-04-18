use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

use pulse::Signal;
use rayon::ThreadPool;

use dispatch::dispatcher::ThreadLocal;
use dispatch::stage::Stage;
use res::Resources;

const ERR_NO_DISPATCH: &str = "wait() called before dispatch or called twice";

/// Like, `Dispatcher` but works
/// asynchronously.
pub struct AsyncDispatcher<'a, R> {
    res: Arc<R>,
    signal: Option<Signal>,
    stages: Arc<Mutex<Vec<Stage<'static>>>>,
    thread_local: ThreadLocal<'a>,
    thread_pool: Arc<ThreadPool>,
}

pub fn new_async<'a, R>(
    res: R,
    stages: Vec<Stage<'static>>,
    thread_local: ThreadLocal<'a>,
    thread_pool: Arc<ThreadPool>,
) -> AsyncDispatcher<'a, R> {
    AsyncDispatcher {
        res: Arc::new(res),
        signal: None,
        stages: Arc::new(Mutex::new(stages)),
        thread_local: thread_local,
        thread_pool: thread_pool,
    }
}

impl<'a, R> AsyncDispatcher<'a, R>
where
    R: Borrow<Resources> + Send + Sync + 'static,
{
    /// Dispatches the systems asynchronously.
    /// Does not execute thread local systems.
    ///
    /// If you want to wait for the systems to finish,
    /// call `wait()`.
    pub fn dispatch(&mut self) {
        let (signal, pulse) = Signal::new();
        self.signal = Some(signal);

        let stages = self.stages.clone();
        let res = self.res.clone();

        self.thread_pool.spawn(move || {
            {
                let stages = stages;
                let mut stages = stages.lock().expect("Mutex poisoned");

                let res = res.as_ref().borrow();

                for stage in &mut *stages {
                    stage.execute(res);
                }
            }

            pulse.pulse();
        })
    }

    /// Waits for all the asynchronously dispatched systems to finish
    /// and executes thread local systems (if there are any).
    pub fn wait(&mut self) {
        self.wait_without_tl();

        let res = self.res.as_ref().borrow();

        for sys in &mut self.thread_local {
            sys.run_now(res);
        }
    }

    /// Waits for all the asynchronously dispatched systems to finish
    /// without executing thread local systems.
    pub fn wait_without_tl(&mut self) {
        self.signal
            .take()
            .expect(ERR_NO_DISPATCH)
            .wait()
            .expect("The worker thread may have panicked");
    }


    /// Checks if any of the asynchronously dispatched systems are running.
    pub fn is_running(&self) -> bool {
        if let Some(ref signal) = self.signal {
            signal.is_pending()
        } else {
            false
        }
    }

    /// Dispatch only thread local systems sequentially.
    ///
    /// If `wait_without_tl()` or `wait()` wasn't called before,
    /// this method will wait.
    pub fn dispatch_thread_local(&mut self) {
        if self.signal.is_some() {
            self.wait_without_tl();
        }

        let res = self.res.as_ref().borrow();

        for sys in &mut self.thread_local {
            sys.run_now(res);
        }
    }

    /// Returns the resources.
    ///
    /// If `wait_without_tl()` or `wait()` wasn't called before,
    /// this method will do that.
    pub fn mut_res(&mut self) -> &mut R {
        if self.signal.is_some() {
            self.wait();
        }

        Arc::get_mut(&mut self.res).expect(ERR_NO_DISPATCH)
    }
}
