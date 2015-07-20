pub mod standard;
pub mod file;

use super::Location;

pub use self::standard::StdScanner;
pub use self::file::FileScanner;

pub struct Scanner<I> where I: Iterator<Item = String> {
    iter: I,
    loc: Location,
    text: Vec<String>,
    buf: Vec<char>,
}

#[derive(Debug, Copy, Clone)]
pub struct Character {
    data: char,
    loc: Location,
}

impl Character {
    pub fn get_char(&self) -> char { self.data }
    pub fn get_loc(&self) -> Location { self.loc }
}

impl<I> Scanner<I> where I: Iterator<Item = String> {
    pub fn new(inner: I) -> Self {
        Scanner{
            iter: inner,
            loc: Location::new(),
            text: Vec::new(),
            buf: Vec::new(),
        }
    }

    pub fn get_ref(&self) -> &I {
        &self.iter
    }

    pub fn get_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<I> Iterator for Scanner<I> where I: Iterator<Item = String> {
    type Item = Character;

    fn next(&mut self) -> Option<Character> {
        let c = match self.buf.get(self.loc.get_col()) {
            Some(&c) => Some(c),
            None     => {
                self.iter.next().map(|line| {
                    self.buf = line.trim_right().chars().collect();
                    self.buf.push('\n');
                    self.text.push(line);
                    self.buf[self.loc.get_col()]
                })
            },
        };

        c.map(|c| {
            let out = Character{
                data: c,
                loc: self.loc,
            };

            if c == '\n' {
                self.buf.clear();
                self.loc.next_row();
            } else {
                self.loc.next_col();
            }

            out
        })
    }
}
