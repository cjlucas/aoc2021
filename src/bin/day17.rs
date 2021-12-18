use aoc2021::prelude::*;

const AREA: TargetArea = TargetArea::new(240, 292, -90, -57);

struct TargetArea {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl TargetArea {
    const fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn within_target(&self, pos: (i64, i64)) -> bool {
        pos.0 >= self.min_x && pos.0 <= self.max_x && pos.1 >= self.min_y && pos.1 <= self.max_y
    }

    fn below_target(&self, pos: (i64, i64)) -> bool {
        pos.1 < self.min_y
    }

    fn overshot_target(&self, pos: (i64, i64)) -> bool {
        pos.0 > self.max_x
    }
}

struct Probe {
    velocity: (i64, i64),
    pos: (i64, i64),
}

impl Probe {
    fn new(initial_velocity: (i64, i64)) -> Self {
        Self {
            velocity: initial_velocity,
            pos: (0, 0),
        }
    }

    fn step(&mut self) {
        //  The probe's x position increases by its x velocity.
        self.pos.0 += self.velocity.0;

        // The probe's y position increases by its y velocity.
        self.pos.1 += self.velocity.1;

        // Due to drag, the probe's x velocity changes by 1 toward
        // the value 0; that is, it decreases by 1 if it is greater
        // than 0, increases by 1 if it is less than 0, or does not
        // change if it is already 0.
        if self.velocity.0.is_positive() {
            self.velocity.0 -= 1;
        } else if self.velocity.0.is_negative() {
            self.velocity.0 += 1;
        }

        // Due to gravity, the probe's y velocity decreases by 1.
        self.velocity.1 -= 1;
    }
}

// IDEA:
//
// We're maximizing y. So begin with an initial velocity of 0,0.
// This will almost certainly fall short of the target due to gravity.
// (once the projectile has fallen below the target, we know that for a fact).
// Once that happens, increment x by 1 and repeat until we reach the target.
// Then we increment y by one. We now know that for y=1, the optimal x value
// must fall between zero and the previous optimal x value. Repeat the same
// process until the target is hit. Increment y again, and repeat. Once you
// reach a y value such that a single increment of x causes the project from
// falling short of the target to overshooting it's target, we know the current
// y value is too large, therefore the max y value is y - 1.

fn part1(area: &TargetArea) -> i64 {
    let mut optimal_initial_velocity = (0, 0);

    for y in 0..100 {
        for x in 0..100 {
            let mut probe = Probe::new((x, y));

            while !area.below_target(probe.pos)
                && !area.overshot_target(probe.pos)
                && !area.within_target(probe.pos)
            {
                probe.step();
            }

            if area.within_target(probe.pos) {
                optimal_initial_velocity = (x, y);
                break;
            }
        }
    }

    let mut probe = Probe::new(optimal_initial_velocity);
    let mut max_y = probe.pos.1;

    while !area.within_target(probe.pos) {
        probe.step();
        if probe.pos.1 > max_y {
            max_y = probe.pos.1;
        }
    }

    max_y
}

fn part2(area: &TargetArea) -> u64 {
    todo!()
}

fn main() {
    println!("part1: {}", part1(&AREA));
    // println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_AREA: TargetArea = TargetArea::new(20, 30, -10, -5);

    #[test]
    fn test_probe_step() {
        let mut probe = Probe::new((7, 2));

        let mut positions = vec![];
        for _ in 0..8 {
            positions.push(probe.pos);
            probe.step();
        }

        let expected = vec![
            (0, 0),
            (7, 2),
            (13, 3),
            (18, 3),
            (22, 2),
            (25, 0),
            (27, -3),
            (28, -7),
        ];

        assert_eq!(positions, expected);
    }

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(&SAMPLE_AREA), 45);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&AREA), 4005);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 724);
    // }
}
