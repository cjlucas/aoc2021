use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day03.txt");

#[derive(Default)]
struct DiagnosticReport {
    bitstrings: Vec<String>,
    bit_tally: Vec<i32>,
}

impl DiagnosticReport {
    fn add_bitstring(&mut self, bitstring: impl AsRef<str>) {
        self.bitstrings.push(bitstring.as_ref().to_string());

        for (idx, c) in bitstring.as_ref().char_indices() {
            if idx >= self.bit_tally.len() {
                self.bit_tally.push(0);
            }

            if c == '1' {
                self.bit_tally[idx] += 1;
            } else {
                self.bit_tally[idx] -= 1;
            }
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

    fn epsilon_rate(&self) -> usize {
        let mut n = 0;

        for (idx, tally) in self.bit_tally.iter().enumerate() {
            if *tally < 0 {
                n += 1 << (self.bit_tally.len() - idx - 1);
            }
        }

        n
    }
}

fn part1(input: &str) -> usize {
    let mut report = DiagnosticReport::default();

    for line in input.lines() {
        report.add_bitstring(line);
    }

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
    let s = find_oxygen_generator_rating(input.lines().map(str::to_string).collect(), 0);

    let mut oxygen_generator_rating = 0;

    for (idx, c) in s.char_indices() {
        if c == '1' {
            oxygen_generator_rating += 1 << (s.len() - idx - 1);
        }
    }

    let s = find_co2_scubber_rating(input.lines().map(str::to_string).collect(), 0);

    let mut co2_scrubber_rating = 0;

    for (idx, c) in s.char_indices() {
        if c == '1' {
            co2_scrubber_rating += 1 << (s.len() - idx - 1);
        }
    }

    oxygen_generator_rating * co2_scrubber_rating
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
