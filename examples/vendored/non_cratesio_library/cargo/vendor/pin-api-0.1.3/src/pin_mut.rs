use core::ops::{CoerceUnsized, Deref, DerefMut};
use core::marker::Unsize;

use Unpin;

#[fundamental]
/// A pinned mutable reference.
///
/// The value referenced by this is guaranteed never to move again, unless it
/// implements `Unpin`.
pub struct PinMut<'a, T: ?Sized + 'a> {
    inner: &'a mut T,
}

impl<'a, T: Unpin + ?Sized> PinMut<'a, T> {
    /// Create a new PinMut from a mutable reference to a moveable type.
    pub fn new(ptr: &'a mut T) -> PinMut<'a, T> {
        PinMut { inner: ptr }
    }
}

impl<'a, T: ?Sized> PinMut<'a, T> {
    /// Construct a new `PinMut` without checking that the data is actually
    /// pinned.
    ///
    /// You must guarantee that the data meets the requirements for
    /// constructing a `PinMut`.
    ///
    /// An example use case is constructing a `PinMut` of a field of this type:
    ///
    /// ```ignore
    /// let inner = unsafe { PinMut::new_unchecked(&mut this.inner) };
    /// ````
    pub unsafe fn new_unchecked(ptr: &'a mut T) -> PinMut<'a, T> {
        PinMut { inner: ptr }
    }

    /// Get a PinMut with a shorter lifetime
    pub fn borrow<'b>(this: &'b mut PinMut<'a, T>) -> PinMut<'b, T> {
        PinMut { inner: this.inner }
    }

    /// Get a reference to the data inside this type.
    ///
    /// This is unsafe, because you must guarantee that you do not move the
    /// data out of the reference that this function returns.
    pub unsafe fn get<'b>(this: &'b mut PinMut<'a, T>) -> &'b T {
        this.inner
    }

    /// Get a mutable reference to the data inside this type.
    ///
    /// This is unsafe, because you must guarantee that you do not move the
    /// data out of the mutable reference that this function returns.
    pub unsafe fn get_mut<'b>(this: &'b mut PinMut<'a, T>) -> &'b mut T {
        this.inner
    }
}

impl<'a, T: ?Sized> Deref for PinMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner
    }
}

impl<'a, T: Unpin + ?Sized> DerefMut for PinMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner
    }
}

impl<'a, T, U> CoerceUnsized<PinMut<'a, U>> for PinMut<'a, T> where
    T: Unsize<U> + ?Sized,
    U: ?Sized,
{ }
