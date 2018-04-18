use std::marker::PhantomData;

use {ResourceId, Resources};

/// Trait for fetching data and running systems. Automatically implemented for systems.
pub trait RunNow<'a> {
    /// Runs the system now.
    ///
    /// # Panics
    ///
    /// Panics if the system tries to fetch resources
    /// which are borrowed in an incompatible way already
    /// (tries to read from a resource which is already written to or
    /// tries to write to a resource which is read from).
    fn run_now(&mut self, res: &'a Resources);
}

impl<'a, T> RunNow<'a> for T
where
    T: System<'a>,
{
    fn run_now(&mut self, res: &'a Resources) {
        let data = T::SystemData::fetch(res, 0);
        self.run(data);
    }
}

#[repr(u8)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum RunningTime {
    VeryShort = 1,
    Short = 2,
    Average = 3,
    Long = 4,
    VeryLong = 5,
}

/// A `System`, executed with a
/// set of required [`Resource`]s.
///
/// [`Resource`]: trait.Resource.html
pub trait System<'a> {
    /// The resource bundle required
    /// to execute this system.
    ///
    /// To create such a resource bundle,
    /// simple derive `SystemData` for it.
    type SystemData: SystemData<'a>;

    /// Executes the system with the required system
    /// data.
    fn run(&mut self, data: Self::SystemData);

    /// Returns a hint how long the system needs for running.
    /// This is used to optimize the way they're executed (might
    /// allow more parallelization).
    ///
    /// Defaults to `RunningTime::Average`.
    fn running_time(&self) -> RunningTime {
        RunningTime::Average
    }
}

/// A struct implementing
/// system data indicates that it
/// bundles some resources which are
/// required for the execution.
pub trait SystemData<'a> {
    /// Creates a new resource bundle
    /// by fetching the required resources
    /// from the [`Resources`] struct.
    ///
    /// # Contract
    ///
    /// Only fetch the resources you
    /// returned from `reads` / `writes`!
    ///
    /// [`Resources`]: trait.Resources.html
    fn fetch(res: &'a Resources, id: usize) -> Self;

    /// A list of [`ResourceId`]s the bundle
    /// needs read access to in order to
    /// build the target resource bundle.
    ///
    /// # Contract
    ///
    /// Exactly return the dependencies you're
    /// going to `fetch`! Doing otherwise *will*
    /// cause a data race.
    ///
    /// This method is only executed once,
    /// thus the returned value may never change
    /// (otherwise it has no effect).
    fn reads(id: usize) -> Vec<ResourceId>;

    /// A list of [`ResourceId`]s the bundle
    /// needs write access to in order to
    /// build the target resource bundle.
    ///
    /// # Contract
    ///
    /// Exactly return the dependencies you're
    /// going to `fetch`! Doing otherwise *will*
    /// cause a data race.
    ///
    /// This method is only executed once,
    /// thus the returned value may never change
    /// (otherwise it has no effect).
    fn writes(id: usize) -> Vec<ResourceId>;
}

impl<'a, T: ?Sized> SystemData<'a> for PhantomData<T> {
    fn fetch(_: &'a Resources, _: usize) -> Self {
        PhantomData
    }

    fn reads(_: usize) -> Vec<ResourceId> {
        Vec::new()
    }

    fn writes(_: usize) -> Vec<ResourceId> {
        Vec::new()
    }
}

macro_rules! impl_data {
    ( $($ty:ident),* ) => {
        impl<'a, $($ty),*> SystemData<'a> for ( $( $ty , )* )
            where $( $ty : SystemData<'a> ),*
        {
            fn fetch(res: &'a Resources, id: usize) -> Self {
                #![allow(unused_variables)]

                ( $( <$ty as SystemData<'a>>::fetch(res, id.clone()), )* )
            }

            fn reads(id: usize) -> Vec<ResourceId> {
                #![allow(unused_mut)]

                let mut r = Vec::new();

                $( {
                        let mut reads = <$ty as SystemData>::reads(id.clone());
                        r.append(&mut reads);
                    } )*

                r
            }

            fn writes(id: usize) -> Vec<ResourceId> {
                #![allow(unused_mut)]

                let mut r = Vec::new();

                $( {
                        let mut writes = <$ty as SystemData>::writes(id.clone());
                        r.append(&mut writes);
                    } )*

                r
            }
        }
    };
}

impl<'a> SystemData<'a> for () {
    fn fetch(_: &'a Resources, _: usize) -> Self {
        ()
    }

    fn reads(_: usize) -> Vec<ResourceId> {
        Vec::new()
    }

    fn writes(_: usize) -> Vec<ResourceId> {
        Vec::new()
    }
}

mod impl_data {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    use super::*;

    impl_data!(A);
    impl_data!(A, B);
    impl_data!(A, B, C);
    impl_data!(A, B, C, D);
    impl_data!(A, B, C, D, E);
    impl_data!(A, B, C, D, E, F);
    impl_data!(A, B, C, D, E, F, G);
    impl_data!(A, B, C, D, E, F, G, H);
    impl_data!(A, B, C, D, E, F, G, H, I);
    impl_data!(A, B, C, D, E, F, G, H, I, J);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
    impl_data!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
}
