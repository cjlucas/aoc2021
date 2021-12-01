use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day01.txt");

fn count_incrementing_windows(input: &str, window_size: usize) -> usize {
    let measurements = parse_lines_as::<i64>(input);

    measurements
        .windows(window_size + 1)
        .filter(|window| window.last().unwrap() > window.first().unwrap())
        .count()
}

fn part1(input: &str) -> usize {
    count_incrementing_windows(input, 1)
}

fn part2(input: &str) -> usize {
    count_incrementing_windows(input, 3)
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(1602, part1(INPUT));
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(1633, part2(INPUT));
    }
}
