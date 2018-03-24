extern crate shred;
#[macro_use]
extern crate shred_derive;

use shred::{Dispatcher, DispatcherBuilder, Fetch, FetchMut, Resources, RunningTime, System};

fn sleep_short() {
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::new(0, 1_000));
}

struct Res;
struct ResB;

#[derive(SystemData)]
struct DummyData<'a> {
    _res: Fetch<'a, Res>,
}

#[derive(SystemData)]
struct DummyDataMut<'a> {
    _res: FetchMut<'a, Res>,
}

struct DummySys;

impl<'a> System<'a> for DummySys {
    type SystemData = DummyData<'a>;

    fn run(&mut self, _data: Self::SystemData) {
        sleep_short()
    }
}

struct DummySysMut;

impl<'a> System<'a> for DummySysMut {
    type SystemData = DummyDataMut<'a>;

    fn run(&mut self, _data: Self::SystemData) {
        sleep_short()
    }
}

struct Whatever<'a>(&'a i32);

impl<'a, 'b> System<'a> for Whatever<'b> {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        println!("{}", self.0);
    }
}

#[test]
fn dispatch_builder() {
    DispatcherBuilder::new()
        .add(DummySys, "a", &[])
        .add(DummySys, "b", &["a"])
        .add(DummySys, "c", &["a"])
        .build();
}

#[test]
#[should_panic(expected = "No such system registered")]
fn dispatch_builder_invalid() {
    DispatcherBuilder::new()
        .add(DummySys, "a", &[])
        .add(DummySys, "b", &["z"])
        .build();
}

#[test]
fn dispatch_basic() {
    let mut res = Resources::new();
    res.add(Res);

    let number = 5;

    let mut d: Dispatcher = DispatcherBuilder::new()
        .add(DummySys, "a", &[])
        .add(DummySys, "b", &["a"])
        .add(Whatever(&number), "w", &[])
        .build();

    d.dispatch(&mut res);
}

#[test]
fn dispatch_ww_block() {
    let mut res = Resources::new();
    res.add(Res);

    let mut d: Dispatcher = DispatcherBuilder::new()
        .add(DummySysMut, "a", &[])
        .add(DummySysMut, "b", &[])
        .build();

    d.dispatch(&mut res);
}

#[test]
fn dispatch_rw_block() {
    let mut res = Resources::new();
    res.add(Res);

    let mut d: Dispatcher = DispatcherBuilder::new()
        .add(DummySys, "a", &[])
        .add(DummySysMut, "b", &[])
        .build();

    d.dispatch(&mut res);
}

#[test]
fn dispatch_rw_block_rev() {
    let mut res = Resources::new();
    res.add(Res);

    let mut d: Dispatcher = DispatcherBuilder::new()
        .add(DummySysMut, "a", &[])
        .add(DummySys, "b", &[])
        .build();

    d.dispatch(&mut res);
}

#[test]
fn dispatch_sequential() {
    let mut res = Resources::new();
    res.add(Res);

    let mut d: Dispatcher = DispatcherBuilder::new()
        .add(DummySysMut, "a", &[])
        .add(DummySys, "b", &[])
        .build();

    d.dispatch_seq(&mut res);
}

#[cfg(feature = "parallel")]
#[test]
fn dispatch_async() {
    let mut res = Resources::new();
    res.add(Res);

    let mut d = DispatcherBuilder::new()
        .add(DummySysMut, "a", &[])
        .add(DummySys, "b", &[])
        .build_async(res);

    d.dispatch();

    d.wait();
}

#[cfg(feature = "parallel")]
#[test]
fn dispatch_async_res() {
    let mut res = Resources::new();
    res.add(Res);

    let mut d = DispatcherBuilder::new()
        .add(DummySysMut, "a", &[])
        .add(DummySys, "b", &[])
        .build_async(res);

    d.dispatch();

    let res = d.mut_res();
    res.add_with_id(Res, 2);
}

#[test]
fn dispatch_stage_group() {
    let mut res = Resources::new();
    res.add(Res);
    res.add(ResB);

    struct ReadingFromResB;

    impl<'a> System<'a> for ReadingFromResB {
        type SystemData = Fetch<'a, ResB>;

        fn run(&mut self, _: Self::SystemData) {
            sleep_short()
        }

        fn running_time(&self) -> RunningTime {
            RunningTime::Short
        }
    }

    struct WritingToResB;

    impl<'a> System<'a> for WritingToResB {
        type SystemData = FetchMut<'a, ResB>;

        fn run(&mut self, _: Self::SystemData) {
            sleep_short()
        }

        fn running_time(&self) -> RunningTime {
            RunningTime::VeryShort
        }
    }

    let mut d: Dispatcher = DispatcherBuilder::new()
        .add(DummySys, "read_a", &[])
        .add(ReadingFromResB, "read_b", &[])
        .add(WritingToResB, "write_b", &[])
        .build();

    d.dispatch(&mut res);
}
