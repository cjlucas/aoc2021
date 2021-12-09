use std::str::FromStr;

use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day09.txt");

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Vec<usize>>,
}

impl HeightMap {
    fn num_rows(&self) -> usize {
        self.heights.len()
    }

    fn num_columns(&self) -> usize {
        self.heights[0].len()
    }

    fn height(&self, row: usize, col: usize) -> Option<usize> {
        if col >= self.heights[0].len() || row >= self.heights.len() {
            return None;
        }

        Some(self.heights[row][col])
    }

    fn adjacent_heights(&self, row: usize, col: usize) -> Vec<usize> {
        let mut adjacent_heights = vec![];
        adjacent_heights.push(self.height(row.wrapping_sub(1), col)); // top
        adjacent_heights.push(self.height(row, col.wrapping_add(1))); // right
        adjacent_heights.push(self.height(row + 1, col)); // down
        adjacent_heights.push(self.height(row, col.wrapping_sub(1))); // right

        adjacent_heights.into_iter().filter_map(|x| x).collect()
    }
}

impl FromStr for HeightMap {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        Ok(Self { heights })
    }
}

fn part1(input: &str) -> usize {
    let map: HeightMap = input.parse().unwrap();

    let mut risk_level = 0;
    for row in 0..map.num_rows() {
        for col in 0..map.num_columns() {
            let height = map.height(row, col).unwrap();
            let min_adjacent_heights = *map.adjacent_heights(row, col).iter().min().unwrap();

            if height < min_adjacent_heights {
                risk_level += height + 1;
            }
        }
    }

    risk_level
}

fn part2(input: &str) -> usize {
    todo!()
}

fn main() {
    dbg!(part1(INPUT));
    // dbg!(part2(INPUT));
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
        assert_eq!(part1(INPUT), 0);
    }

    // #[test]
    // fn test_part2_sample() {
    //     assert_eq!(part2(SAMPLE_INPUT), 12);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 20373);
    // }
}
