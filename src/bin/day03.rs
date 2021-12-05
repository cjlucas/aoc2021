use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day03.txt");

#[derive(Default)]
struct DiagnosticReport {
    bitstrings: Vec<String>,
    bit_tally: Vec<i32>,
}

impl DiagnosticReport {
    fn new(bitstrings: impl Iterator<Item = String>) -> Self {
        let bitstrings: Vec<_> = bitstrings.collect();
        let bit_tally = Self::tally_bitstrings(&bitstrings);

        Self {
            bitstrings,
            bit_tally,
        }
    }
    fn gamma_rate(&self) -> usize {
        let mut n = 0;

        for (idx, tally) in self.bit_tally.iter().enumerate() {
            if *tally >= 0 {
                n += 1 << (self.bit_tally.len() - idx - 1);
            }
        }

        n
    }

    fn tally_bitstrings(bitstrings: &[String]) -> Vec<i32> {
        let mut bit_tally = vec![0; bitstrings[0].len()];

        for bitstring in bitstrings {
            for (idx, c) in bitstring.char_indices() {
                if c == '1' {
                    bit_tally[idx] += 1;
                } else {
                    bit_tally[idx] -= 1;
                }
            }
        }

        bit_tally
    }

    fn epsilon_rate(&self) -> usize {
        let mut n = 0;

        for (idx, tally) in self.bit_tally.iter().enumerate() {
            if *tally < 0 {
                n += 1 << (self.bit_tally.len() - idx - 1);
            }
        }

        n
    }

    fn oxygen_generator_rating(&self) -> usize {
        let mut bitstrings = self.bitstrings.clone();
        let mut bit_tally = self.bit_tally.clone();

        for idx in 0..self.bit_tally.len() {
            if bitstrings.len() == 1 {
                break;
            }

            let c = if bit_tally[idx] >= 0 { '1' } else { '0' };

            bitstrings = bitstrings
                .into_iter()
                .filter(|bs| bs.chars().nth(idx).unwrap() == c)
                .collect();

            bit_tally = Self::tally_bitstrings(&bitstrings);
        }

        Self::bitstring_to_usize(&bitstrings[0])
    }

    fn co2_scrubber_rating(&self) -> usize {
        let mut bitstrings = self.bitstrings.clone();
        let mut bit_tally = self.bit_tally.clone();

        for idx in 0..self.bit_tally.len() {
            if bitstrings.len() == 1 {
                break;
            }

            let c = if bit_tally[idx] >= 0 { '0' } else { '1' };

            bitstrings = bitstrings
                .into_iter()
                .filter(|bs| bs.chars().nth(idx).unwrap() == c)
                .collect();

            bit_tally = Self::tally_bitstrings(&bitstrings);
        }

        Self::bitstring_to_usize(&bitstrings[0])
    }

    fn bitstring_to_usize(s: &String) -> usize {
        let mut n = 0;

        for (idx, c) in s.char_indices() {
            if c == '1' {
                n += 1 << (s.len() - idx - 1);
            }
        }

        n
    }
}

fn part1(input: &str) -> usize {
    let bitstrings = input.lines().map(|s| s.to_string());
    let mut report = DiagnosticReport::new(bitstrings);

    report.gamma_rate() * report.epsilon_rate()
}

fn find_oxygen_generator_rating(bitstrings: Vec<String>, index: usize) -> String {
    if bitstrings.len() == 1 {
        return bitstrings[0].clone();
    }

    let mut cnt = 0i32;

    for bs in &bitstrings {
        if bs.chars().nth(index).unwrap() == '1' {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    let c = if cnt >= 0 { '1' } else { '0' };

    let bitstrings = bitstrings
        .into_iter()
        .filter(|s| s.chars().nth(index).unwrap() == c)
        .collect();

    find_oxygen_generator_rating(bitstrings, index + 1)
}

fn find_co2_scubber_rating(bitstrings: Vec<String>, index: usize) -> String {
    if bitstrings.len() == 1 {
        dbg!(&bitstrings[0]);
        return bitstrings[0].clone();
    }

    let mut cnt = 0i32;

    for bs in &bitstrings {
        if bs.chars().nth(index).unwrap() == '1' {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    let c = if cnt >= 0 { '0' } else { '1' };

    let bitstrings = bitstrings
        .into_iter()
        .filter(|s| s.chars().nth(index).unwrap() == c)
        .collect();

    find_co2_scubber_rating(bitstrings, index + 1)
}

fn part2(input: &str) -> usize {
    let bitstrings = input.lines().map(|s| s.to_string());
    let mut report = DiagnosticReport::new(bitstrings);

    report.oxygen_generator_rating() * report.co2_scrubber_rating()
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
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

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 230);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4105235);
    }
}
