use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day10.txt");

fn part1(input: &str) -> usize {
    let mut syntax_error_score_lut: HashMap<char, usize> = HashMap::new();
    syntax_error_score_lut.insert(')', 3);
    syntax_error_score_lut.insert(']', 57);
    syntax_error_score_lut.insert('}', 1197);
    syntax_error_score_lut.insert('>', 25137);

    let mut syntax_error_score = 0;

    for line in input.lines() {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        println!("invalid line: {}", line);
                        syntax_error_score += syntax_error_score_lut.get(&')').unwrap();
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        println!("invalid line: {}", line);
                        syntax_error_score += syntax_error_score_lut.get(&']').unwrap();
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        println!("invalid line: {}", line);
                        syntax_error_score += syntax_error_score_lut.get(&'}').unwrap();
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        println!("invalid line: {}", line);
                        syntax_error_score += syntax_error_score_lut.get(&'>').unwrap();
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    syntax_error_score
}

fn part2(input: &str) -> usize {
    let mut corrupted_lines = HashSet::new();
    for line in input.lines() {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        println!("invalid line: {}", line);
                        corrupted_lines.insert(line);
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        println!("invalid line: {}", line);
                        corrupted_lines.insert(line);
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        println!("invalid line: {}", line);
                        corrupted_lines.insert(line);
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        println!("invalid line: {}", line);
                        corrupted_lines.insert(line);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let mut autocomplete_scores: Vec<usize> = vec![];

    for line in input.lines() {
        if corrupted_lines.contains(line) {
            continue;
        }

        let mut stack = vec![];
        let mut autocomplete_score = 0;

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        unreachable!();
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        unreachable!();
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        unreachable!();
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        unreachable!();
                    }
                }
                _ => unreachable!(),
            }
        }

        while let Some(c) = stack.pop() {
            match c {
                '(' => {
                    autocomplete_score *= 5;
                    autocomplete_score += 1;
                }
                '[' => {
                    autocomplete_score *= 5;
                    autocomplete_score += 2;
                }
                '{' => {
                    autocomplete_score *= 5;
                    autocomplete_score += 3;
                }
                '<' => {
                    autocomplete_score *= 5;
                    autocomplete_score += 4;
                }
                _ => unreachable!(),
            }
        }

        autocomplete_scores.push(autocomplete_score);
    }

    autocomplete_scores.sort();

    autocomplete_scores[autocomplete_scores.len() / 2]
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day10_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 26397);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 240123);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 288957);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3260812321);
    }
}
