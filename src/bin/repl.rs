use std::io;
use std::io::Write;

use lam::scanner::StdScanner;

fn write(out: &str) {
    print!("{}", out);
    if let Err(why) = io::stdout().flush() {
        println!("Failed to flush stdout: {}", why)
    }
}

pub fn repl() {
    let mut scanner = StdScanner::new();

    loop {
        write("> ");

        match scanner.read_line() {
            None       => {
                println!("\nExiting...");
                break;
            }
            Some(line) => {
                write(line);
            }
        }
    }
}
