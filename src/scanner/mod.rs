use std::io;
use std::collections::VecDeque;

pub type Character = char;

pub struct StdScanner {
    std: io::Stdin,
    buf: VecDeque<String>,
    ind: usize,
}

impl StdScanner {
    pub fn new() -> Self {
        StdScanner{
            std: io::stdin(),
            buf: VecDeque::new(),
            ind: 0,
        }
    }

    pub fn read_line(&mut self) -> Option<&String> {
        let mut line = String::new();

        let bytes_read = match self.std.read_line(&mut line) {
            Err(_)  => 0,
            Ok(bytes) => bytes,
        };

        match bytes_read {
            0 => None,
            _ => {
                self.buf.push_back(line);
                self.buf.back()
            }
        }
    }
}

impl Iterator for StdScanner {
    type Item = Character;

    fn next(&mut self) -> Option<Character> {
        None
    }
}