use std::fs::read_to_string;

use aoc2024::day8;

fn main() {
    let mut sum = 0u64;
    let s = read_to_string("./input/2024/day8.txt").unwrap();
    let input = s.as_bytes();
    for _ in 0..1000 {
        sum += day8::part1_first(&input[..input.len() - 1]) as u64;
        //         sum += day7::part1_first(
        //             b"190: 10 19
        // 3267: 81 40 27
        // 83: 17 5
        // 156: 15 6
        // 7290: 6 8 6 15
        // 161011: 16 10 13
        // 192: 17 8 14
        // 21037: 9 7 18 13
        // 292: 11 6 16 20",
        //         ) as u64;
    }
    dbg!(sum);
}
