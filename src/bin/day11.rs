use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day11.txt");

struct OctopusGrid {
    octopus_energy_levels: [usize; 100],
}

impl OctopusGrid {
    fn energy_level(&self, row: usize, col: usize) -> Option<usize> {
        if row >= 10 || col >= 10 {
            return None;
        }

        Some(self.octopus_energy_levels[(row * 10) + col])
    }

    fn incr_energy_level(&mut self, row: usize, col: usize) {
        let energy_level = self.energy_level(row, col).unwrap();

        self.octopus_energy_levels[(row * 10) + col] = energy_level + 1;
    }

    fn reset_energy_level(&mut self, row: usize, col: usize) {
        self.octopus_energy_levels[(row * 10) + col] = 0;
    }

    fn adjacent_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        vec![
            // N
            (row.wrapping_sub(1), col),
            //NE
            (row.wrapping_sub(1), col.wrapping_add(1)),
            // E
            (row, col.wrapping_add(1)),
            // SE
            (row.wrapping_add(1), col.wrapping_add(1)),
            // S
            (row.wrapping_add(1), col),
            // SW
            (row.wrapping_add(1), col.wrapping_sub(1)),
            // W
            (row, col.wrapping_sub(1)),
            // NW
            (row.wrapping_sub(1), col.wrapping_sub(1)),
        ]
    }

    fn simulate_flash(&mut self, row: usize, col: usize) {
        for (row, col) in self.adjacent_coords(row, col) {
            if self.energy_level(row, col).is_some() {
                self.incr_energy_level(row, col);
                if self.energy_level(row, col).unwrap() == 10 {
                    self.simulate_flash(row, col)
                }
            }
        }
    }

    fn step(&mut self) {
        for row in 0..10 {
            for col in 0..10 {
                self.incr_energy_level(row, col);
                if self.energy_level(row, col).unwrap() == 10 {
                    self.simulate_flash(row, col);
                }
            }
        }
    }

    fn reset_flashes(&mut self) -> usize {
        let mut num_flashes = 0;

        for row in 0..10 {
            for col in 0..10 {
                if self.energy_level(row, col).unwrap() > 9 {
                    self.reset_energy_level(row, col);
                    num_flashes += 1;
                }
            }
        }

        num_flashes
    }
}

impl FromStr for OctopusGrid {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octopus_energy_levels = [0usize; 100];
        let mut idx = 0;

        for line in s.lines() {
            for c in line.chars() {
                octopus_energy_levels[idx] = c.to_digit(10).unwrap() as usize;
                idx += 1;
            }
        }

        Ok(Self {
            octopus_energy_levels,
        })
    }
}

impl std::fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..10 {
            for col in 0..10 {
                f.write_str(&self.energy_level(row, col).unwrap().to_string())?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let mut grid: OctopusGrid = input.parse().unwrap();

    let mut num_flashes = 0;

    for _ in 0..100 {
        grid.step();

        num_flashes += grid.reset_flashes();
    }

    num_flashes
}

fn part2(input: &str) -> usize {
    let mut grid: OctopusGrid = input.parse().unwrap();

    for step in 0.. {
        grid.step();

        if grid.reset_flashes() == 100 {
            return step + 1;
        }
    }

    unreachable!()
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day11_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 1656);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1640);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 195);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 312);
    }
}
