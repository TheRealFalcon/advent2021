use std::fs::File;
use std::io::{BufRead, BufReader};

mod day2;
use day2::day2_part2;

static INPUT: &str = "src/day2_input.txt";

fn main() {
    let reader = BufReader::new(File::open(INPUT).unwrap());
    let lines: Vec<_> = reader.lines().filter_map(Result::ok).collect();

    day2_part2(lines);
}
