use std::fs::File;
use std::io::{BufRead, BufReader, Read};

mod day4;
use day4::part1;
use day4::part2;

static INPUT: &str = "src/day4_input.txt";

fn main() {
    let mut reader = BufReader::new(File::open(INPUT).unwrap());
    // let lines: Vec<_> = reader.lines().filter_map(Result::ok).collect();
    let mut data = String::new();
    reader.read_to_string(&mut data).unwrap();

    part1(data.clone());
    part2(data.clone());
}
