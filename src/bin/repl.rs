use std::io;
use std::io::Write;

fn write(out: &str) {
    print!("{}", out);
    match io::stdout().flush() {
        Ok(u)  => u,
        Err(e) => println!("Failed to flush stdout: {}", e),
    };
}

pub fn repl() {
    let mut input = String::new();

    loop {
        write("> ");

        input.clear();
        let bytes_read = match io::stdin().read_line(&mut input) {
            Ok(b)  => b,
            Err(e) => { println!("Failed to read stdin: {}", e); 0 },
        };

        if bytes_read == 0 {
            println!("\nExiting...");
            break;
        }

        write(&input);
    }
}