use aoc2021::prelude::*;

struct Grid {
    points: HashMap<Point, u64>,
}

impl Grid {
    fn width(&self) -> usize {
        self.points.keys().max_by_key(|point| point.x).unwrap().x + 1
    }

    fn height(&self) -> usize {
        self.points.keys().max_by_key(|point| point.y).unwrap().y + 1
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let neighbors = [
            // N
            Point {
                x: point.x,
                y: point.y.wrapping_sub(1),
            },
            // E
            Point {
                x: point.x.wrapping_add(1),
                y: point.y,
            },
            // S
            Point {
                x: point.x,
                y: point.y.wrapping_add(1),
            },
            // W
            Point {
                x: point.x.wrapping_sub(1),
                y: point.y,
            },
        ];

        neighbors
            .into_iter()
            .filter(|point| self.points.contains_key(&point))
            .collect()
    }

    fn risk_level(&self, point: &Point) -> u64 {
        *self.points.get(point).unwrap()
    }
}

impl FromStr for Grid {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, risk_level) in line.chars().enumerate() {
                let risk_level = risk_level.to_digit(10).unwrap() as u64;

                points.insert(Point { x, y }, risk_level);
            }
        }

        Ok(Self { points })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

const INPUT: &'static str = include_str!("../../inputs/day15.txt");

fn part1(input: &str) -> u64 {
    let grid: Grid = input.parse().unwrap();

    let mut distance: HashMap<(usize, usize), u64> = HashMap::new();

    let mut points = HashSet::new();

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let pos = (x, y);
            let dist = if pos == (0, 0) { 0 } else { u64::MAX };
            distance.insert(pos, dist);
            points.insert(Point { x, y });
        }
    }

    while !points.is_empty() {
        let point = points
            .iter()
            .cloned()
            .min_by_key(|point| distance.get(&(point.x, point.y)).unwrap())
            .unwrap();

        points.remove(&point);

        for neighbor in grid.neighbors(&point) {
            if !points.contains(&neighbor) {
                continue;
            }

            let dist = *distance.get(&(point.x, point.y)).unwrap();
            let alt = dist + grid.risk_level(&neighbor);

            if alt < *distance.get(&(neighbor.x, neighbor.y)).unwrap() {
                distance.insert((neighbor.x, neighbor.y), alt);
            }
        }

        dbg!(points.len());
    }

    let end = Point {
        x: grid.width() - 1,
        y: grid.height() - 1,
    };

    *distance.get(&(end.x, end.y)).unwrap()
}

// fn part2(input: &str) -> String {
//     todo!()
// }

fn main() {
    println!("part1: {}", part1(INPUT));
    // println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day15_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 40);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 581);
    }

    // #[test]
    // fn test_part2() {
    //     let expected = include_str!("../../outputs/day13_part2.txt");
    //     assert_eq!(part2(INPUT), expected);
    // }
}
