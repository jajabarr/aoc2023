#![allow(dead_code)]
#![allow(unused_labels)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::env;

mod _template;
mod _traits;
mod _util;
mod four;
mod one;
mod three;
mod two;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    solve();
}
