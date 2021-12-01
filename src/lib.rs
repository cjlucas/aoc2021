use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn read_lines_as<T: FromStr>(fpath: &str) -> impl Iterator<Item = T>
where
    <T as FromStr>::Err: Debug,
{
    fs::read_to_string(fpath)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect::<Vec<String>>()
        .into_iter()
        .map(|s| s.parse().unwrap())
}
