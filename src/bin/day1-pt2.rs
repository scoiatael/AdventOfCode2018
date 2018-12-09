use std::io;
use std::collections::HashSet;
use std::option;

extern crate AdventOfCode2018;
use AdventOfCode2018::Lines;

fn parse_int(b : String) -> i32 {
    b.parse().unwrap()
}

fn solve() -> option::Option<i32> {
    let mut sum_so_far = 0;
    let mut sums = HashSet::new();

    sums.insert(sum_so_far);
    loop {
        for b in Lines::new("./inputs/day1/part1.txt").unwrap() {
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
