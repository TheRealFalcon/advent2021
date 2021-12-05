use simple_error::SimpleError;
use std::str::FromStr;

#[derive(Debug)]
struct Cell {
    value: u32,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn is_bingo(&self) -> bool {
        let row_win = [0, 5, 10, 15, 20].iter().any(|&starting_index| {
            (starting_index..starting_index + 5).all(|x| self.cells[x].marked)
        });
        let column_win = (0..5).any(|s| {
            [s, s + 5, s + 10, s + 15, s + 20]
                .iter()
                .all(|&x| self.cells[x].marked)
        });
        row_win || column_win
    }
    fn mark(&mut self, num: u32) {
        for cell in &mut self.cells {
            if cell.value == num {
                cell.marked = true;
            }
        }
    }
}

impl FromStr for Board {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Vec<Cell> = s
            .split_whitespace()
            .map(|x| Cell {
                value: x.parse::<u32>().unwrap(),
                marked: false,
            })
            .collect();
        Ok(Board { cells })
    }
}

pub fn part1(input: String) {
    let mut splits = input.split("\n\n");
    let drawings = splits
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());
    let mut boards: Vec<Board> = splits.map(|x| x.parse().unwrap()).collect();

    for drawing in drawings {
        for board in &mut boards {
            board.mark(drawing);
            if board.is_bingo() {
                let unmarked_sum = board
                    .cells
                    .iter()
                    .filter(|x| !x.marked)
                    .fold(0, |acc, x| acc + x.value);
                dbg!(unmarked_sum * drawing);
                return;
            }
        }
    }
}

pub fn part2(input: String) {
    println!("part2");
    let mut splits = input.split("\n\n");
    let drawings = splits
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());
    let mut boards: Vec<Board> = splits.map(|x| x.parse().unwrap()).collect();

    let mut total: u32 = 0;
    for drawing in drawings {
        for board in &mut boards {
            board.mark(drawing);
            if board.is_bingo() {
                let unmarked_sum = board
                    .cells
                    .iter()
                    .filter(|x| !x.marked)
                    .fold(0, |acc, x| acc + x.value);
                total = unmarked_sum * drawing;
            }
        }
        boards.retain(|x| !x.is_bingo());
        if boards.is_empty() {
            dbg!(total);
            return;
        }
    }
}
