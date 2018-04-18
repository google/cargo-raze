// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Test that #[derive(Copy, Clone)] produces a shallow copy
//! even when a member violates RFC 1521

#[macro_use]
extern crate derivative;

use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

/// A struct that pretends to be Copy, but actually does something
/// in its Clone impl
#[derive(Copy)]
struct Liar;

/// Static cooperating with the rogue Clone impl
static CLONED: AtomicBool = ATOMIC_BOOL_INIT;

impl Clone for Liar {
    fn clone(&self) -> Self {
        // this makes Clone vs Copy observable
        CLONED.store(true, Ordering::SeqCst);

        *self
    }
}

/// This struct is actually Copy... at least, it thinks it is!
#[derive(Copy, Clone)]
struct TheirTheir(Liar);

#[derive(Derivative)]
#[derivative(Copy, Clone)]
struct OurOur1(Liar);
#[derive(Derivative)]
#[derivative(Clone, Copy)]
struct OurOur2(Liar);

#[derive(Copy)]
#[derive(Derivative)]
#[derivative(Clone)]
struct TheirOur1(Liar);
#[derive(Copy)]
#[derive(Derivative)]
#[derivative(Clone)]
struct TheirOur2(Liar);

#[test]
fn main() {
    let _ = TheirTheir(Liar).clone();
    assert!(!CLONED.load(Ordering::SeqCst), "TheirTheir");

    let _ = OurOur1(Liar).clone();
    assert!(!CLONED.load(Ordering::SeqCst), "OurOur1");
    let _ = OurOur2(Liar).clone();
    assert!(!CLONED.load(Ordering::SeqCst), "OurOur2");

    // Ideally this would work the same, just testing that the behaviour does not change:
    CLONED.store(false, Ordering::SeqCst);
    let _ = TheirOur1(Liar).clone();
    assert!(CLONED.load(Ordering::SeqCst), "TheirOur1");
    CLONED.store(false, Ordering::SeqCst);
    let _ = TheirOur2(Liar).clone();
    assert!(CLONED.load(Ordering::SeqCst), "TheirOur2");
}
