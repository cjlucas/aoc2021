pub mod prelude {
    pub use itertools::Itertools;
    pub use std::collections::{HashMap, HashSet, VecDeque};
    use std::fmt::Debug;
    pub use std::str::FromStr;

    pub fn parse_lines<T: FromStr>(raw_lines: &str) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        raw_lines
            .lines()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<T>>()
    }
}
