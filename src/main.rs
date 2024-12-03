use std::fs::read_to_string;

use aoc2024::day3;

fn main() {
    let mut sum = 0u64;
    for _ in 0..1 {
        let s = read_to_string("./input/2024/day3.txt").unwrap();
        let input = s.as_bytes();
        sum += day3::part2(&input[..input.len() - 1]) as u64;
        // sum += day3::part2(
        //     b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        // ) as u64;
    }
    dbg!(sum);
}
