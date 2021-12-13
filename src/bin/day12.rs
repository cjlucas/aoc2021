use aoc2021::prelude::*;

const INPUT: &'static str = include_str!("../../inputs/day12.txt");

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

    build_path(&paths, HashMap::new(), vec![], "start", 1)
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
