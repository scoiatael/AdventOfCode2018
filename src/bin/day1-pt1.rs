use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn parse_int(b : String) -> i32 {
    b.parse().unwrap()
}

fn main() -> io::Result<()> {
    let f = File::open("./inputs/day1/part1.txt")?;
    let reader = BufReader::new(f);

    let sum = reader.lines().fold(0, |a, b| a + parse_int(b.unwrap()));

    println!("{}\n", sum);

    Ok(())
}
