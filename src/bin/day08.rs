use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day08.txt");

fn part1(input: &str) -> usize {
    let mut answer = 0;

    for line in input.lines() {
        let (_, output_value) = line.split_once(" | ").unwrap();
        let output_digits: Vec<&str> = output_value.split_whitespace().collect();

        for digit in output_digits {
            match digit.len() {
                2 | 3 | 4 | 7 => answer += 1,
                _ => (),
            }
        }
    }

    answer
}

fn part2(input: &str) -> usize {
    todo!()
}

fn main() {
    dbg!(part1(INPUT));
    // dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../inputs/day08_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 26);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 470);
    }

    // #[test]
    // fn test_part2_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 12);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 20373);
    // }
}
