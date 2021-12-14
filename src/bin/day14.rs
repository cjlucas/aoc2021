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
    let (template, rules) = input.split_once("\n\n").unwrap();

    let template: Vec<_> = template.chars().collect();
    let mut window_freq: HashMap<[char; 2], usize> = template
        .windows(2)
        .map(|window| window.try_into().unwrap())
        .counts();
    let rules: HashMap<[char; 2], char> = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();

            let from = from.chars().take(2).collect::<Vec<_>>().try_into().unwrap();
            let to = to.chars().nth(0).unwrap();

            (from, to)
        })
        .collect();

    for step in 0..1 {
        let mut insertions: Vec<[char; 2]> = vec![];
        let mut deletions: Vec<[char; 2]> = vec![];

        for (window, &freq) in window_freq.iter() {
            if freq == 0 {
                continue;
            }

            let window: [char; 2] = (*window).try_into().unwrap();

            if let Some(ch) = rules.get(&window) {
                let mut first = window.clone();
                first[1] = *ch;
                let mut second = window.clone();
                second[0] = *ch;

                for _ in 0..freq {
                    insertions.push(first);
                    insertions.push(second);

                    deletions.push(window);
                }

                // insertions.push_front((idx + 1, *ch));
            }
        }

        for window in insertions {
            if let Some(freq) = window_freq.get_mut(window.as_slice()) {
                *freq += 1;
            } else {
                window_freq.insert(window, 1);
            }
        }

        dbg!(step);

        for window in deletions {
            *window_freq.get_mut(window.as_slice()).unwrap() -= 1;
        }
    }

    // let counts: Vec<usize> = template.chars().counts().values().cloned().collect();
    // let max = *counts.iter().max().unwrap();
    // let min = *counts.iter().min().unwrap();

    // max - min

    0
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
        assert_eq!(part1(INPUT), 2360);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 2188189693529);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 3260812321);
    // }
}
