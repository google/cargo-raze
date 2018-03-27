use core::ops::{CoerceUnsized, Deref, DerefMut};
use core::marker::Unsize;

use Unpin;
use Pin;
use PinMut;

#[fundamental]
/// A PinBox is a box that pins the data inside it. It guarantees that that
/// data will not be moved out of it unless that data implements the
/// `Unpin` trait.
pub struct PinBox<T: ?Sized> {
    inner: Box<T>
}

impl<T> PinBox<T> {
    /// Pin a pointer to the heap.
    pub fn new(data: T) -> PinBox<T> {
        PinBox { inner: Box::new(data) }
    }

    /// Move the inner type out of the PinBox.
    ///
    /// This is unsafe because the type may be a type which is not safe to
    /// move.
    pub unsafe fn into_inner_unchecked(self) -> T {
        *self.inner
    }
}

impl<T: ?Sized> PinBox<T> {
    /// Get a mutable pinned reference to the data in this PinBox.
    pub fn as_pin<'a>(&'a self) -> Pin<'a, T> {
        unsafe { Pin::new_unchecked(&*self.inner) }
    }

    /// Get a mutable pinned reference to the data in this PinBox.
    pub fn as_pin_mut<'a>(&'a mut self) -> PinMut<'a, T> {
        unsafe { PinMut::new_unchecked(&mut*self.inner) }
    }

    /// Move the inner box out of this PinBox.
    ///
    /// This is unsafe because it is possible to move the interior type out of
    /// this box, and the interior type may be a type that is not safe to move.
    pub unsafe fn into_box_unchecked(self) -> Box<T> {
        self.inner
    }
}

impl<T: Unpin> PinBox<T> {
    /// Move the data from this PinBox onto the stack.
    pub fn into_inner(self) -> T {
        *self.inner
    }
}

impl<T: Unpin + ?Sized> PinBox<T> {
    /// Consume this PinBox and get the internal Box out of it.
    pub fn into_box(self) -> Box<T> {
        self.inner
    }
}

impl<T: Unpin + ?Sized> Deref for PinBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<T: Unpin + ?Sized> DerefMut for PinBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}

impl<T: ?Sized> From<Box<T>> for PinBox<T> {
    fn from(boxed: Box<T>) -> PinBox<T> {
        PinBox { inner: boxed }
    }
}

unsafe impl<T: ?Sized> Unpin for PinBox<T> { }

impl<T, U> CoerceUnsized<PinBox<U>> for PinBox<T> where
    T: Unsize<U> + ?Sized,
    U: ?Sized,
{ }
