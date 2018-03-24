use std::borrow::Borrow;

use dispatch::util::check_intersection;
use res::{ResourceId, Resources};
use system::RunNow;
use system::System;

use rayon::{join, ThreadPool};

/// The "leave node" for the `Par` / `Seq` list.
pub struct Nil;

/// The `par!` macro may be used to easily create a structure
/// which runs things in parallel.
///
/// ## Examples
///
/// ```
/// #[macro_use(par)]
/// extern crate shred;
///
/// # use shred::System;
/// # struct SysA; impl<'a> System<'a> for SysA { type SystemData = (); fn run(&mut self, _: ()){}}
/// # struct SysB; impl<'a> System<'a> for SysB { type SystemData = (); fn run(&mut self, _: ()){}}
/// # struct SysC; impl<'a> System<'a> for SysC { type SystemData = (); fn run(&mut self, _: ()){}}
/// # fn main() {
/// par![
///     SysA,
///     SysB,
///     SysC,
/// ]
/// # ;}
/// ```
#[macro_export]
macro_rules! par {
    ($head:expr, $( $tail:expr ,)+) => {
        {
            $crate::Par::new($head)
                $( .with($tail) )*
        }
    };
}

/// The `seq!` macro may be used to easily create a structure
/// which runs things sequentially.
///
/// ## Examples
///
/// ```
/// #[macro_use(seq)]
/// extern crate shred;
///
/// # struct SysA;
/// # struct SysB;
/// # struct SysC;
/// # fn main() {
/// seq![
///     SysA,
///     SysB,
///     SysC,
/// ]
/// # ;}
/// ```
#[macro_export]
macro_rules! seq {
    ($head:expr, $( $tail:expr ,)+) => {
        {
            $crate::Seq::new($head)
                $( .with($tail) )*
        }
    };
}

impl<'a> System<'a> for Nil {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {}
}

/// Runs two tasks in parallel.
/// These two tasks are called `head` and `tail`
/// in the following documentation.
pub struct Par<H, T> {
    head: H,
    tail: T,
}

impl<H> Par<H, Nil> {
    /// Creates a new `Par` struct, with the tail being a no-op.
    pub fn new(head: H) -> Self {
        Par { head, tail: Nil }
    }

    /// Adds `sys` as the second job and returns a new `Par` struct
    /// with the previous struct as head and a no-op tail.
    pub fn with<T>(self, sys: T) -> Par<Par<H, T>, Nil>
    where
        H: for<'a> RunWithPool<'a>,
        T: for<'a> RunWithPool<'a>,
    {
        debug_assert!(
            {
                let mut reads = Vec::new();
                let mut writes = Vec::new();
                self.head.reads(&mut reads);
                self.head.writes(&mut writes);

                let mut sys_reads = Vec::new();
                let mut sys_writes = Vec::new();
                sys.reads(&mut sys_reads);
                sys.writes(&mut sys_writes);

                !(check_intersection(writes.iter(), sys_reads.iter()) ||
                    check_intersection(writes.iter(), sys_writes.iter()) ||
                    check_intersection(reads.iter(), sys_writes.iter()))
            },
            "Tried to add system with conflicting reads / writes"
        );

        Par {
            head: Par {
                head: self.head,
                tail: sys,
            },
            tail: Nil,
        }
    }
}

/// A dispatcher intended to be used with
/// `Par` and `Seq`  structures.
///
/// This is more flexible and performant than `Dispatcher`,
/// however, you have to check conflicts yourself.
/// That means you cannot run two systems in parallel
/// which write to the same resource; if you'd do that,
/// one of the systems will panic while trying to fetch
/// the `SystemData`.
///
/// ## Thread-local systems
///
/// This dispatcher also allows more freedom
/// for thread-local systems; you can execute wherever you want,
/// just not in parallel with other systems (putting one inside
/// `par!` will give you a compile-time error saying the `Send` requirement
/// is unmet).
///
/// ## Examples
///
/// ```
/// # extern crate rayon;
/// #[macro_use(par, seq)]
/// extern crate shred;
///
/// # use rayon::ThreadPool;
/// #
/// # use shred::{ParSeq, Resources, System};
/// #
/// # macro_rules! impl_sys {
/// #     ($( $id:ident )*) => {
/// #         $(
/// #             impl<'a> ::shred::System<'a> for $id {
/// #                 type SystemData = ();
/// #                 fn run(&mut self, _: Self::SystemData) {}
/// #             }
/// #         )*
/// #     };
/// # }
/// #
/// # struct SysA;
/// # struct SysB;
/// # struct SysC;
/// # struct SysD;
/// # struct SysWithLifetime<'a>(&'a u8);
/// # struct SysLocal(*const u8);
/// #
/// # impl_sys!(SysA SysB SysC SysD SysLocal);
/// #
/// # impl<'a, 'b> System<'a> for SysWithLifetime<'b> {
/// #     type SystemData = ();
/// #
/// #     fn run(&mut self, _: Self::SystemData) {}
/// # }
///
/// # fn main() {
/// # #![cfg_attr(rustfmt, rustfmt_skip)]
/// #
/// # let pool = ThreadPool::new(Default::default()).unwrap();
/// #
/// # let mut res = Resources::new();
/// let x = 5u8;
///
/// let mut dispatcher = ParSeq::new(
///     seq![
///         par![
///             SysA,
///             SysWithLifetime(&x),
///             seq![
///                 SysC,
///                 SysD,
///             ],
///         ],
///         SysB,
///         SysLocal(&x as *const u8),
///     ],
///     &pool,
/// );
///
/// dispatcher.dispatch(&mut res);
/// # }
/// ```
pub struct ParSeq<P, T> {
    run: T,
    pool: P,
}

impl<P, T> ParSeq<P, T>
where
    P: Borrow<ThreadPool>,
    T: for<'a> RunWithPool<'a>,
{
    /// Creates a new `ParSeq` dispatcher.
    /// `run` is usually created by using the `par!` / `seq!`
    /// macros.
    pub fn new(run: T, pool: P) -> Self {
        ParSeq { run, pool }
    }

    /// Dispatches the systems using `res`.
    /// This doesn't call any virtual functions.
    ///
    /// Please note that this method assumes that no resource
    /// is currently borrowed. If that's the case, it panics.
    pub fn dispatch(&mut self, res: &Resources) {
        self.run.run(res, self.pool.borrow());
    }
}

pub trait RunWithPool<'a> {
    fn run(&mut self, res: &'a Resources, pool: &ThreadPool);

    fn reads(&self, reads: &mut Vec<ResourceId>);
    fn writes(&self, writes: &mut Vec<ResourceId>);
}

impl<'a, T> RunWithPool<'a> for T
where
    T: System<'a>,
{
    fn run(&mut self, res: &'a Resources, _: &ThreadPool) {
        RunNow::run_now(self, res);
    }

    fn reads(&self, reads: &mut Vec<ResourceId>) {
        use system::SystemData;

        reads.extend(T::SystemData::reads(0))
    }

    fn writes(&self, writes: &mut Vec<ResourceId>) {
        use system::SystemData;

        writes.extend(T::SystemData::writes(0))
    }
}

impl<'a, H, T> RunWithPool<'a> for Par<H, T>
where
    H: RunWithPool<'a> + Send,
    T: RunWithPool<'a> + Send,
{
    fn run(&mut self, res: &'a Resources, pool: &ThreadPool) {
        let head = &mut self.head;
        let tail = &mut self.tail;

        let head = move || head.run(res, pool);
        let tail = move || tail.run(res, pool);

        if pool.current_thread_index().is_none() {
            pool.join(head, tail);
        } else {
            join(head, tail);
        }
    }

    fn reads(&self, reads: &mut Vec<ResourceId>) {
        self.head.reads(reads);
        self.tail.reads(reads);
    }

    fn writes(&self, writes: &mut Vec<ResourceId>) {
        self.head.writes(writes);
        self.tail.writes(writes);
    }
}

/// Runs two tasks sequentially.
/// These two tasks are called `head` and `tail`
/// in the following documentation.
pub struct Seq<H, T> {
    head: H,
    tail: T,
}

impl<H> Seq<H, Nil> {
    /// Creates a new `Seq` struct, with the tail being a no-op.
    pub fn new(head: H) -> Self {
        Seq { head, tail: Nil }
    }

    /// Adds `sys` as the second job and returns a new `Seq` struct
    /// with the previous struct as head and a no-op tail.
    pub fn with<T>(self, sys: T) -> Seq<Seq<H, T>, Nil> {
        Seq {
            head: Seq {
                head: self.head,
                tail: sys,
            },
            tail: Nil,
        }
    }
}

impl<'a, H, T> RunWithPool<'a> for Seq<H, T>
where
    H: RunWithPool<'a>,
    T: RunWithPool<'a>,
{
    fn run(&mut self, res: &'a Resources, pool: &ThreadPool) {
        self.head.run(res, pool);
        self.tail.run(res, pool);
    }

    fn reads(&self, reads: &mut Vec<ResourceId>) {
        self.head.reads(reads);
        self.tail.reads(reads);
    }

    fn writes(&self, writes: &mut Vec<ResourceId>) {
        self.head.writes(writes);
        self.tail.writes(writes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::*;

    #[test]
    fn nested_joins() {
        let pool = ThreadPool::new(Default::default()).unwrap();

        pool.join(|| join(|| join(|| join(|| (), || ()), || ()), || ()), || ());
    }

    #[test]
    fn build_par() {
        let pool = ThreadPool::new(Default::default()).unwrap();

        struct A(Arc<AtomicUsize>);

        impl<'a> System<'a> for A {
            type SystemData = ();

            fn run(&mut self, _: Self::SystemData) {
                self.0.fetch_add(1, Ordering::AcqRel);
            }
        }

        let nr = Arc::new(AtomicUsize::new(0));

        Par::new(A(nr.clone()))
            .with(A(nr.clone()))
            .with(A(nr.clone()))
            .run(&Resources::new(), &pool);

        assert_eq!(nr.load(Ordering::Acquire), 3);

        par![A(nr.clone()), A(nr.clone()),].run(&Resources::new(), &pool);

        assert_eq!(nr.load(Ordering::Acquire), 5);
    }

    #[test]
    fn build_seq() {
        let pool = ThreadPool::new(Default::default()).unwrap();

        struct A(Arc<AtomicUsize>);

        impl<'a> System<'a> for A {
            type SystemData = ();

            fn run(&mut self, _: Self::SystemData) {
                self.0.fetch_add(1, Ordering::AcqRel);
            }
        }

        let nr = Arc::new(AtomicUsize::new(0));

        Seq::new(A(nr.clone()))
            .with(A(nr.clone()))
            .with(A(nr.clone()))
            .run(&Resources::new(), &pool);

        assert_eq!(nr.load(Ordering::Acquire), 3);
    }
}
