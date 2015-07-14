use std::io;
use std::collections::VecDeque;

use super::Scan;

pub struct StdScanner {
    std: io::Stdin,
    buf: VecDeque<Vec<char>>,
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

    pub fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();

        match self.std.read_line(&mut line) {
            Ok(bytes) if bytes > 0 => {
                self.buf.push_back(line.chars().collect());
                Some(line)
            }
            _ => {
                None
            }
        }
    }
}

impl Scan for StdScanner {}

impl Iterator for StdScanner {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let cl = self.buf.front()
            .map(|line| (line[self.ind], line.len()));

        cl.map(|(c, l)| {
            if self.ind + 1 == l {
                self.ind = 0;
                self.buf.pop_front();
            } else {
                self.ind += 1;
            }
            c
        })
    }
}