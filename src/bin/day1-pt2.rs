use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::option;

fn parse_int(b : String) -> i32 {
    b.parse().unwrap()
}

fn solve() -> option::Option<i32> {
    let mut sum_so_far = 0;
    let mut sums = HashSet::new();

    sums.insert(sum_so_far);
    loop {
        let f = File::open("./inputs/day1/part1.txt").unwrap();
        let reader = BufReader::new(f);

        for b in reader.lines() {
            sum_so_far += parse_int(b.unwrap());

            if sums.contains(&sum_so_far) {
                return Some(sum_so_far)
            }
            sums.insert(sum_so_far);
        }
    }
}

fn main() -> io::Result<()> {
    println!("{:?}\n", solve());

    Ok(())
}
