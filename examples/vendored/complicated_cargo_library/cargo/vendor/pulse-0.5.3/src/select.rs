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


use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use {Signal, ArmedSignal, Pulse, Waiting, Barrier, Signals};

pub struct Inner {
    pub ready: Vec<usize>,
    pub trigger: Option<Pulse>,
}

pub struct Handle(pub Arc<Mutex<Inner>>);

/// A `Select` listens to 1 or more signals. It will wait until
/// any signal becomes available before Pulsing. `Select` will then
/// return the `Signal` that has been `Pulsed`. `Select` has no defined
/// ordering of events for `Signal`s when there are more then one `Signals`
/// pending.
pub struct Select {
    inner: Arc<Mutex<Inner>>,
    signals: HashMap<usize, ArmedSignal>,
}

impl Select {
    /// Create a new empty `Select`
    pub fn new() -> Select {
        Select {
            inner: Arc::new(Mutex::new(Inner {
                ready: Vec::new(),
                trigger: None,
            })),
            signals: HashMap::new(),
        }
    }

    /// Add a signal to the `Select`, a unique id that is associated
    /// With the signal is returned. This can be used to remove the
    /// signal from the `Select` or to lookup the `Pulse` when it fires.
    pub fn add(&mut self, pulse: Signal) -> usize {
        let id = pulse.id();
        let p = pulse.arm(Waiting::select(Handle(self.inner.clone())));
        self.signals.insert(id, p);
        id
    }

    /// Remove a `Signal1 from the `Select` using it's unique id.
    pub fn remove(&mut self, id: usize) -> Option<Signal> {
        self.signals
            .remove(&id)
            .map(|x| x.disarm())
    }

    /// Convert all the signals present in the `Select` into a `Barrier`
    pub fn into_barrier(self) -> Barrier {
        let vec: Vec<Signal> = self.signals
                                   .into_iter()
                                   .map(|(_, p)| p.disarm())
                                   .collect();

        Barrier::new(&vec)
    }

    /// This is a non-blocking attempt to get a `Signal` from a `Select`
    /// this will return a `Some(Signal)` if there is a pending `Signal`
    /// in the select. Otherwise it will return `None`
    pub fn try_next(&mut self) -> Option<Signal> {
        let mut guard = self.inner.lock().unwrap();
        if let Some(x) = guard.ready.pop() {
            return Some(self.signals.remove(&x).map(|x| x.disarm()).unwrap());
        }
        None
    }

    /// Get the number of Signals being watched
    pub fn len(&self) -> usize {
        self.signals.len()
    }
}

impl Iterator for Select {
    type Item = Signal;

    fn next(&mut self) -> Option<Signal> {
        loop {
            if self.signals.len() == 0 {
                return None;
            }

            let pulse = {
                let mut guard = self.inner.lock().unwrap();
                while let Some(x) = guard.ready.pop() {
                    if let Some(x) = self.signals.remove(&x) {
                        return Some(x.disarm());
                    }
                }
                let (pulse, t) = Signal::new();
                guard.trigger = Some(t);
                pulse
            };
            pulse.wait().unwrap();
        }
    }
}

impl Signals for Select {
    fn signal(&self) -> Signal {
        let (pulse, t) = Signal::new();
        let mut guard = self.inner.lock().unwrap();
        if guard.ready.len() == 0 {
            guard.trigger = Some(t);
        } else {
            t.pulse();
        }
        pulse
    }
}

/// `SelectMap` is a wrapper around a `Select` rather then use
/// a unique id to find out what signal has been asserts, `SelectMap`
/// will return an supplied object.
pub struct SelectMap<T> {
    select: Select,
    items: HashMap<usize, T>,
}

impl<T> SelectMap<T> {
    /// Create a new empty `SelectMap`
    pub fn new() -> SelectMap<T> {
        SelectMap {
            select: Select::new(),
            items: HashMap::new(),
        }
    }

    /// Add a `Signal` and an associated value into the `SelectMap`
    pub fn add(&mut self, signal: Signal, value: T) {
        let id = self.select.add(signal);
        self.items.insert(id, value);
    }

    /// This is a non-blocking attempt to get a `Signal` from a `SelectMap`
    /// this will return a `Some((Signal, T))` if there is a pending `Signal`
    /// in the select. Otherwise it will return `None`
    pub fn try_next(&mut self) -> Option<(Signal, T)> {
        self.select.try_next().map(|x| {
            let id = x.id();
            (x, self.items.remove(&id).unwrap())
        })
    }

    /// Get the number of items in the `SelectMap`
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> Iterator for SelectMap<T> {
    type Item = (Signal, T);

    fn next(&mut self) -> Option<(Signal, T)> {
        self.select.next().map(|x| {
            let id = x.id();
            (x, self.items.remove(&id).unwrap())
        })
    }
}

impl<T> Signals for SelectMap<T> {
    fn signal(&self) -> Signal {
        self.select.signal()
    }
}
