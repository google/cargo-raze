use core::ops::{CoerceUnsized, Deref};
use core::marker::Unsize;

use Unpin;

#[fundamental]
/// A pinned reference.
///
/// The value referenced by this is guaranteed never to move again, unless it
/// implements `Unpin`.
pub struct Pin<'a, T: ?Sized + 'a> {
    inner: &'a T,
}

impl<'a, T: Unpin + ?Sized> Pin<'a, T> {
    /// Create a new Pin from a reference to a moveable type.
    pub fn new(ptr: &'a T) -> Pin<'a, T> {
        Pin { inner: ptr }
    }
}

impl<'a, T: ?Sized> Pin<'a, T> {
    /// Construct a new `Pin` without checking that the data is actually
    /// pinned.
    ///
    /// You must guarantee that the data meets the requirements for
    /// constructing a `Pin`.
    ///
    /// An example use case is constructing a `Pin` of a field of this type:
    ///
    /// ```ignore
    /// let inner = unsafe { Pin::new_unchecked(&this.inner) };
    /// ````
    pub unsafe fn new_unchecked(ptr: &'a T) -> Pin<'a, T> {
        Pin { inner: ptr }
    }

    /// Get a Pin with a shorter lifetime
    pub fn borrow<'b>(this: &'b Pin<'a, T>) -> Pin<'b, T> {
        Pin { inner: this.inner }
    }

    /// Get a reference to the data inside this type.
    ///
    /// This is unsafe, because you must guarantee that you do not move the
    /// data out of the reference that this function returns.
    pub unsafe fn get<'b>(this: &'b mut Pin<'a, T>) -> &'b T {
        this.inner
    }
}

impl<'a, T: ?Sized> Deref for Pin<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner
    }
}

impl<'a, T, U> CoerceUnsized<Pin<'a, U>> for Pin<'a, T> where
    T: Unsize<U> + ?Sized,
    U: ?Sized,
{ }
