use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day07.txt");

fn part1(input: &str) -> usize {
    let crab_positions: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();

    let mut best = vec![];

    for pos in min_pos..=max_pos {
        let mut fuel_usage = 0;

        for crab in &crab_positions {
            fuel_usage += (*crab as i64 - pos as i64).abs() as usize
        }

        best.push((pos, fuel_usage));
    }

    *best
        .iter()
        .min_by_key(|(_, fuel_usage)| fuel_usage)
        .map(|(_, fuel_usage)| fuel_usage)
        .unwrap()
}

fn part2(input: &str) -> usize {
    let crab_positions: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();

    let mut best = vec![];

    for pos in min_pos..=max_pos {
        let mut fuel_usage = 0;

        for crab in &crab_positions {
            let steps = (*crab as i64 - pos as i64).abs() as usize;

            for i in 0..=steps {
                fuel_usage += i;
            }
        }

        best.push((pos, fuel_usage));
    }

    *best
        .iter()
        .min_by_key(|(_, fuel_usage)| fuel_usage)
        .map(|(_, fuel_usage)| fuel_usage)
        .unwrap()
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 37);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 364898);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 168);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 104149091);
    }
}
