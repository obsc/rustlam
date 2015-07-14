pub mod standard;

pub use self::standard::StdScanner;

pub trait Scan: Iterator<Item = char> {}

pub struct Scanner<S: Scan> {
    scanner: S,
    src: u32,
    line: u32,
    col: u32,
}

#[derive(Debug)]
pub struct Character {
    cargo: char,
    src: u32,
    line: u32,
    col: u32,
}

impl<S: Scan> Scanner<S> {
    pub fn new(inner: S) -> Self {
        Scanner{
            scanner: inner,
            src: 0,
            line: 0,
            col: 0,
        }
    }

    pub fn get_ref(&self) -> &S {
        &self.scanner
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.scanner
    }
}

impl<S: Scan> Iterator for Scanner<S> {
    type Item = Character;

    fn next(&mut self) -> Option<Character> {
        self.scanner.next().map(|c| {
            let out = Character{
                cargo: c,
                src: self.src,
                line: self.line,
                col: self.col,
            };

            self.src += 1;
            if c == '\n' {
                self.col = 0;
                self.line += 1;
            } else {
                self.col += 1;
            }

            out
        })
    }
}