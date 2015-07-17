use std::{io, fs, path};

use std::io::BufRead;

pub struct FileScanner {
    file: io::BufReader<fs::File>,
}

impl FileScanner {
    pub fn new(filename: &str) -> Option<Self> {
        let path = path::Path::new(filename);

        fs::File::open(path).ok().map(|f| {
            FileScanner{
                file: io::BufReader::new(f),
            }
        })
    }
}

impl Iterator for FileScanner {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut line = String::new();

        match self.file.read_line(&mut line) {
            Ok(bytes) if bytes > 0 => Some(line),
            _                      => None,
        }
    }
}
