extern crate rayon;
#[macro_use(par, seq)]
extern crate shred;

use rayon::ThreadPool;

use shred::{ParSeq, Resources, System};

macro_rules! impl_sys {
    ($( $id:ident )*) => {
        $(
            impl<'a> ::shred::System<'a> for $id {
                type SystemData = ();
                fn run(&mut self, _: Self::SystemData) {
                    println!(stringify!($id));
                }
            }
        )*
    };
}

struct SysA;
struct SysB;
struct SysC;
struct SysD;
struct SysWithLifetime<'a>(&'a u8);
struct SysLocal(*const u8);

impl_sys!(SysA SysB SysC SysD SysLocal);

impl<'a, 'b> System<'a> for SysWithLifetime<'b> {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        println!("SysWithLifetime");
    }
}

fn main() {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    let pool = ThreadPool::new(Default::default()).unwrap();

    let res = Resources::new();
    let x = 5u8;

    let mut dispatcher = ParSeq::new(
        seq![
            par![
                SysA,
                SysWithLifetime(&x),
                seq![
                    SysC,
                    SysD,
                ],
            ],
            SysB,
            SysLocal(&x as *const u8),
        ],
        &pool,
    );

    dispatcher.dispatch(&res);
}
