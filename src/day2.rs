pub fn day2_part1(input: Vec<String>) {
    let mut horizontal_total = 0;
    let mut vertical_total = 0;
    for line in input {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let num: i32 = split.next().unwrap().parse().unwrap();
        match direction {
            "forward" => horizontal_total += num,
            "down" => vertical_total += num,
            "up" => vertical_total -= num,
            _ => panic!("Mark!"),
        }
    }
    println!("{}", horizontal_total * vertical_total);
}

pub fn day2_part2(input: Vec<String>) {
    let mut horizontal_total = 0;
    let mut vertical_total = 0;
    let mut aim = 0;
    for line in input {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let num: i32 = split.next().unwrap().parse().unwrap();
        match direction {
            "forward" => {
                horizontal_total += num;
                vertical_total += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => panic!("Mark!"),
        }
    }
    println!("{}", horizontal_total * vertical_total);
}
