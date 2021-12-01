use aoc2021::prelude::*;

fn count_incrementing_windows(window_size: usize) -> usize {
    let lines = read_lines_as::<i64>("inputs/day01.txt");
    let mut windows = lines.windows(window_size);

    let mut prev_sum: i64 = windows.next().unwrap().iter().sum();
    let mut incr_cnt = 0;

    for window in windows {
        let window_sum = window.iter().sum();

        if window_sum > prev_sum {
            incr_cnt += 1;
        }

        prev_sum = window_sum;
    }

    incr_cnt
}

fn main() {
    println!("part1 = {}", count_incrementing_windows(1));
    println!("part2 = {}", count_incrementing_windows(3));
}
