const INPUT: &'static str = include_str!("../../inputs/day07.txt");

fn calculate_fuel_usage(
    crabs: &[usize],
    pos: usize,
    fuel_usage_calculator: fn(usize) -> usize,
) -> usize {
    crabs
        .iter()
        .map(|crab| fuel_usage_calculator((*crab as i64 - pos as i64).abs() as usize))
        .sum()
}

fn find_optimal_fuel_usage(
    crabs: &[usize],
    positions: &[usize],
    fuel_usage_calculator: fn(usize) -> usize,
) -> usize {
    if positions.len() == 1 {
        return calculate_fuel_usage(crabs, positions[0], fuel_usage_calculator);
    }

    let partition = positions.len() / 2;
    let left = &positions[..partition];
    let right = &positions[partition..];

    let l = find_optimal_fuel_usage(crabs, left, fuel_usage_calculator);
    let r = find_optimal_fuel_usage(crabs, right, fuel_usage_calculator);

    l.min(r)
}

fn run(input: &str, fuel_usage_calculator: fn(usize) -> usize) -> usize {
    let crab_positions: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();
    let possible_positions: Vec<usize> = (min_pos..=max_pos).collect();

    find_optimal_fuel_usage(&crab_positions, &possible_positions, fuel_usage_calculator)
}

fn part1(input: &str) -> usize {
    run(input, |steps| steps)
}

fn part2(input: &str) -> usize {
    run(input, |steps| (0..=steps).sum())
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
