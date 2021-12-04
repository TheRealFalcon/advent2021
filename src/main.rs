use std::fs::File;
use std::io::{BufRead, BufReader};

mod day3;
use day3::day3_part1;
use day3::day3_part2;

#[macro_use]
extern crate itertools;

static INPUT: &str = "src/day3_input.txt";

fn main() {
    let reader = BufReader::new(File::open(INPUT).unwrap());
    let lines: Vec<_> = reader.lines().filter_map(Result::ok).collect();

    day3_part1(&lines);
    day3_part2(&lines);
}
