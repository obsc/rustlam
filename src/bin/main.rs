extern crate lam;

mod repl;

use std::env;
use repl::repl;

use lam::scanner::{Scanner, FileScanner};

fn main() {
    let arg = env::args().nth(1);
    match arg {
        Some(f) => {
          FileScanner::new(&*f).map_or_else(|| {
            println!("Unable to read file.")
          }, |f| {
            let scanner = Scanner::new(f);
            for c in scanner {
              println!("{:?}", c);
            }
          })
        }
        None    => repl(),
    };
}
