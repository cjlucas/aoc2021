use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day14.txt");

fn part1(input: &str) -> usize {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let mut template = template.to_string();
    let rules: HashMap<[char; 2], char> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();

            let from = from.chars().take(2).collect::<Vec<_>>().try_into().unwrap();
            let to = to.chars().nth(0).unwrap();

            (from, to)
        })
        .collect();

    for step in 0..10 {
        let mut insertions: VecDeque<(usize, char)> = VecDeque::new();

        for (idx, window) in template.chars().collect::<Vec<_>>().windows(2).enumerate() {
            let window: [char; 2] = window.try_into().unwrap();

            if let Some(ch) = rules.get(&window) {
                insertions.push_front((idx + 1, *ch));
            }
        }

        for (idx, ch) in insertions {
            template.insert(idx, ch);
        }

        println!("{}: {}", step, template);
    }

    let counts: Vec<usize> = template.chars().counts().values().cloned().collect();
    let max = *counts.iter().max().unwrap();
    let min = *counts.iter().min().unwrap();

    max - min
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

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day14_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 1588);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 240123);
    }

    // #[test]
    // fn test_part2_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 288957);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 3260812321);
    // }
}
