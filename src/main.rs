use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;
use day1::day1_part2;

static INPUT: &str = "src/day1_input.txt";

fn main() {
    let reader = BufReader::new(File::open(INPUT).unwrap());
    let lines: Vec<_> = reader.lines().filter_map(Result::ok).collect();

    // day1_part1(lines);
    day1_part2(lines);
}
