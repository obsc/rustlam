use std::io;
use std::io::Write;

use lam::scanner::{Scanner, StdScanner};

fn write(out: &str) {
    print!("{}", out);
    if let Err(why) = io::stdout().flush() {
        println!("Failed to flush stdout: {}", why)
    }
}

pub fn repl() {
    let mut scanner = Scanner::new(StdScanner::new());

    loop {
        write("> ");

        match scanner.get_mut().read_line() {
            None       => {
                println!("\nExiting...");
                break;
            }
            Some(line) => {
                write(&line);
            }
        }

        for c in &mut scanner {
            println!("{:?}", c);
        }
    }
}
