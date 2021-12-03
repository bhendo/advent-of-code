use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Lines};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<Lines<BufReader<File>>> {
    match File::open(&filename) {
        Err(why) => return Err(why),
        Ok(file) => Ok(BufReader::new(file).lines()),
    }
}
