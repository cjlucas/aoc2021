use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day06.txt");

fn part1(input: &str) -> usize {
    let mut timers: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..80 {
        let mut new_fish = 0;
        for t in &mut timers {
            if *t == 0 {
                new_fish += 1;
                *t = 6;
            } else {
                *t -= 1;
            }
        }

        for _ in 0..new_fish {
            timers.push(8);
        }
    }

    timers.len()
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

    const SAMPLE_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 5934);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 353274);
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
