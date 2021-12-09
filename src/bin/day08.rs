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
    let mut digits: [HashSet<char>; 10] = Default::default();
    digits[0] = "abcefg".chars().collect();
    digits[1] = "cf".chars().collect();
    digits[2] = "acdeg".chars().collect();
    digits[3] = "acdfg".chars().collect();
    digits[4] = "bcdf".chars().collect();
    digits[5] = "abdfg".chars().collect();
    digits[6] = "abdefg".chars().collect();
    digits[7] = "acf".chars().collect();
    digits[8] = "abcdefg".chars().collect();
    digits[9] = "abcdfg".chars().collect();

    let mut mapping: HashMap<char, HashSet<char>> = HashMap::new();
    mapping.insert('a', "abcdefg".chars().collect());
    mapping.insert('b', "abcdefg".chars().collect());
    mapping.insert('c', "abcdefg".chars().collect());
    mapping.insert('d', "abcdefg".chars().collect());
    mapping.insert('e', "abcdefg".chars().collect());
    mapping.insert('f', "abcdefg".chars().collect());
    mapping.insert('g', "abcdefg".chars().collect());

    for line in input.lines() {
        let (signal_patterns, output_digits) = line.split_once(" | ").unwrap();
        let signal_patterns: Vec<&str> = signal_patterns.split_whitespace().collect();
        let output_digits: Vec<&str> = output_digits.split_whitespace().collect();

        let mut partial_mappings: HashMap<String, String> = HashMap::new();
        for pattern in &signal_patterns {
            for (i, digit) in digits.iter().enumerate() {
                if (i == 1 || i == 4 || i == 7 || i == 8) && pattern.len() == digit.len() {
                    partial_mappings.insert(pattern.chars().collect(), digit.iter().collect());
                }
            }
        }

        loop {
            let mut new_partial_mappings: HashMap<String, String> = HashMap::new();

            for (signal1, segment1) in &partial_mappings {
                let signal1 = signal1.chars().collect::<HashSet<_>>();
                let segment1 = segment1.chars().collect::<HashSet<_>>();

                for (signal2, segment2) in &partial_mappings {
                    let signal2 = signal2.chars().collect::<HashSet<_>>();
                    let segment2 = segment2.chars().collect::<HashSet<_>>();

                    if signal2.is_subset(&signal1) && signal1 != signal2 {
                        let signals = signal1.difference(&signal2).cloned().collect();
                        let segments = segment1.difference(&segment2).cloned().collect();
                        new_partial_mappings.insert(signals, segments);
                    }
                }
            }

            for (k, v) in &new_partial_mappings {
                partial_mappings.insert(k.clone(), v.clone());
            }

            let cnt = partial_mappings
                .iter()
                .filter(|(from, to)| from.len() == 1 && to.len() == 1)
                .count();

            let mut foo: HashMap<String, String> = HashMap::new();

            for (from, to) in &partial_mappings {
                if from.len() == 1 && to.len() == 1 {
                    foo.insert(from.clone(), to.clone());
                }
            }

            dbg!(&partial_mappings);
        }
    }

    0
}

fn main() {
    dbg!(part1(INPUT));
    // dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHORT_SAMPLE_INPUT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    // const SHORT_SAMPLE_INPUT: &str = "dab ab | cdfeb fcadb cdfeb cdbaf";
    const SAMPLE_INPUT: &str = include_str!("../../inputs/day08_sample.txt");

    // #[test]
    // fn test_part1_sample() {
    //     assert_eq!(part1(SAMPLE_INPUT), 26);
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(INPUT), 470);
    // }

    #[test]
    fn test_part2_short_sample() {
        assert_eq!(part2(SHORT_SAMPLE_INPUT), 5353);
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
