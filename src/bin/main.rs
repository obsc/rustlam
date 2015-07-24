extern crate lam;

mod repl;

use std::env;
use repl::repl;

use lam::lexer::Lexer;
use lam::scanner::FileScanner;

fn main() {
    let arg = env::args().nth(1);
    match arg {
        Some(f) => {
          FileScanner::new(&*f).map_or_else(|| {
            println!("Unable to read file.")
          }, |f| {
            let lexer = Lexer::new(f);
            for tok in lexer {
              println!("{:?}", tok);
            }
          })
        }
        None    => repl(),
    };
}
