use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day02.txt");

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: usize,
}

impl std::str::FromStr for Command {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        let direction = match s.next().unwrap() {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => unreachable!(),
        };

        let distance = s.next().unwrap().parse::<usize>().unwrap();

        Ok(Command {
            direction,
            distance,
        })
    }
}

fn part1(input: &str) -> usize {
    let mut pos = 0;
    let mut depth = 0;

    for cmd in parse_lines::<Command>(input) {
        match cmd.direction {
            Direction::Forward => pos += cmd.distance,
            Direction::Up => depth -= cmd.distance,
            Direction::Down => depth += cmd.distance,
        }
    }

    pos * depth
}

fn part2(input: &str) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in parse_lines::<Command>(input) {
        match cmd.direction {
            Direction::Forward => {
                pos += cmd.distance;
                depth += aim * cmd.distance;
            }
            Direction::Up => aim -= cmd.distance,
            Direction::Down => aim += cmd.distance,
        }
    }

    pos * depth
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 150);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1635930);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 900);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1781819478);
    }
}
