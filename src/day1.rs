use itertools::Itertools;

fn get_totals(numbers: Vec<i32>) {
    let total = numbers
        .iter()
        .tuple_windows()
        .filter(|(x, y)| y > x)
        .count();
    dbg!(total);
}

pub fn day1_part1(input: Vec<String>) {
    let numbers = input
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    get_totals(numbers);
}

pub fn day1_part2(input: Vec<String>) {
    let numbers = input.into_iter().map(|s| s.parse::<i32>().unwrap());
    let windows: Vec<i32> = numbers
        .tuple_windows::<(_, _, _)>()
        .map(|(x, y, z)| x + y + z)
        .collect();
    get_totals(windows);
}
