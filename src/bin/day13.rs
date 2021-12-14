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
                x: fold_pos - (point.x - fold_pos),
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
                y: fold_pos - (point.y - fold_pos),
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

enum FoldInstruction {
    Up(usize),
    Left(usize),
}

impl FromStr for FoldInstruction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, pos) = s
            .split_whitespace()
            .last()
            .unwrap()
            .split_once('=')
            .unwrap();

        let pos: usize = pos.parse().unwrap();

        let inst = if axis == "x" {
            FoldInstruction::Left(pos)
        } else {
            FoldInstruction::Up(pos)
        };

        Ok(inst)
    }
}

fn process_fold_instructions(
    paper: &mut TransparentPaper,
    fold_instructions: Vec<FoldInstruction>,
) {
    for fold_instruction in fold_instructions {
        match fold_instruction {
            FoldInstruction::Up(pos) => paper.fold_y(pos),
            FoldInstruction::Left(pos) => paper.fold_x(pos),
        }
    }
}

fn part1(input: &str) -> usize {
    let (paper, fold_instructions) = input.split_once("\n\n").unwrap();
    let mut paper: TransparentPaper = paper.parse().unwrap();
    let fold_instructions: Vec<_> = fold_instructions
        .lines()
        .map(|line| line.parse().unwrap())
        .take(1)
        .collect();

    process_fold_instructions(&mut paper, fold_instructions);

    paper.points.len()
}

fn part2(input: &str) -> String {
    let (paper, fold_instructions) = input.split_once("\n\n").unwrap();
    let mut paper: TransparentPaper = paper.parse().unwrap();
    let fold_instructions: Vec<_> = fold_instructions
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    process_fold_instructions(&mut paper, fold_instructions);

    paper.to_string()
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2:");
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
