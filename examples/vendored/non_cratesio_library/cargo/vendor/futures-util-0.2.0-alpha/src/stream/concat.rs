use core::mem;
use core::fmt::{Debug, Formatter, Result as FmtResult};
use core::default::Default;

use futures_core::{Async, Future, Poll, Stream};
use futures_core::task;

/// A stream combinator to concatenate the results of a stream into the first
/// yielded item.
///
/// This structure is produced by the `Stream::concat` method.
#[must_use = "streams do nothing unless polled"]
pub struct Concat<S>
    where S: Stream,
{
    inner: ConcatSafe<S>
}

impl<S: Debug> Debug for Concat<S> where S: Stream, S::Item: Debug {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Concat")
            .field("inner", &self.inner)
            .finish()
    }
}

pub fn new<S>(s: S) -> Concat<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator + Default,
{
    Concat {
        inner: new_safe(s)
    }
}

impl<S> Future for Concat<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator + Default,

{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self, cx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        self.inner.poll(cx).map(|a| {
            match a {
                Async::Pending => Async::Pending,
                Async::Ready(None) => Async::Ready(Default::default()),
                Async::Ready(Some(e)) => Async::Ready(e)
            }
        })
    }
}


#[derive(Debug)]
struct ConcatSafe<S>
    where S: Stream,
{
    stream: S,
    extend: Inner<S::Item>,
}

fn new_safe<S>(s: S) -> ConcatSafe<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator,
{
    ConcatSafe {
        stream: s,
        extend: Inner::First,
    }
}

impl<S> Future for ConcatSafe<S>
    where S: Stream,
          S::Item: Extend<<<S as Stream>::Item as IntoIterator>::Item> + IntoIterator,

{
    type Item = Option<S::Item>;
    type Error = S::Error;

    fn poll(&mut self, cx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.stream.poll_next(cx) {
                Ok(Async::Ready(Some(i))) => {
                    match self.extend {
                        Inner::First => {
                            self.extend = Inner::Extending(i);
                        },
                        Inner::Extending(ref mut e) => {
                            e.extend(i);
                        },
                        Inner::Done => unreachable!(),
                    }
                },
                Ok(Async::Ready(None)) => {
                    match mem::replace(&mut self.extend, Inner::Done) {
                        Inner::First => return Ok(Async::Ready(None)),
                        Inner::Extending(e) => return Ok(Async::Ready(Some(e))),
                        Inner::Done => panic!("cannot poll Concat again")
                    }
                },
                Ok(Async::Pending) => return Ok(Async::Pending),
                Err(e) => {
                    self.extend = Inner::Done;
                    return Err(e)
                }
            }
        }
    }
}


#[derive(Debug)]
enum Inner<E> {
    First,
    Extending(E),
    Done,
}
