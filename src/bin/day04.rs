use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day04.txt");

const BINGO_BOARD_SIZE: usize = 5;

#[derive(Debug)]
struct BingoGame {
    numbers: VecDeque<usize>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn draw(&mut self) -> Option<usize> {
        self.numbers.remove(0)
    }

    fn mark_boards(&mut self, number: usize) -> Vec<BingoBoard> {
        for board in self.boards.iter_mut() {
            board.mark_number(number);
        }

        let mut winning_boards = vec![];
        while let Some(idx) = self.boards.iter().position(|board| board.has_bingo()) {
            winning_boards.push(self.boards.swap_remove(idx));
        }

        winning_boards
    }
}

impl std::str::FromStr for BingoGame {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_terminator("\n\n");

        let numbers: VecDeque<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let boards: Vec<BingoBoard> = split
            .map(|board| board.parse::<BingoBoard>().unwrap())
            .collect();

        Ok(BingoGame { numbers, boards })
    }
}

#[derive(Debug, Default)]
struct BingoBoard {
    numbers: [(usize, bool); BINGO_BOARD_SIZE * BINGO_BOARD_SIZE],
}

impl BingoBoard {
    fn new(numbers: [usize; BINGO_BOARD_SIZE * BINGO_BOARD_SIZE]) -> Self {
        let mut board = Self::default();

        for (idx, (n, _)) in board.numbers.iter_mut().enumerate() {
            *n = numbers[idx];
        }

        board
    }

    fn mark_number(&mut self, number: usize) {
        for (n, marked) in self.numbers.iter_mut() {
            if *n == number {
                *marked = true;
            }
        }
    }

    fn has_bingo(&self) -> bool {
        for i in 0..BINGO_BOARD_SIZE {
            if self.row(i).all(|(_, marked)| marked) {
                return true;
            }

            if self.col(i).all(|(_, marked)| marked) {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> usize {
        self.numbers
            .iter()
            .filter_map(|(n, marked)| if !marked { Some(n) } else { None })
            .sum()
    }

    fn row(&self, row: usize) -> impl Iterator<Item = (usize, bool)> + '_ {
        (0..BINGO_BOARD_SIZE)
            .map(move |col| (row * BINGO_BOARD_SIZE) + col)
            .map(|idx| self.numbers[idx])
    }

    fn col(&self, col: usize) -> impl Iterator<Item = (usize, bool)> + '_ {
        (0..BINGO_BOARD_SIZE)
            .map(move |row| col + (BINGO_BOARD_SIZE * row))
            .map(|idx| self.numbers[idx])
    }
}

impl std::str::FromStr for BingoBoard {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = [0; BINGO_BOARD_SIZE * BINGO_BOARD_SIZE];

        for (row, line) in s.lines().enumerate() {
            for (col, num) in line.split_whitespace().enumerate() {
                let idx = (row * BINGO_BOARD_SIZE) + col;
                numbers[idx] = num.parse().unwrap();
            }
        }

        Ok(BingoBoard::new(numbers))
    }
}

fn part1(input: &str) -> usize {
    let mut game = input.parse::<BingoGame>().unwrap();

    while let Some(n) = game.draw() {
        if let Some(board) = game.mark_boards(n).first() {
            return board.unmarked_sum() * n;
        }
    }

    unreachable!("ran out of numbers before winning board found");
}

fn part2(input: &str) -> usize {
    let mut game = input.parse::<BingoGame>().unwrap();

    let mut answer = 0;
    while let Some(n) = game.draw() {
        // Note: in the case where multiple boards win on a given draw (as happens in part2),
        // we don't actually care which one wins, even though part2 require a SINGLE BOARD
        // to be the last winning board... kinda bullshit.
        if let Some(board) = game.mark_boards(n).first() {
            answer = board.unmarked_sum() * n;
        }
    }

    answer
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day04_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 4512);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 34506);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 1924);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 7686);
    }
}
