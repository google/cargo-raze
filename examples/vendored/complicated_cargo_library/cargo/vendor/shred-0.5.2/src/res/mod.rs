//! Module for resource related types

pub use self::entry::Entry;

use std::any::TypeId;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use fnv::FnvHashMap;
use mopa::Any;

use self::entry::create_entry;
use cell::{Ref, RefMut, TrustCell};
use system::SystemData;

mod entry;

const RESOURCE_NOT_FOUND: &str = "No resource with the given id";

/// Return value of [`Resources::fetch`].
///
/// [`Resources::fetch`]: struct.Resources.html#method.fetch
pub struct Fetch<'a, T: 'a> {
    inner: Ref<'a, Box<Resource>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Deref for Fetch<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> SystemData<'a> for Fetch<'a, T>
where
    T: Resource,
{
    fn fetch(res: &'a Resources, id: usize) -> Self {
        res.fetch(id)
    }

    fn reads(id: usize) -> Vec<ResourceId> {
        vec![ResourceId::new_with_id::<T>(id)]
    }

    fn writes(_: usize) -> Vec<ResourceId> {
        vec![]
    }
}

/// Return value of [`Resources::fetch_id`].
///
/// [`Resources::fetch_id`]: struct.Resources.html#method.fetch_id
pub struct FetchId<'a> {
    inner: Ref<'a, Box<Resource>>,
}

impl<'a> Deref for FetchId<'a> {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        self.inner.as_ref()
    }
}

/// Return value of [`Resources::fetch_id_mut`].
///
/// [`Resources::fetch_id_mut`]: struct.Resources.html#method.fetch_id_mut
pub struct FetchIdMut<'a> {
    inner: RefMut<'a, Box<Resource>>,
}

impl<'a> Deref for FetchIdMut<'a> {
    type Target = Resource;

    fn deref(&self) -> &Resource {
        self.inner.as_ref()
    }
}

impl<'a> DerefMut for FetchIdMut<'a> {
    fn deref_mut(&mut self) -> &mut Resource {
        self.inner.as_mut()
    }
}

/// Return value of [`Resources::fetch_mut`].
///
/// [`Resources::fetch_mut`]: struct.Resources.html#method.fetch_mut
pub struct FetchMut<'a, T: 'a> {
    inner: RefMut<'a, Box<Resource>>,
    phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Deref for FetchMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> DerefMut for FetchMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.inner.downcast_mut_unchecked() }
    }
}

impl<'a, T> SystemData<'a> for FetchMut<'a, T>
where
    T: Resource,
{
    fn fetch(res: &'a Resources, id: usize) -> Self {
        res.fetch_mut(id)
    }

    fn reads(_: usize) -> Vec<ResourceId> {
        vec![]
    }

    fn writes(id: usize) -> Vec<ResourceId> {
        vec![ResourceId::new_with_id::<T>(id)]
    }
}

impl<'a, T> SystemData<'a> for Option<Fetch<'a, T>>
where
    T: Resource,
{
    fn fetch(res: &'a Resources, id: usize) -> Self {
        res.try_fetch(id)
    }

    fn reads(id: usize) -> Vec<ResourceId> {
        vec![ResourceId::new_with_id::<T>(id)]
    }

    fn writes(_: usize) -> Vec<ResourceId> {
        vec![]
    }
}

impl<'a, T> SystemData<'a> for Option<FetchMut<'a, T>>
where
    T: Resource,
{
    fn fetch(res: &'a Resources, id: usize) -> Self {
        res.try_fetch_mut(id)
    }

    fn reads(_: usize) -> Vec<ResourceId> {
        vec![]
    }

    fn writes(id: usize) -> Vec<ResourceId> {
        vec![ResourceId::new_with_id::<T>(id)]
    }
}

/// A resource defines a set of data
/// which can only be accessed according
/// to Rust's typical borrowing model (one writer xor multiple readers).
pub trait Resource: Any + Send + Sync {}

mopafy!(Resource);

impl<T> Resource for T
where
    T: Any + Send + Sync,
{
}

/// The id of a [`Resource`],
/// which is a tuple struct with a type
/// id and an additional resource id (represented with a `usize`).
///
/// The default resource id is `0`.
///
/// [`Resource`]: trait.Resource.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId(pub TypeId, pub usize);

impl ResourceId {
    /// Creates a new resource id from
    /// a given type with the default
    /// extra id.
    pub fn new<T: Resource>() -> Self {
        Self::new_with_id::<T>(0)
    }

    /// Creates a new resource id from
    /// a given type and an additional id.
    pub fn new_with_id<T: Resource>(id: usize) -> Self {
        ResourceId(TypeId::of::<T>(), id)
    }
}

/// A resource container, which
/// provides methods to access to
/// the contained resources.
///
/// # Resource Ids
///
/// Resources are in general identified
/// by `ResourceId`, which consists of a `TypeId`
/// and a `usize`. The `usize` may be used as
/// an additional identifier. In many cases, there
/// are convenience methods which assume this id is `0`.
#[derive(Default)]
pub struct Resources {
    resources: FnvHashMap<ResourceId, TrustCell<Box<Resource>>>,
}

impl Resources {
    /// Creates a new, empty resource container.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a new resource to this container.
    ///
    /// This method calls `add_with_id` with
    /// 0 for the id.
    ///
    /// # Panics
    ///
    /// Panics if the resource is already registered.
    ///
    /// # Examples
    ///
    /// Every type satisfying `Any + Debug + Send + Sync`
    /// automatically implements `Resource`:
    ///
    /// ```rust
    /// # #![allow(dead_code)]
    /// #[derive(Debug)]
    /// struct MyRes(i32);
    /// ```
    ///
    /// When you have a resource, simply
    /// register it like this:
    ///
    /// ```rust
    /// # #[derive(Debug)] struct MyRes(i32);
    /// use shred::Resources;
    ///
    /// let mut res = Resources::new();
    /// res.add(MyRes(5));
    /// ```
    pub fn add<R>(&mut self, r: R)
    where
        R: Resource,
    {
        self.add_with_id(r, 0)
    }

    /// Like `add()`, but allows specifying
    /// and id while `add()` assumes `0`.
    pub fn add_with_id<R>(&mut self, r: R, id: usize)
    where
        R: Resource,
    {
        use std::collections::hash_map::Entry;

        let entry = self.resources.entry(ResourceId::new_with_id::<R>(id));

        if let Entry::Vacant(e) = entry {
            e.insert(TrustCell::new(Box::new(r)));
        } else {
            panic!("Tried to add a resource though it is already registered");
        }
    }

    /// Returns true if the specified type / id combination
    /// is registered.
    pub fn has_value(&self, res_id: ResourceId) -> bool {
        self.resources.contains_key(&res_id)
    }

    /// Returns an entry for the resource with type `R` and id 0.
    pub fn entry<R>(&mut self) -> Entry<R>
    where
        R: Resource,
    {
        create_entry(self.resources.entry(ResourceId::new::<R>()))
    }

    /// Fetches the resource with the specified type `T`.
    /// The id is useful if you don't define your resources
    /// in Rust or you want a more dynamic resource handling.
    /// By default, the `#[derive(SystemData)]` passes `()`
    /// as id.
    ///
    /// # Panics
    ///
    /// Panics if the resource is being accessed mutably.
    /// Also panics if there is no such resource.
    pub fn fetch<T>(&self, id: usize) -> Fetch<T>
    where
        T: Resource,
    {
        self.try_fetch(id).expect(RESOURCE_NOT_FOUND)
    }

    /// Like `fetch`, but returns an `Option` instead of panicking in the case of the resource
    /// being accessed mutably.
    pub fn try_fetch<T>(&self, id: usize) -> Option<Fetch<T>>
    where
        T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>(), id).map(|r| {
            Fetch {
                inner: r.borrow(),
                phantom: PhantomData,
            }
        })
    }

    /// Fetches the resource with the specified type `T` mutably.
    ///
    /// Please see `fetch` for details.
    pub fn fetch_mut<T>(&self, id: usize) -> FetchMut<T>
    where
        T: Resource,
    {
        self.try_fetch_mut(id).expect(RESOURCE_NOT_FOUND)
    }

    /// Like `fetch_mut`, but returns an `Option` instead of panicking in the case of the resource
    /// being accessed mutably.
    pub fn try_fetch_mut<T>(&self, id: usize) -> Option<FetchMut<T>>
    where
        T: Resource,
    {
        self.try_fetch_internal(TypeId::of::<T>(), id).map(|r| {
            FetchMut {
                inner: r.borrow_mut(),
                phantom: PhantomData,
            }
        })
    }

    /// Fetches the resource with the specified type id.
    ///
    /// Please see `fetch` for details.
    ///
    /// # Panics
    ///
    /// Panics if no resource with the id exists.
    pub fn fetch_id(&self, id: TypeId, comp_id: usize) -> FetchId {
        self.try_fetch_id(id, comp_id).expect(RESOURCE_NOT_FOUND)
    }

    /// Like `fetch_id`, but returns an `Option` rather than panicking.
    pub fn try_fetch_id(&self, id: TypeId, comp_id: usize) -> Option<FetchId> {
        self.try_fetch_internal(id, comp_id)
            .map(|r| FetchId { inner: r.borrow() })
    }

    /// Fetches the resource with the specified type id mutably.
    ///
    /// Please see `fetch` for details.
    ///
    /// # Panics
    ///
    /// Panics if no resource with the id exists.
    pub fn fetch_id_mut(&self, id: TypeId, comp_id: usize) -> FetchIdMut {
        self.try_fetch_id_mut(id, comp_id)
            .expect(RESOURCE_NOT_FOUND)
    }

    /// Like `fetch_id_mut`, but returns an `Option` rather than panicking.
    pub fn try_fetch_id_mut(&self, id: TypeId, comp_id: usize) -> Option<FetchIdMut> {
        self.try_fetch_internal(id, comp_id).map(|r| {
            FetchIdMut {
                inner: r.borrow_mut(),
            }
        })
    }

    fn try_fetch_internal(&self, id: TypeId, cid: usize) -> Option<&TrustCell<Box<Resource>>> {
        self.resources.get(&ResourceId(id, cid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Res;

    #[test]
    fn res_id() {
        assert_eq!(ResourceId::new::<Res>(), ResourceId::new_with_id::<Res>(0));
        assert_eq!(
            ResourceId::new_with_id::<Res>(5),
            ResourceId(TypeId::of::<Res>(), 5)
        );
    }

    #[test]
    fn fetch_aspects() {
        assert_eq!(
            Fetch::<Res>::reads(4),
            vec![ResourceId::new_with_id::<Res>(4)]
        );
        assert_eq!(Fetch::<Res>::writes(8), vec![]);

        let mut res = Resources::new();
        res.add_with_id(Res, 56);
        Fetch::<Res>::fetch(&res, 56);
    }

    #[test]
    fn fetch_mut_aspects() {
        assert_eq!(FetchMut::<Res>::reads(4), vec![]);
        assert_eq!(
            FetchMut::<Res>::writes(8),
            vec![ResourceId::new_with_id::<Res>(8)]
        );

        let mut res = Resources::new();
        res.add_with_id(Res, 56);
        FetchMut::<Res>::fetch(&res, 56);
    }

    #[test]
    fn add() {
        let mut res = Resources::new();
        res.add(Res);

        assert!(res.has_value(ResourceId::new::<Res>()));
        assert!(!res.has_value(ResourceId::new_with_id::<Res>(1)));
        assert!(!res.has_value(ResourceId::new_with_id::<Res>(1)));
    }

    #[allow(unused)]
    #[test]
    #[should_panic(expected = "Already borrowed")]
    fn read_write_fails() {
        let mut res = Resources::new();
        res.add(Res);

        let read = res.fetch::<Res>(0);
        let write = res.fetch_mut::<Res>(0);
    }

    #[allow(unused)]
    #[test]
    #[should_panic(expected = "Already borrowed mutably")]
    fn write_read_fails() {
        let mut res = Resources::new();
        res.add(Res);

        let write = res.fetch_mut::<Res>(0);
        let read = res.fetch::<Res>(0);
    }

    #[test]
    fn fetch_uses_id() {
        let mut res = Resources::new();
        res.add_with_id(5i32, 1);
        res.add_with_id(50i32, 2);

        {
            assert_eq!(*res.fetch::<i32>(1), 5);
            assert_eq!(*res.fetch::<i32>(2), 50);
        }

        {
            *res.fetch_mut::<i32>(1) *= 2;
            *res.fetch_mut::<i32>(2) *= 2;
        }

        {
            assert_eq!(*res.fetch::<i32>(1), 10);
            assert_eq!(*res.fetch::<i32>(2), 100);
        }
    }
}
