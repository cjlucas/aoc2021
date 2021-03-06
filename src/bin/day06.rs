const INPUT: &'static str = include_str!("../../inputs/day06.txt");

fn simulate_lanternfish_lifecycle(input: &str, num_days: usize) -> usize {
    let mut timers = [0usize; 9];

    for timer in input.trim().split(',') {
        let timer: usize = timer.parse().unwrap();
        timers[timer] += 1;
    }

    for _ in 0..num_days {
        timers.rotate_left(1);
        timers[6] += timers[8];
    }

    timers.iter().sum()
}

fn part1(input: &str) -> usize {
    simulate_lanternfish_lifecycle(input, 80)
}

fn part2(input: &str) -> usize {
    simulate_lanternfish_lifecycle(input, 256)
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 5934);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 353274);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 26984457539);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1609314870967);
    }
}
