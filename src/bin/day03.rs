use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day03.txt");

fn part1(input: &str) -> usize {
    let num_lines = input.lines().count();
    let num_bits = input.lines().next().unwrap().len();
    let mut counts = vec![0; num_bits];

    for line in input.lines() {
        for (idx, c) in line.char_indices() {
            if c == '1' {
                counts[idx] += 1;
            }
        }
    }

    let mut gamma_rate = 0;

    for (idx, cnt) in counts.iter().enumerate() {
        if *cnt > (num_lines - cnt) {
            gamma_rate += 1 << (num_bits - idx - 1);
        }
    }

    let mut epsilon_rate = 0;

    for (idx, cnt) in counts.iter().enumerate() {
        if *cnt < (num_lines - cnt) {
            epsilon_rate += 1 << (num_bits - idx - 1);
        }
    }

    gamma_rate * epsilon_rate
}

// fn part2(input: &str) -> usize {}

fn main() {
    dbg!(part1(INPUT));
    // dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 198);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3847100);
    }

    // #[test]
    // fn test_part2_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 900);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 1781819478);
    // }
}
