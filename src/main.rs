use std::fs::read_to_string;

use aoc2024::day1;

fn main() {
    let mut sum = 0u64;
    for _ in 0..100000 {
        sum += day1::part1_fast(read_to_string("./input/2024/day1.txt").unwrap().as_bytes()) as u64;
    }
    dbg!(sum);
}
