use std::io;
use std::io::Write;

fn write(out: &str) {
    print!("{}", out);
    io::stdout().flush().unwrap_or(());
}

pub fn repl() {
    loop {
        let mut input = String::new();
        write("> ");

        let read_bytes = io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line!");

        if read_bytes == 0 {
            println!("\nExiting...");
            break;
        }

        write(&input);
    }
}