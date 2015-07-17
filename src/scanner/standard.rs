use std::io;
use std::collections::VecDeque;

pub struct StdScanner {
    std: io::Stdin,
    buf: VecDeque<String>,
}

impl StdScanner {
    pub fn new() -> Self {
        StdScanner{
            std: io::stdin(),
            buf: VecDeque::new(),
        }
    }

    pub fn next_line(&mut self) -> Option<&String> {
        let mut line = String::new();

        match self.std.read_line(&mut line) {
            Ok(bytes) if bytes > 0 => {
                self.buf.push_back(line);
                self.buf.back()
            },
            _ => None,
        }
    }
}

impl Iterator for StdScanner {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.buf.pop_front()
    }
}
