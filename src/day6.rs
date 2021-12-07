fn common(input: &str, days: u32) {
    let mut counts: [u64; 9] = [0; 9];

    let all_fish: Vec<u8> = input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    for fish in all_fish {
        counts[fish as usize] += 1;
    }

    for _day in 0..days {
        let reproducing_fish = counts[0];
        for index in 1..9 {
            counts[index - 1] = counts[index];
        }
        counts[6] += reproducing_fish;
        counts[8] = reproducing_fish;
    }
    dbg!(counts.iter().sum::<u64>());
}

pub fn part1(input: &str) {
    common(input, 80);
}

pub fn part2(input: &str) {
    common(input, 256);
}
