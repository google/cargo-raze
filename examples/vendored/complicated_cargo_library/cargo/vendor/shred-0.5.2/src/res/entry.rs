use std::collections::hash_map::Entry as StdEntry;
use std::marker::PhantomData;

use cell::TrustCell;
use res::{FetchMut, Resource, ResourceId};

/// An entry to a resource of the `Resources` struct.
/// This is similar to the Entry API found in the standard library.
///
/// ## Examples
///
/// ```
/// use shred::Resources;
///
/// #[derive(Debug)]
/// struct Res(i32);
///
/// let mut res = Resources::new();
///
/// let value = res.entry().or_insert(Res(4));
/// println!("{:?}", value.0 * 2);
/// ```
pub struct Entry<'a, T: 'a> {
    inner: StdEntry<'a, ResourceId, TrustCell<Box<Resource>>>,
    marker: PhantomData<T>,
}

impl<'a, T> Entry<'a, T>
where
    T: Resource + 'a,
{
    /// Returns this entry's value, inserts and returns `v` otherwise.
    ///
    /// Please note that you should use `or_insert_with` in case the creation of the
    /// value is expensive.
    pub fn or_insert(self, v: T) -> FetchMut<'a, T> {
        self.or_insert_with(move || v)
    }

    /// Returns this entry's value, inserts and returns the return value of `f` otherwise.
    pub fn or_insert_with<F>(self, f: F) -> FetchMut<'a, T>
    where
        F: FnOnce() -> T,
    {
        let value = self.inner
            .or_insert_with(move || TrustCell::new(Box::new(f())));
        let inner = value.borrow_mut();

        FetchMut {
            inner,
            phantom: PhantomData,
        }
    }
}

pub fn create_entry<'a, T>(e: StdEntry<'a, ResourceId, TrustCell<Box<Resource>>>) -> Entry<'a, T> {
    Entry {
        inner: e,
        marker: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use res::Resources;

    #[test]
    fn test_entry() {
        struct Res;

        let mut res = Resources::new();
        res.entry().or_insert(Res);

        assert!(res.has_value(ResourceId::new::<Res>()));
    }
}
