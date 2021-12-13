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
        build_path2(
            self,
            CavePath::new(),
            &Cave::new("start"),
            max_small_cave_vists,
        )
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Cave {
    name: String,
    connected_caves: Vec<Cave>,
}

impl Cave {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let connected_caves = vec![];

        Self {
            name,
            connected_caves,
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
struct CavePath {
    caves: Vec<Cave>,
    visits: HashMap<String, usize>,
}

impl CavePath {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, cave: Cave) {
        let name = cave.name.to_string();

        if let Some(visits) = self.visits.get_mut(&name) {
            *visits += 1;
        } else {
            self.visits.insert(name, 1);
        }

        self.caves.push(cave);
    }

    fn path(&self) -> Vec<&Cave> {
        self.caves.iter().collect()
    }

    fn num_vists(&self, cave: &Cave) -> usize {
        *self.visits.get(&cave.name).unwrap_or(&0)
    }

    fn visited_cave(&self, cave: &Cave) -> bool {
        self.num_vists(cave) > 0
    }
}

fn build_path2(
    map: &CaveMap,
    mut path: CavePath,
    cave: &Cave,
    max_small_cave_vists: usize,
) -> usize {
    path.add(cave.clone());

    if cave.is_end() {
        return 1;
    }

    let mut num_paths = 0;

    for connected_cave in map.connected_caves(&cave) {
        if connected_cave.is_start() {
            continue;
        }

        let num_vists = path.num_vists(connected_cave);

        if connected_cave.is_small() && num_vists >= max_small_cave_vists {
            continue;
        }

        num_paths += build_path2(map, path.clone(), connected_cave, max_small_cave_vists);
    }

    num_paths
}

fn build_path(
    paths: &HashMap<&str, HashSet<&str>>,
    mut memo: HashMap<String, usize>,
    mut path: Vec<String>,
    next_node: &str,
    max_small_node: usize,
) -> usize {
    path.push(next_node.to_string());

    if memo.contains_key(next_node) {
        *memo.get_mut(next_node).unwrap() += 1;
        // dbg!(memo.get(next_node));
    } else {
        memo.insert(next_node.to_string(), 1);
    }

    if next_node == "end" {
        let path = path.join(",");
        // println!("{}", path);
        return 1;
    }

    let mut num_paths = 0;

    for node in paths.get(next_node).unwrap() {
        if node == &"start" {
            continue;
        }

        if node.to_string() == node.to_lowercase()
            && (memo.get(&node.to_string()) == Some(&1)
                && path
                    .iter()
                    .filter(|n| **n == n.to_lowercase())
                    .map(|n| path.iter().filter(|x| *x == n).count() == max_small_node)
                    .any(|x| x)
                || memo.get(&node.to_string()) == Some(&2))
        {
            continue;
        }

        // println!("hereee");
        num_paths += build_path(paths, memo.clone(), path.clone(), node, max_small_node);
    }

    num_paths
}

fn part1(input: &str) -> usize {
    let mut map = CaveMap::new();

    for line in input.lines() {
        for (src, dest) in line.split_once('-') {
            map.add_connection(src, dest);
        }
    }

    map.count_paths(1)
}

fn part2(input: &str) -> usize {
    let mut paths: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        for (src, dest) in line.split_once('-') {
            if !paths.contains_key(src) {
                paths.insert(src, HashSet::new());
            }

            let dests = paths.get_mut(&src).unwrap();
            dests.insert(dest);

            if !paths.contains_key(dest) {
                paths.insert(dest, HashSet::new());
            }

            let srcs = paths.get_mut(&dest).unwrap();
            srcs.insert(src);
        }
    }

    build_path(&paths, HashMap::new(), vec![], "start", 2)
}

fn main() {
    // dbg!(part1(INPUT));
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
