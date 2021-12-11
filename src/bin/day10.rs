const INPUT: &'static str = include_str!("../../inputs/day10.txt");

enum SyntaxError {
    UnexpectedCharacter(char),
}

fn parse_line(line: &str) -> Result<Vec<char>, SyntaxError> {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return Err(SyntaxError::UnexpectedCharacter(c));
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return Err(SyntaxError::UnexpectedCharacter(c));
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return Err(SyntaxError::UnexpectedCharacter(c));
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return Err(SyntaxError::UnexpectedCharacter(c));
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(stack)
}

fn part1(input: &str) -> usize {
    let mut syntax_error_score = 0;

    for line in input.lines() {
        if let Err(SyntaxError::UnexpectedCharacter(c)) = parse_line(line) {
            match c {
                ')' => syntax_error_score += 3,
                ']' => syntax_error_score += 57,
                '}' => syntax_error_score += 1197,
                '>' => syntax_error_score += 25137,
                _ => unreachable!(),
            }
        }
    }

    syntax_error_score
}

fn part2(input: &str) -> usize {
    let mut autocomplete_scores: Vec<usize> = vec![];

    let uncorrupted_lines = input.lines().filter_map(|line| parse_line(line).ok());
    for mut remaining_chars in uncorrupted_lines {
        let mut autocomplete_score = 0;

        while let Some(c) = remaining_chars.pop() {
            autocomplete_score *= 5;
            match c {
                '(' => autocomplete_score += 1,
                '[' => autocomplete_score += 2,
                '{' => autocomplete_score += 3,
                '<' => autocomplete_score += 4,
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
