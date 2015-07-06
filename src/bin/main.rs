mod repl;

use std::env;
use repl::repl;

fn main() {
    let mut args = env::args().skip(1);
    match args.next() {
        Some(f) => println!("{}", f),
        None    => repl(),
    };
}