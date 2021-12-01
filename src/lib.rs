pub mod prelude {
    use std::fmt::Debug;
    use std::str::FromStr;

    pub fn read_lines_as<T: FromStr>(fpath: &str) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: Debug,
    {
        std::fs::read_to_string(fpath)
            .unwrap()
            .lines()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<T>>()
            .into_iter()
    }
}
