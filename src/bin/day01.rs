use aoc2021::prelude::*;

fn count_incrementing_windows(window_size: usize) -> usize {
    let measurements = read_lines_as::<i64>("inputs/day01.txt");

    measurements
        .windows(window_size + 1)
        .filter(|window| window.last().unwrap() > window.first().unwrap())
        .count()
}

fn part1() -> usize {
    count_incrementing_windows(1)
}

fn part2() -> usize {
    count_incrementing_windows(3)
}

fn main() {
    dbg!(part1());
    dbg!(part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1602, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1633, part2());
    }
}
