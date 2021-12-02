use std::fs::File;
use std::io::prelude::*;
use std::io::{Result, Lines, BufReader};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    match File::open(&filename) {
        Err(why) => return Err(why),
        Ok(file) => Ok(BufReader::new(file).lines()),
    }
}