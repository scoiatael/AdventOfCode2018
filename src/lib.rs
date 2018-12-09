use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::option;

pub struct Lines {
    iter: io::Lines<BufReader<File>>
}

impl Lines {
    pub fn new(filename: &str) -> io::Result<Lines> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let lines = reader.lines();
        Ok(Lines { iter: lines })
    }
}

impl Iterator for Lines {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        self.iter.next()
    }
}
