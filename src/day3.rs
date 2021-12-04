// This is gross

fn get_pivoted(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut pivoted: Vec<Vec<char>> = Vec::new();
    let bit_length = input[0].len();
    for _ in 0..bit_length {
        pivoted.push(Vec::new());
    }
    for bits in input {
        for (index, char) in bits.chars().enumerate() {
            pivoted[index].push(char);
        }
    }
    pivoted
}

pub fn day3_part1(input: &Vec<String>) -> (String, String) {
    let pivoted = get_pivoted(input);

    let mut num_string: String = String::new();
    let mut opposite_string: String = String::new();
    for list in pivoted {
        let ones = list.iter().filter(|x| **x == '1').count();
        if ones as f32 >= list.len() as f32 / 2.0 {
            num_string.push('1');
            opposite_string.push('0');
        } else {
            num_string.push('0');
            opposite_string.push('1');
        }
    }
    let num = isize::from_str_radix(num_string.as_str(), 2).unwrap();
    let flipped_num = isize::from_str_radix(opposite_string.as_str(), 2).unwrap();
    dbg!(num + flipped_num);
    (num_string, opposite_string)
}

pub fn day3_part2(input: &Vec<String>) {
    println!("part 2!");
    let mut ones_vec = input.clone();
    let mut zeroes_vec = input.clone();
    let line_size = (&ones_vec)[0].len();
    for index in 0..line_size {
        let commons = day3_part1(&ones_vec);
        ones_vec.retain(|line| line.as_bytes()[index] == commons.0.as_bytes()[index]);
        if ones_vec.len() == 1 {
            break;
        }
    }
    for index in 0..line_size {
        let commons = day3_part1(&zeroes_vec);
        zeroes_vec.retain(|line| line.as_bytes()[index] == commons.1.as_bytes()[index]);
        if zeroes_vec.len() == 1 {
            break;
        }
    }
    dbg!(
        isize::from_str_radix(&ones_vec[0], 2).unwrap()
            * isize::from_str_radix(&zeroes_vec[0], 2).unwrap()
    );
}
