use std::fs::read_to_string;

use aoc2024::day11;

fn main() {
    let mut sum = 0u64;
    let s = read_to_string("./input/2024/day10.txt").unwrap();
    for _ in 0..10 {
        sum += day11::part2(&s) as u64;
        //         sum += day10::part1_first(
        //             b"0123
        // 1234
        // 8765
        // 9876",
        //         ) as u64;
    }
    dbg!(sum);
}
