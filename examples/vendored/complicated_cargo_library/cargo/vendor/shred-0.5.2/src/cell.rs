use std::cell::UnsafeCell;
use std::error::Error;
use std::fmt::{Display, Error as FormatError, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug)]
pub struct InvalidBorrow;

impl Display for InvalidBorrow {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "Tried to borrow when it was illegal")
    }
}

impl Error for InvalidBorrow {
    fn description(&self) -> &str {
        "This error is returned when you try to borrow immutably when it's already \
         borrowed mutably or you try to borrow mutably when it's already borrowed"
    }
}

#[derive(Debug)]
pub struct Ref<'a, T: 'a> {
    flag: &'a AtomicUsize,
    value: &'a T,
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        self.flag.fetch_sub(1, Ordering::Release);
    }
}

#[derive(Debug)]
pub struct RefMut<'a, T: 'a> {
    flag: &'a AtomicUsize,
    value: &'a mut T,
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        self.flag.store(0, Ordering::Release)
    }
}

/// A custom cell similar to
/// `RefCell`, but it is thread-safe.
#[derive(Debug)]
pub struct TrustCell<T> {
    flag: AtomicUsize,
    inner: UnsafeCell<T>,
}

impl<T> TrustCell<T> {
    pub fn new(val: T) -> Self {
        TrustCell {
            flag: AtomicUsize::new(0),
            inner: UnsafeCell::new(val),
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.check_flag_read().expect("Already borrowed mutably");

        Ref {
            flag: &self.flag,
            value: unsafe { &*self.inner.get() },
        }
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.check_flag_write().expect("Already borrowed");

        RefMut {
            flag: &self.flag,
            value: unsafe { &mut *self.inner.get() },
        }
    }

    fn check_flag_read(&self) -> Result<(), InvalidBorrow> {
        loop {
            let val = self.flag.load(Ordering::Acquire);

            if val == !0 {
                return Err(InvalidBorrow);
            }

            if self.flag.compare_and_swap(val, val + 1, Ordering::AcqRel) == val {
                return Ok(());
            }
        }
    }

    fn check_flag_write(&self) -> Result<(), InvalidBorrow> {
        match self.flag.compare_and_swap(0, !0, Ordering::AcqRel) {
            0 => Ok(()),
            _ => Err(InvalidBorrow),
        }
    }
}

unsafe impl<T> Sync for TrustCell<T>
where
    T: Sync,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi() {
        let cell: TrustCell<_> = TrustCell::new(5);

        let a = cell.borrow();
        let b = cell.borrow();

        assert_eq!(10, *a + *b);
    }

    #[test]
    fn write() {
        let cell: TrustCell<_> = TrustCell::new(5);

        {
            let mut a = cell.borrow_mut();
            *a += 2;
            *a += 3;
        }

        assert_eq!(10, *cell.borrow());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "Already borrowed mutably")]
    fn panic_already() {
        let cell: TrustCell<_> = TrustCell::new(5);

        let mut a = cell.borrow_mut();
        *a = 7;

        assert_eq!(7, *cell.borrow());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "Already borrowed")]
    fn panic_already_write() {
        let cell: TrustCell<_> = TrustCell::new(5);

        let mut a = cell.borrow_mut();
        *a = 7;

        assert_eq!(7, *cell.borrow_mut());
    }
}
