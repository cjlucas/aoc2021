use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day13.txt");

struct TransparentPaper {
    points: HashSet<Point>,
    max_x: usize,
    max_y: usize,
}

impl TransparentPaper {
    fn new() -> Self {
        Self {
            points: HashSet::new(),
            max_x: 0,
            max_y: 0,
        }
    }
    fn add_point(&mut self, point: Point) {
        self.max_x = self.max_x.max(point.x);
        self.max_y = self.max_y.max(point.y);

        self.points.insert(point);
    }

    fn fold_x(&mut self, x: usize) {
        let fold_pos = x;

        let points = std::mem::take(&mut self.points);

        let (above, below): (Vec<_>, Vec<_>) =
            points.into_iter().partition(|point| point.x < fold_pos);

        self.points = above.into_iter().collect();

        for point in below {
            self.points.insert(Point {
                y: point.y,
                x: dbg!(fold_pos) - (dbg!(point.x) - fold_pos),
            });
        }

        self.max_x = self.points.iter().map(|point| point.x).max().unwrap();
    }

    fn fold_y(&mut self, y: usize) {
        let fold_pos = y;

        let points = std::mem::take(&mut self.points);

        let (above, below): (Vec<_>, Vec<_>) =
            points.into_iter().partition(|point| point.y < fold_pos);

        self.points = above.into_iter().collect();

        for point in below {
            self.points.insert(Point {
                x: point.x,
                y: dbg!(fold_pos) - (dbg!(point.y) - fold_pos),
            });
        }

        self.max_y = self.points.iter().map(|point| point.y).max().unwrap();
    }
}

impl std::fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if self.points.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for TransparentPaper {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut paper = Self::new();

        for point in s.lines().map(|line| line.parse().unwrap()) {
            paper.add_point(point);
        }

        Ok(paper)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Ok(Point { x, y })
    }
}

fn part1(input: &str) -> usize {
    let mut split = input.split("\n\n");
    let mut paper: TransparentPaper = split.next().unwrap().parse().unwrap();

    for fold_instruction in split.next().unwrap().lines().take(1) {
        let (axis, pos) = fold_instruction
            .split_whitespace()
            .last()
            .unwrap()
            .split_once('=')
            .unwrap();

        let pos: usize = pos.parse().unwrap();

        if axis == "x" {
            paper.fold_x(pos)
        } else {
            paper.fold_y(pos)
        }
    }

    paper.points.len()
}

fn part2(input: &str) -> String {
    let mut split = input.split("\n\n");
    let mut paper: TransparentPaper = split.next().unwrap().parse().unwrap();

    for fold_instruction in split.next().unwrap().lines() {
        let (axis, pos) = fold_instruction
            .split_whitespace()
            .last()
            .unwrap()
            .split_once('=')
            .unwrap();

        let pos: usize = pos.parse().unwrap();

        if axis == "x" {
            paper.fold_x(pos)
        } else {
            paper.fold_y(pos)
        }
    }

    paper.to_string()
}

fn main() {
    dbg!(part1(INPUT));
    println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day13_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 17);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 724);
    }

    #[test]
    fn test_part2() {
        let expected = include_str!("../../outputs/day13_part2.txt");
        assert_eq!(part2(INPUT), expected);
    }
}
