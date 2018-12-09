extern crate AdventOfCode2018;
use AdventOfCode2018::Lines;

fn parse_int(b : String) -> i32 {
    b.parse().unwrap()
}

fn main() {
    let f = Lines::new("./inputs/day1/part1.txt").unwrap();

    let sum = f.fold(0, |a, b| a + parse_int(b.unwrap()));

    println!("{}\n", sum);
}
