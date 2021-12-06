use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day05.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
    points: HashSet<Point>,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        let mut points = HashSet::new();
        let (mut x, mut y) = (start.x, start.y);
        points.insert(start.clone());

        while x != end.x || y != end.y {
            if x < end.x {
                x += 1;
            } else if x > end.x {
                x -= 1;
            }

            if y < end.y {
                y += 1;
            } else if y > end.y {
                y -= 1
            }

            points.insert(Point { x, y });
        }

        Self { start, end, points }
    }
    fn intersecting_points<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &'a Point> {
        self.points.intersection(&other.points)
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

impl std::str::FromStr for Line {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        Ok(Self::new(start, end))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

fn count_intersecting_points(mut lines: Vec<Line>) -> usize {
    let mut intersecting_points = HashSet::new();

    while let Some(line) = lines.pop() {
        for other_line in &lines {
            for point in line.intersecting_points(&other_line) {
                intersecting_points.insert(point.clone());
            }
        }
    }

    intersecting_points.len()
}

fn part1(input: &str) -> usize {
    let lines: Vec<_> = parse_lines::<Line>(input)
        .into_iter()
        .filter(|line| !line.is_diagonal())
        .collect();

    count_intersecting_points(lines)
}

fn part2(input: &str) -> usize {
    let lines = parse_lines::<Line>(input);

    count_intersecting_points(lines)
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
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
        assert_eq!(part1(INPUT), 6113);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 20373);
    }
}
