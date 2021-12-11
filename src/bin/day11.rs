use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day11.txt");

struct OctopusGrid {
    octopi: [Octopus; 100],
}

impl OctopusGrid {
    fn octopus(&self, row: usize, col: usize) -> Option<&Octopus> {
        if row >= 10 || col >= 10 {
            return None;
        }

        Some(&self.octopi[(row * 10) + col])
    }

    fn octopus_mut(&mut self, row: usize, col: usize) -> Option<&mut Octopus> {
        if row >= 10 || col >= 10 {
            return None;
        }

        Some(&mut self.octopi[(row * 10) + col])
    }

    fn adjacent_octopi_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        vec![
            // N
            (row.wrapping_sub(1), col),
            // NE
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
        for (row, col) in self.adjacent_octopi_coords(row, col) {
            if let Some(octopus) = self.octopus_mut(row, col) {
                if octopus.incr_energy() {
                    self.simulate_flash(row, col)
                }
            }
        }
    }

    fn step(&mut self) {
        for row in 0..10 {
            for col in 0..10 {
                let octopus = self.octopus_mut(row, col).unwrap();
                if octopus.incr_energy() {
                    self.simulate_flash(row, col)
                }
            }
        }
    }

    fn reset_flashes(&mut self) -> usize {
        let mut num_flashes = 0;

        for row in 0..10 {
            for col in 0..10 {
                let octopus = self.octopus_mut(row, col).unwrap();

                if octopus.has_flashed() {
                    octopus.reset_energy();
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
        let mut octopi = [Octopus::default(); 100];
        let mut idx = 0;

        for line in s.lines() {
            for c in line.chars() {
                octopi[idx].energy = c.to_digit(10).unwrap() as usize;
                idx += 1;
            }
        }

        Ok(Self { octopi })
    }
}

impl std::fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..10 {
            for col in 0..10 {
                let octopus = self.octopus(row, col).unwrap();
                f.write_str(&octopus.energy.to_string())?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Default)]
struct Octopus {
    energy: usize,
}

impl Octopus {
    fn incr_energy(&mut self) -> bool {
        self.energy += 1;
        self.energy == 10
    }

    fn has_flashed(&self) -> bool {
        self.energy >= 10
    }

    fn reset_energy(&mut self) {
        self.energy = 0;
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
