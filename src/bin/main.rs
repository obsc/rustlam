extern crate lam;

mod repl;

use std::env;
use repl::repl;

fn main() {
    let arg = env::args().nth(1);
    match arg {
        Some(f) => println!("{}", f),
        None    => repl(),
    };
}