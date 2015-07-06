use std::io;
use std::io::Write;

fn write(out: &str) {
    print!("{}", out);
    match io::stdout().flush() {
        Err(why) => println!("Failed to flush stdout: {}", why),
        Ok(unit) => unit,
    };
}

pub fn repl() {
    let mut input = String::new();

    loop {
        write("> ");

        input.clear();
        let bytes_read = match io::stdin().read_line(&mut input) {
            Err(why)  => { println!("Failed to read stdin: {}", why); 0 },
            Ok(bytes) => bytes,
        };

        if bytes_read == 0 {
            println!("\nExiting...");
            break;
        }

        write(&input);
    }
}