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

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use {Pulse, Signal, Waiting, Signals};

pub struct Inner {
    pub count: AtomicUsize,
    pub trigger: Mutex<Option<Pulse>>,
}

/// A `Barrier` can listen for 1 or more `Signals`. It will only transition
/// to a `Pulsed` state once all the `Signals` have `Pulsed`.
pub struct Barrier {
    inner: Arc<Inner>,
}

pub struct Handle(pub Arc<Inner>);

impl Barrier {
    /// Create a new Barrier from an Vector of `Siganl`s
    pub fn new(pulses: &[Signal]) -> Barrier {
        // count items
        let inner = Arc::new(Inner {
            count: AtomicUsize::new(pulses.len()),
            trigger: Mutex::new(None),
        });

        for pulse in pulses {
            pulse.clone().arm(Waiting::barrier(Handle(inner.clone())));
        }

        Barrier { inner: inner }
    }
}

impl Signals for Barrier {
    fn signal(&self) -> Signal {
        let (p, t) = Signal::new();

        let mut guard = self.inner.trigger.lock().unwrap();
        if self.inner.count.load(Ordering::Relaxed) == 0 {
            t.pulse();
        } else {
            *guard = Some(t);
        }
        p
    }
}
