use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day14.txt");

fn doit(input: &str, steps: usize) -> usize {
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

    let mut char_freq: HashMap<char, usize> = template.into_iter().counts();

    for _ in 0..steps {
        let mut insertions: HashMap<[char; 2], usize> = HashMap::new();
        let mut deletions: HashMap<[char; 2], usize> = HashMap::new();

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

                *insertions.entry(first).or_default() += freq;
                *insertions.entry(second).or_default() += freq;

                *deletions.entry(window).or_default() += freq;
                *char_freq.entry(*ch).or_default() += freq;

                // insertions.push_front((idx + 1, *ch));
            }
        }

        for (window, num_insertions) in insertions {
            if let Some(freq) = window_freq.get_mut(window.as_slice()) {
                *freq += num_insertions;
            } else {
                window_freq.insert(window, num_insertions);
            }

            // let ch = rules.get(window.as_slice()).unwrap();
            // *char_freq.entry(*ch).or_default() += num_insertions;
        }

        for (window, num_deletions) in deletions {
            *window_freq.get_mut(window.as_slice()).unwrap() -= num_deletions;

            // for c in window {
            //     *char_freq.get_mut(&c).unwrap() -= num_deletions;
            // }
        }
    }

    // for (window, freq) in window_freq {
    //     for ch in window {
    //         if let Some(ch_freq) = char_freq.get_mut(&ch) {
    //             *ch_freq += freq;
    //         } else {
    //             char_freq.insert(ch, freq);
    //         }
    //     }
    // }

    let max = *char_freq.values().max().unwrap();
    let min = *char_freq.values().min().unwrap();

    dbg!(&char_freq);

    for (ch, freq) in char_freq {
        println!("{}: {}", ch, freq / 2);
    }

    max - min
}

fn part1(input: &str) -> usize {
    doit(input, 10)
}

fn part2(input: &str) -> usize {
    doit(input, 40)
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2967977072188);
    }
}
