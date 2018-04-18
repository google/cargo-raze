Pulse
=====
[![Build Status](https://travis-ci.org/slide-rs/pulse.svg?branch=master)](https://travis-ci.org/slide-rs/pulse)
[![Pulse](http://meritbadge.herokuapp.com/pulse)](https://crates.io/crates/pulse)

Imagine you are building a fancy high performance channel for sending data between two threads. At some point, your are going to need to figure out a way to wait on the queue for new data to become available. Spinning on a `try_recv` sucks, and some people like their phones to have more than 30 minutes of battery. We need to implement `recv`.

There are a few ways to do this, you could use a condition variable + mutex. So you lock your channel, check to see if there is data, then wait on the condition variable if there is nothing to read. If you wanted to do this without locking, you could use a Semaphore. Just do a `acquire()` on the semaphore, and if it ever returns you know there is data waiting to be read. As a bonus, it lets multiple threads wait on the same channel. Which is neat.

So you are all happy, your benchmarks are good. Then someone, posts a issue asking if there is a way to wait on two channels. Hmm, this is not a trivial problem. A semaphore does not offer an api that would let you `acquire()` on more than one Semaphore at a time. Condition variables don’t either :(

Pulse to the rescue! Pulse’s solution to the problem is a one-shot Signal. A Signal represents whether something has occurred or not. It has an extremely simple state transition diagram. It can only be Pending, Ready, or in an Dropped, and there is no way to unset a signal as it state is sticky.

![state](https://raw.githubusercontent.com/csherratt/pulse/master/.images/states.png)

It looks like this in practice:

```rust
// Create a new signal and pulse, the pulse is the setting side of a signal
let (signal, pulse) = Signal::new();

thread::spawn(move || {
    // Do awesome things here, like uploading a cat picture to internet

    // Trigger a pulse now that we are done
    pulse.pulse();
});

// Wait for the pulse! :D
signal.wait().unwrap();
```

You must be asking now, "Ok, but how does this help me with multiple channels? You got some sort of silly flag variable that you can wait on. So now instead of waiting on a Semaphore, I am waiting on your stupid Signal."

Well, You can also Select over multiple signals:

```rust
let mut select = SelectMap::new();
let (signal, pulse) = Signal::new();
let join = thread::spawn(move || {
    // Do something slow
    pulse.pulse();
});
select.add(signal, join);

let (signal, pulse) = Signal::new();
let join = thread::spawn(move || {
    // Do something else slow
    pulse.pulse();
});
select.add(signal, join);

for (_, join) in select {
    join.join(); // \o/
}
```

You ain't seen nothing yet. `Select` and `SelectMap` both need to wait, so how would they block... they need some sort of blocking primitive to Signal on... Do you, see where I am going with this?

This is the true beauty of Pulse’s design. The blocking primitives are all composable. Select is just waiting on one or more Signals. It will assert its pulse once any of the signals are ready. This is much like an Logical OR gate. And Like a logical OR gate, they can be chained*.

![or](https://raw.githubusercontent.com/csherratt/pulse/master/.images/or_gate.png)

What if you want to wait for all the signals to assert before you continue? You could, just do something like this:

```rust
for signal in signals {
    signal.wait();
}
```

But now you are going to wake up every time a signal is ready. That could be quite a few contexts switches. We need an AND gate or a Barrier of some sort.

```rust
// A barrier of all the signals, waits until they are all ready
Barrier::new(&signals).wait();
```

![and](https://raw.githubusercontent.com/csherratt/pulse/master/.images/and_gate.png)

Of course a Barrier is just a Signal too... you can wire it into a Select, or another barrier.. It’s _all_ composable.

It gets better:
---------------

These large signal chains can be used to easily build your own fancy dancy work scheduler. Since a Signal can represent a wait point, You can map Signal to continuation. When the Signal asserts as ready, the continuation is ready to run. There are every provision to change the meaning of a `wait`, which makes Pulse useful for writing a custom fiber/green threads/coroutine implementation. :D

There must be some ugly part to all of this?
--------------------------------------------

And there is, Pulse does not magically make your hyper faster channel work in all cases. It is still a hard problem to know when things need to be woken up. If you install a pulse, and another thread enqueues at the same time you have to make sure you have not left yourself in a situation where no thread will trigger the pulse. 

My recommended strategy is to double check conditions, once before you install the Pulse (the fast path) and once after. The thread that installed the pulse can uninstall its own pulse and trigger it. But there be dragons.

You can use a library like [atom](https://github.com/csherratt/atom) to store a Pulse inside of a data structure without a lock.

Why is it called Pulse?
-----------------------

 1. Because someone already claimed `Event` on crates.io
 2. It's common way to set a flip-flop in hardware. You send a signal in the form a pulse, the flip-flop gets set and assets a high signal.
That signal can go to an AND gate, or an OR gate... ect ect.

Is it 1.0.0 Compatible?
------------------------

Yes. By default it uses only stable api's. There is experimental support for callbacks that is not compatible with rust 1.0.0, but that is hidden behind a feature flag.
