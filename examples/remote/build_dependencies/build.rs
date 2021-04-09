use std::io::{stdout, BufWriter};

fn main() {
  let out = b"Hello fellow Rustaceans!";
  let width = 24;

  let mut writer = BufWriter::new(stdout());
  ferris_says::say(out, width, &mut writer).unwrap();
}
