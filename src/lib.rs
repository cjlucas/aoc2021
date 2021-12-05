pub mod prelude {
    pub use std::collections::{HashSet, VecDeque};

    use std::fmt::Debug;
    use std::str::FromStr;

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
