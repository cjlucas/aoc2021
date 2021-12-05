use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day05.txt");

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn covers(&self, point: &Point) -> bool {
        let mut result = false;

        if self.start.x == point.x {
            result =
                point.y >= self.start.y.min(self.end.y) && point.y <= self.start.y.max(self.end.y);
        }

        if !result && self.start.y == point.y {
            result =
                point.x >= self.start.x.min(self.end.x) && point.x <= self.start.x.max(self.end.x);
        }

        result
    }
}

impl std::str::FromStr for Line {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        Ok(Self { start, end })
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl std::str::FromStr for Point {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Ok(Self { x, y })
    }
}

fn part1(input: &str) -> usize {
    let lines: Vec<_> = parse_lines::<Line>(input);

    let mut max_x = 0;
    let mut max_y = 0;

    for line in &lines {
        if line.start.x > max_x {
            max_x = line.start.x;
        }

        if line.end.x > max_x {
            max_x = line.end.x;
        }

        if line.start.y > max_y {
            max_y = line.start.y;
        }

        if line.end.y > max_y {
            max_y = line.end.y;
        }
    }

    let mut at_least_two_overlaps = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let point = Point { x, y };
            let mut hits = 0;

            for line in lines.iter().filter(|line| !line.is_diagonal()) {
                if line.covers(&point) {
                    hits += 1;
                }
            }

            if hits >= 2 {
                at_least_two_overlaps += 1;
            }
        }
    }

    at_least_two_overlaps
}

// fn part2(input: &str) -> usize {
// }

fn main() {
    dbg!(part1(INPUT));
    // dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day05_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    // #[test]
    // fn test_part2_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 1924);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 7686);
    // }
}
