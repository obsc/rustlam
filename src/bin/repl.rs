use std::io;
use std::io::Write;

use lam::lexer::Lexer;
use lam::scanner::StdScanner;

fn write(out: &str) {
    print!("{}", out);
    if let Err(why) = io::stdout().flush() {
        println!("Failed to flush stdout: {}", why)
    }
}

pub fn repl() {
    let mut lexer = Lexer::new(StdScanner::new());

    loop {
        write("> ");

        match lexer.get_mut().next_line() {
            None       => {
                println!("\nExiting...");
                break;
            }
            Some(line) => {
                write(&line);
            }
        }

        for tok in &mut lexer {
            println!("{:?}", tok);
        }
    }
}
