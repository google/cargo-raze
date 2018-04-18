//   Copyright 2015 Colin Sherratt
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

extern crate atom;
extern crate time;

use std::sync::atomic::AtomicUsize;
use std::thread;
use std::mem;
use std::fmt;
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::cell::RefCell;

use atom::*;
use time::precise_time_s;
use fnbox::FnBox;

pub use select::{Select, SelectMap};
pub use barrier::Barrier;
mod select;
mod barrier;
mod fnbox;

/// Drop rules
/// This may be freed iff state is Signald | Dropped
/// and Waiting is Dropped
struct Inner {
    state: AtomicUsize,
    waiting: Atom<Box<Waiting>>,
}

// TODO 64bit sized, probably does not matter now
const PULSED: usize = 0x8000_0000;
const TX_DROP: usize = 0x4000_0000;
const TX_FLAGS: usize = PULSED | TX_DROP;
const REF_COUNT: usize = !TX_FLAGS;

struct Waiting {
    next: Option<Box<Waiting>>,
    wake: Wake,
}

impl GetNextMut for Box<Waiting> {
    type NextPtr = Option<Box<Waiting>>;

    fn get_next(&mut self) -> &mut Option<Box<Waiting>> {
        &mut self.next
    }
}

enum Wake {
    Thread(thread::Thread),
    Select(select::Handle),
    Barrier(barrier::Handle),
    Callback(Box<FnBox>),
}

impl Waiting {
    fn wake(s: Box<Self>, id: usize) {
        let mut next = Some(s);
        while let Some(s) = next {
            // There must be a better way to do this...
            let s = *s;
            let Waiting { next: n, wake } = s;
            next = n;
            match wake {
                Wake::Thread(thread) => thread.unpark(),
                Wake::Select(select) => {
                    let trigger = {
                        let mut guard = select.0.lock().unwrap();
                        guard.ready.push(id);
                        guard.trigger.take()
                    };
                    trigger.map(|x| x.pulse());
                }
                Wake::Barrier(barrier) => {
                    let count = barrier.0.count.fetch_sub(1, Ordering::Relaxed);
                    if count == 1 {
                        let mut guard = barrier.0.trigger.lock().unwrap();
                        if let Some(t) = guard.take() {
                            t.pulse();
                        }
                    }
                }
                Wake::Callback(cb) => cb.call_box(),
            }
        }
    }

    fn id(&self) -> usize {
        unsafe { mem::transmute(self) }
    }

    fn thread() -> Box<Waiting> {
        Box::new(Waiting {
            next: None,
            wake: Wake::Thread(thread::current()),
        })
    }

    fn select(handle: select::Handle) -> Box<Waiting> {
        Box::new(Waiting {
            next: None,
            wake: Wake::Select(handle),
        })
    }

    fn barrier(handle: barrier::Handle) -> Box<Waiting> {
        Box::new(Waiting {
            next: None,
            wake: Wake::Barrier(handle),
        })
    }

    fn callback<F>(cb: F) -> Box<Waiting>
        where F: FnOnce() + 'static
    {
        Box::new(Waiting {
            next: None,
            wake: Wake::Callback(Box::new(cb)),
        })
    }
}

unsafe impl Send for Pulse {}
// This should be safe a pulse requires ownership to
// actually `pulse`
unsafe impl Sync for Pulse {}

/// A `Pulse` is represents an unfired signal. It is the tx side of Signal
/// A `Pulse` can only purpose it to be fired, and then it will be moved
/// as to never allow it to fire again. `Dropping` a pulse will `pulse`
/// The signal, but the signal will enter an error state.
pub struct Pulse {
    inner: *mut Inner,
}

impl fmt::Debug for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let id: usize = unsafe { mem::transmute(self.inner) };
        write!(f, "Pulse({:?})", id)
    }
}

fn delete_inner(state: usize, inner: *mut Inner) {
    if state & REF_COUNT == 1 {
        let inner: Box<Inner> = unsafe { mem::transmute(inner) };
        drop(inner);
    }
}

impl Drop for Pulse {
    fn drop(&mut self) {
        self.set(TX_DROP);
        self.wake();
        let state = self.inner().state.fetch_sub(1, Ordering::Relaxed);
        delete_inner(state, self.inner)
    }
}

impl Pulse {
    /// Create a Pulse from a usize. This is naturally unsafe.
    #[inline]
    pub unsafe fn cast_from_usize(ptr: usize) -> Pulse {
        Pulse { inner: mem::transmute(ptr) }
    }

    /// Convert a trigger to a `usize`, This is unsafe
    /// and it will kill your kittens if you are not careful
    #[inline]
    pub unsafe fn cast_to_usize(self) -> usize {
        let us = mem::transmute(self.inner);
        mem::forget(self);
        us
    }

    #[inline]
    fn inner(&self) -> &Inner {
        unsafe { mem::transmute(self.inner) }
    }

    #[inline]
    fn set(&self, state: usize) -> usize {
        self.inner().state.fetch_or(state, Ordering::Relaxed)
    }

    #[inline]
    fn wake(&self) {
        let id = unsafe { mem::transmute(self.inner) };
        match self.inner().waiting.take() {
            None => (),
            Some(v) => Waiting::wake(v, id),
        }
    }

    /// Pulse the `pulse` which will transition the `Signal` out from pending
    /// to ready. This moves the pulse so that it can only be fired once.
    #[inline]
    pub fn pulse(self) {
        self.set(PULSED);
        self.wake();

        let state = self.inner().state.fetch_sub(1, Ordering::Relaxed);
        delete_inner(state, self.inner);
        mem::forget(self)
    }
}


unsafe impl Send for Signal {}
// This should be safe a signal requires ownership to do anything
// the inner is all atomically modified data anyhow
unsafe impl Sync for Signal {}

/// A `Signal` represents listens for a `pulse` to occur in the system. A
/// `Signal` has one of three states. Pending, Pulsed, or Errored. Pending
/// means the pulse has not fired, but still exists. Pulsed meaning the 
/// pulse has fired, and no longer exists. Errored means the pulse was dropped
/// without firing. This normally means a programming error of some sort.
pub struct Signal {
    inner: *mut Inner,
}

impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "Signal(id={:?}, pending={:?})",
               self.id(),
               self.is_pending())
    }
}

impl Clone for Signal {
    #[inline(always)]
    fn clone(&self) -> Signal {
        self.inner().state.fetch_add(1, Ordering::Relaxed);
        Signal { inner: self.inner }
    }
}

impl Drop for Signal {
    #[inline]
    fn drop(&mut self) {
        let flag = self.inner().state.fetch_sub(1, Ordering::Relaxed);
        delete_inner(flag, self.inner);
    }
}

impl Signal {
    /// Create a Signal and a Pulse that are associated.
    pub fn new() -> (Signal, Pulse) {
        let inner = Box::new(Inner {
            state: AtomicUsize::new(2),
            waiting: Atom::empty(),
        });

        let inner = unsafe { mem::transmute(inner) };

        (Signal { inner: inner }, Pulse { inner: inner })
    }

    /// Create a signal that is already pulsed
    pub fn pulsed() -> Signal {
        let inner = Box::new(Inner {
            state: AtomicUsize::new(1 | PULSED),
            waiting: Atom::empty(),
        });

        let inner = unsafe { mem::transmute(inner) };

        Signal { inner: inner }
    }

    #[inline]
    fn inner(&self) -> &Inner {
        unsafe { mem::transmute(self.inner) }
    }

    /// Read out the state of the Signal
    #[inline]
    pub fn state(&self) -> SignalState {
        let flags = self.inner().state.load(Ordering::Relaxed);
        match (flags & TX_DROP == TX_DROP, flags & PULSED == PULSED) {
            (_, true) => SignalState::Pulsed,
            (true, _) => SignalState::Dropped,
            (_, _) => SignalState::Pending,
        }
    }

    /// Check to see if the signal is pending. A signal 
    #[inline]
    pub fn is_pending(&self) -> bool {
        self.state() == SignalState::Pending
    }

    /// Add a waiter to a waitlist
    fn add_to_waitlist(&self, waiter: Box<Waiting>) -> usize {
        let id = waiter.id();

        if !self.is_pending() {
            Waiting::wake(waiter, self.id());
            return id;
        }

        self.inner().waiting.replace_and_set_next(waiter);

        // if armed fire now
        if !self.is_pending() {
            if let Some(t) = self.inner().waiting.take() {
                Waiting::wake(t, self.id());
            }
        }
        id
    }

    /// Remove Waiter with `id` from the waitlist
    fn remove_from_waitlist(&self, id: usize) {
        let mut wl = self.inner().waiting.take();
        while let Some(mut w) = wl {
            let next = w.next.take();
            if w.id() != id {
                self.add_to_waitlist(w);
            }
            wl = next;
        }
    }

    /// Arm a pulse to wake 
    fn arm(self, waiter: Box<Waiting>) -> ArmedSignal {
        let id = self.add_to_waitlist(waiter);
        ArmedSignal {
            id: id,
            pulse: self,
        }
    }

    /// This is a unique id that can be used to identify the signal from others
    /// See `Select` for how this api is useful.
    pub fn id(&self) -> usize {
        unsafe { mem::transmute_copy(&self.inner) }
    }

    /// Block the current thread until a `pulse` is ready.
    /// This will block indefinably if the pulse never fires.
    #[inline]
    pub fn wait(self) -> Result<(), WaitError> {
        match self.state() {
            SignalState::Pulsed => Ok(()),
            SignalState::Dropped => Err(WaitError::Dropped),
            SignalState::Pending => {
                let s = take_scheduler().expect("no scheduler found");
                let res = s.wait(self);
                swap_scheduler(s);
                res
            }
        }
    }

    /// Block until either the pulse is sent, or the timeout is reached
    pub fn wait_timeout_ms(self, ms: u32) -> Result<(), TimeoutError> {
        SCHED.with(|sched| {
            let s = sched.borrow_mut().take().expect("Waited while: no scheduler installed");
            let res = s.wait_timeout_ms(self, ms);
            *sched.borrow_mut() = Some(s);
            res
        })
    }

    pub fn callback<F>(self, cb: F)
        where F: FnOnce() + 'static
    {
        self.add_to_waitlist(Waiting::callback(cb));
    }
}

/// Described the possible states of a Signal
#[derive(Debug, PartialEq, Eq)]
pub enum SignalState {
    Pending,
    Pulsed,
    Dropped,
}

impl IntoRawPtr for Pulse {
    #[inline(always)]
    unsafe fn into_raw(self) -> *mut () {
        let inner = self.inner;
        mem::forget(self);
        mem::transmute(inner)
    }
}

impl FromRawPtr for Pulse {
    #[inline(always)]
    unsafe fn from_raw(ptr: *mut ()) -> Pulse {
        Pulse { inner: mem::transmute(ptr) }
    }
}

impl IntoRawPtr for Signal {
    #[inline(always)]
    unsafe fn into_raw(self) -> *mut () {
        let inner = self.inner;
        mem::forget(self);
        mem::transmute(inner)
    }
}

impl FromRawPtr for Signal {
    #[inline(always)]
    unsafe fn from_raw(ptr: *mut ()) -> Signal {
        Signal { inner: mem::transmute(ptr) }
    }
}

/// Represents the possible errors that can occur on a `Signal`
#[derive(Debug, PartialEq, Eq)]
pub enum WaitError {
    /// The `Pulse` was dropped before it could `Pulse`
    Dropped,
}

/// Represents the possible errors from a wait timeout
#[derive(Debug, PartialEq, Eq)]
pub enum TimeoutError {
    /// A `WaitError` has occurred
    Error(WaitError),
    /// The `Signal` timed-out before a `Pulse` was observed.
    Timeout,
}

struct ArmedSignal {
    pulse: Signal,
    id: usize,
}

impl Deref for ArmedSignal {
    type Target = Signal;

    fn deref(&self) -> &Signal {
        &self.pulse
    }
}

impl ArmedSignal {
    fn disarm(self) -> Signal {
        self.remove_from_waitlist(self.id);
        self.pulse
    }
}

/// allows an object to assert a wait signal
pub trait Signals {
    /// Get a signal from a object
    fn signal(&self) -> Signal;

    /// Block the current thread until the object
    /// assets a pulse.
    fn wait(&self) -> Result<(), WaitError> {
        let signal = self.signal();
        signal.wait()
    }

    /// Block the current thread until the object
    /// assets a pulse. Or until the timeout has been asserted.
    fn wait_timeout_ms(&self, ms: u32) -> Result<(), TimeoutError> {
        let signal = self.signal();
        signal.wait_timeout_ms(ms)
    }
}

/// This is the hook into the async wait methods provided
/// by `pulse`. It is required for the user to override
/// the current system scheduler.
pub trait Scheduler: std::fmt::Debug {
    /// Wait until the signal is made `ready` or `errored`
    fn wait(&self, signal: Signal) -> Result<(), WaitError>;

    /// Wait until the signal is made `ready` or `errored` or the
    /// timeout has been reached.
    fn wait_timeout_ms(&self, signal: Signal, timeout: u32) -> Result<(), TimeoutError>;
}

/// This is the `default` system scheduler that is used if no
/// user provided scheduler is installed. It is very basic
/// and will block the OS thread using `thread::park`
#[derive(Debug)]
pub struct ThreadScheduler;

impl Scheduler for ThreadScheduler {
    fn wait(&self, signal: Signal) -> Result<(), WaitError> {
        loop {
            let id = signal.add_to_waitlist(Waiting::thread());
            if signal.is_pending() {
                thread::park();
            }
            signal.remove_from_waitlist(id);

            match signal.state() {
                SignalState::Pending => (),
                SignalState::Pulsed => return Ok(()),
                SignalState::Dropped => return Err(WaitError::Dropped),
            }
        }
    }

    fn wait_timeout_ms(&self, signal: Signal, ms: u32) -> Result<(), TimeoutError> {
        let mut now = (precise_time_s() * 1000.) as u64;
        let end = now + ms as u64;

        loop {
            let id = signal.add_to_waitlist(Waiting::thread());
            if signal.is_pending() {
                now = (precise_time_s() * 1000.) as u64;
                if now > end {
                    return Err(TimeoutError::Timeout);
                }
                thread::park_timeout_ms((end - now) as u32);
            }
            signal.remove_from_waitlist(id);

            match signal.state() {
                SignalState::Pending => (),
                SignalState::Pulsed => return Ok(()),
                SignalState::Dropped => return Err(TimeoutError::Error(WaitError::Dropped)),
            }
        }
    }
}

/// The TLS scheduler
thread_local!(static SCHED: RefCell<Option<Box<Scheduler>>> = RefCell::new(Some(Box::new(ThreadScheduler))));

// this is inline never to avoid the SCHED pointer being cached
#[inline(never)]
fn take_scheduler() -> Option<Box<Scheduler>> {
    use std::mem;
    let mut sched = None;
    SCHED.with(|s| mem::swap(&mut *s.borrow_mut(), &mut sched));
    sched
}

/// Replace the current Scheduler with your own supplied scheduler.
/// all `wait()` commands will be run through this scheduler now.
///
/// This will return the current TLS scheduler, which may be useful
/// to restore it later.
#[inline(never)]
pub fn swap_scheduler(sched: Box<Scheduler>) -> Option<Box<Scheduler>> {
    use std::mem;
    let mut sched = Some(sched);
    SCHED.with(|s| mem::swap(&mut *s.borrow_mut(), &mut sched));
    sched
}

/// Call the suppled closure using the supplied schedulee
pub fn with_scheduler<F>(f: F, sched: Box<Scheduler>) -> Option<Box<Scheduler>>
    where F: FnOnce()
{
    let old = swap_scheduler(sched);
    f();
    old.and_then(|o| swap_scheduler(o))
}
