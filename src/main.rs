use std::fs::read_to_string;

use aoc2024::day2;

fn main() {
    let mut sum = 0u64;
    for _ in 0..1 {
        // let s = read_to_string("./input/2024/day2.txt").unwrap();
        // let input = s.as_bytes();
        // sum += day2::part2_opt(&input[..input.len() - 1]) as u64;
        sum += day2::part2_opt(b"44 47 48 49 48") as u64;
        //         sum += day2::part2_opt(
        //             b"7 6 4 2 1
        // 1 2 7 8 9
        // 9 7 6 2 1
        // 1 3 2 4 5
        // 8 6 4 4 1
        // 1 3 6 7 9",
        //         ) as u64;
    }
    dbg!(sum);
}
