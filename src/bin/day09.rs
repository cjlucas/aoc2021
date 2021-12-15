use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day09.txt");

#[derive(Debug)]
struct HeightMap {
    points: Vec<Vec<Point>>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    row: usize,
    col: usize,
    height: usize,
}

impl Point {
    fn height(&self) -> usize {
        self.height
    }
}

impl HeightMap {
    fn num_rows(&self) -> usize {
        self.points.len()
    }

    fn num_columns(&self) -> usize {
        self.points[0].len()
    }

    fn point(&self, row: usize, col: usize) -> Option<&Point> {
        if col >= self.points[0].len() || row >= self.points.len() {
            return None;
        }

        Some(&self.points[row][col])
    }

    fn is_low_point(&self, point: &Point) -> bool {
        let min_adjacent_heights = self
            .adjacent_points(point)
            .into_iter()
            .map(Point::height)
            .min()
            .unwrap();

        point.height < min_adjacent_heights
    }

    fn low_points(&self) -> Vec<&Point> {
        let mut points = vec![];

        for row in 0..self.num_rows() {
            for col in 0..self.num_columns() {
                let point = self.point(row, col).unwrap();

                if self.is_low_point(point) {
                    points.push(point);
                }
            }
        }

        points
    }

    fn basins(&self) -> Vec<HashSet<&Point>> {
        self.low_points()
            .into_iter()
            .map(|point| self.basin(point))
            .collect()
    }

    fn basin<'a>(&'a self, point: &'a Point) -> HashSet<&'a Point> {
        let adjacent_basin_points: Vec<_> = self
            .adjacent_points(point)
            .into_iter()
            .filter(|p| p.height > point.height && p.height < 9)
            .collect();

        let mut points = HashSet::new();

        points.insert(point);

        for point in adjacent_basin_points {
            for p in self.basin(point) {
                points.insert(p);
            }
        }

        points
    }

    fn adjacent_points(&self, point: &Point) -> Vec<&Point> {
        let row = point.row;
        let col = point.col;

        let mut adjacent_points = vec![];
        adjacent_points.push(self.point(row.wrapping_sub(1), col)); // top
        adjacent_points.push(self.point(row, col.wrapping_add(1))); // right
        adjacent_points.push(self.point(row + 1, col)); // down
        adjacent_points.push(self.point(row, col.wrapping_sub(1))); // right

        adjacent_points.into_iter().filter_map(|x| x).collect()
    }
}

impl FromStr for HeightMap {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        let height = c.to_digit(10).unwrap() as usize;
                        Point { row, col, height }
                    })
                    .collect()
            })
            .collect();

        Ok(Self { points })
    }
}

fn part1(input: &str) -> usize {
    let map: HeightMap = input.parse().unwrap();

    map.low_points()
        .into_iter()
        .map(Point::height)
        .map(|x| x + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let map: HeightMap = input.parse().unwrap();

    map.basins()
        .iter()
        .map(HashSet::len)
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, len| acc * len)
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day09_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 15);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 577);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 1134);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1069200);
    }
}
