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


#![feature(test)]

extern crate test;
extern crate pulse;

use std::sync::mpsc::channel;
use std::sync::Mutex;
use test::Bencher;
use pulse::*;

#[bench]
fn pulse_already_set(b: &mut Bencher) {
    let (p, t)  = Signal::new();
    t.pulse();

    b.iter(|| {
        p.is_pending();
    });
}

#[bench]
fn pulse_create_and_set(b: &mut Bencher) {
    b.iter(|| {
        let (p, t)  = Signal::new();
        t.pulse();
        p.wait().unwrap();
    });
}

/*#[bench]
fn pulse_set(b: &mut Bencher) {
    let (mut p, _) = Signal::new();

    b.iter(|| {
        let t = p.recycle().unwrap();
        t.pulse();
        p.wait().unwrap();
    });
}*/

#[bench]
fn mutex_lock_time(b: &mut Bencher) {
    let mutex = Mutex::new(7);
    b.iter(|| {
        drop(mutex.lock().unwrap());
    });
}

#[bench]
fn oneshot_channel(b: &mut Bencher) {
    b.iter(|| {
        let (tx, rx) = channel();
        tx.send(()).unwrap();
        rx.recv().unwrap();
    });
}