mopa 0.2.2
==========

[![Build Status](https://travis-ci.org/chris-morgan/mopa.svg?branch=master)](https://travis-ci.org/chris-morgan/mopa)

<!-- The rest of this section comes straight from the crate docs from the source. -->

**MOPA: My Own Personal Any.** A macro to implement all the `Any` methods on your own trait.

You like `Any`—its ability to store any `'static` type as a trait object and then downcast it
back to the original type is very convenient, and in fact you need it for whatever misguided
reason. But it’s not enough. What you *really* want is your own trait object type with `Any`’s
functionality glued onto it. Maybe you have a `Person` trait and you want your people to be
able to do various things, but you also want to be able to conveniently downcast the person to
its original type, right? Alas, you can’t write a type like `Box<Person + Any>` (at present,
anyway). So what do you do instead? Do you give up? No, no! No, no! Enter MOPA.

> There once was a quite friendly trait  
> Called `Person`, with much on its plate.  
>     “I need to be `Any`  
>     To downcast to `Benny`—  
> But I’m not, so I guess I’ll just wait.”

A pitiful tale, isn’t it? Especially given that there was a bear chasing it with intent to eat
it. Fortunately now you can *mopafy* `Person` in three simple steps:

1. Add the `mopa` crate to your `Cargo.toml` as usual and your crate root like so:

   ```rust,ignore
   #[macro_use]
   extern crate mopa;
   ```

2. Make `Any` (`mopa::Any`, not `std::any::Any`) a supertrait of `Person`;

3. `mopafy!(Person);`.

And lo, you can now write `person.is::<Benny>()` and `person.downcast_ref::<Benny>()` and so on
to your heart’s content. Simple, huh?

Oh, by the way, it was actually the person on the bear’s plate. There wasn’t really anything on
`Person`’s plate after all.

```rust
#[macro_use]
extern crate mopa;

struct Bear {
    // This might be a pretty fat bear.
    fatness: u16,
}

impl Bear {
    fn eat(&mut self, person: Box<Person>) {
        self.fatness = (self.fatness as i16 + person.weight()) as u16;
    }
}

trait Person: mopa::Any {
    fn panic(&self);
    fn yell(&self) { println!("Argh!"); }
    fn sleep(&self);
    fn weight(&self) -> i16;
}

mopafy!(Person);

struct Benny {
    // (Benny is not a superhero. He can’t carry more than 256kg of food at once.)
    kilograms_of_food: u8,
}

impl Person for Benny {
    fn panic(&self) { self.yell() }
    fn sleep(&self) { /* ... */ }
    fn weight(&self) -> i16 {
        // Who’s trying to find out? I’m scared!
        self.yell();
        self.kilograms_of_food as i16 + 60
    }
}

struct Chris;

impl Chris {
    // Normal people wouldn’t be brave enough to hit a bear but Chris might.
    fn hit(&self, bear: &mut Bear) {
        println!("Chris hits the bear! How brave! (Or maybe stupid?)");
        // Meh, boundary conditions, what use are they in examples?
        // Chris clearly hits quite hard. Poor bear.
        bear.fatness -= 1;
    }
}

impl Person for Chris {
    fn panic(&self) { /* ... */ }
    fn sleep(&self) { /* ... */ }
    fn weight(&self) -> i16 { -5 /* antigravity device! cool! */ }
}

fn simulate_simulation(person: Box<Person>, bear: &mut Bear) {
    if person.is::<Benny>() {
        // None of the others do, but Benny knows this particular
        // bear by reputation and he’s *really* going to be worried.
        person.yell()
    }
    // If it happens to be Chris, he’ll hit the bear.
    person.downcast_ref::<Chris>().map(|chris| chris.hit(bear));
    bear.eat(person);
}

fn main() {
    let mut bear = Bear { fatness: 10 };
    simulate_simulation(Box::new(Benny { kilograms_of_food: 5 }), &mut bear);
    simulate_simulation(Box::new(Chris), &mut bear);
}
```

Now *should* you do something like this? Probably not. Enums are probably a better solution for
this particular case as written; frankly I believe that almost the only time you should
downcast an Any trait object (or a mopafied trait object) is with a generic parameter, when
producing something like `AnyMap`, for example. If you control *all* the code, `Any` trait
objects are probably not the right solution; they’re good for cases with user-defined
types across a variety of libraries. But the question of purpose and suitability is open, and I
don’t have a really good example of such a use case here at present. TODO.

Usage
-----

Cargo all the way. http://crates.io/crates/mopa

Author
------

[Chris Morgan](http://chrismorgan.info/) ([chris-morgan](https://github.com/chris-morgan)) is the primary author and maintainer of this library.

License
-------

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
