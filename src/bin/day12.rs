use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day12.txt");

#[derive(Default)]
struct CaveMap {
    connected_caves: HashMap<String, HashSet<Cave>>,
}

impl CaveMap {
    fn new() -> Self {
        Default::default()
    }

    fn count_paths(&self, max_small_cave_vists: usize) -> usize {
        self.build_path(CavePath::new(), &Cave::new("start"), max_small_cave_vists)
    }

    fn build_path<'a>(
        &self,
        mut path: CavePath<'a>,
        cave: &'a Cave,
        max_small_cave_vists: usize,
    ) -> usize {
        path.add(cave);

        if cave.is_end() {
            return 1;
        }

        let mut num_paths = 0;

        for connected_cave in self.connected_caves(&cave) {
            if connected_cave.is_start() {
                continue;
            }

            if connected_cave.is_small() {
                let num_vists = path.num_vists(connected_cave);

                if num_vists == max_small_cave_vists {
                    continue;
                }

                if num_vists == 1
                    && path
                        .caves
                        .iter()
                        .filter(|c| c.is_small())
                        .any(|c| path.num_vists(c) == max_small_cave_vists)
                {
                    continue;
                }
            }

            num_paths += self.build_path(path.clone(), connected_cave, max_small_cave_vists);
        }

        num_paths
    }

    fn add_connection(&mut self, a: &str, b: &str) {
        let a = Cave::new(a);
        let b = Cave::new(b);

        if !self.connected_caves.contains_key(&a.name) {
            self.connected_caves.insert(a.name.clone(), HashSet::new());
        }

        if !self.connected_caves.contains_key(&b.name) {
            self.connected_caves.insert(b.name.clone(), HashSet::new());
        }

        let caves = self.connected_caves.get_mut(&a.name).unwrap();
        caves.insert(b.clone());

        let caves = self.connected_caves.get_mut(&b.name).unwrap();
        caves.insert(a);
    }

    fn connected_caves(&self, cave: &Cave) -> Vec<&Cave> {
        self.connected_caves
            .get(&cave.name)
            .unwrap()
            .iter()
            .collect()
    }
}

impl FromStr for CaveMap {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Self::new();

        for line in s.lines() {
            for (src, dest) in line.split_once('-') {
                map.add_connection(src, dest);
            }
        }

        Ok(map)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Cave {
    name: String,
}

impl Cave {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn is_start(&self) -> bool {
        self.name == "start"
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_small(&self) -> bool {
        self.name == self.name.to_lowercase() // TODO: memoize
    }
}

#[derive(Clone, Default)]
struct CavePath<'a> {
    caves: Vec<&'a Cave>,
    visits: HashMap<&'a String, usize>,
}

impl<'a> CavePath<'a> {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, cave: &'a Cave) {
        if let Some(visits) = self.visits.get_mut(&cave.name) {
            *visits += 1;
        } else {
            self.visits.insert(&cave.name, 1);
        }

        self.caves.push(cave);
    }

    fn num_vists(&self, cave: &Cave) -> usize {
        *self.visits.get(&cave.name).unwrap_or(&0)
    }
}

fn part1(input: &str) -> usize {
    let map: CaveMap = input.parse().unwrap();
    map.count_paths(1)
}

fn part2(input: &str) -> usize {
    let map: CaveMap = input.parse().unwrap();
    map.count_paths(2)
}

fn main() {
    dbg!(part1(INPUT));
    dbg!(part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &'static str = include_str!("../../inputs/day12_sample.txt");

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT), 226);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3497);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(SAMPLE_INPUT), 3509);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 93686);
    }
}
