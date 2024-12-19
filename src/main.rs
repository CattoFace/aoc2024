use std::fs::read_to_string;

use aoc2024::day19;

fn main() {
    let mut sum = 0u64;
    let _s = read_to_string("./input/2024/day19.txt").unwrap();
    for _ in 0..100 {
        // println!("{}", day18::part2(&_s));
        sum += day19::part2(&_s) as u64;
        //         sum += day19::part2_recursive(
        //             b"r, wr, b, g, bwu, rb, gb, br
        //
        // rrbgbr",
        //         ) as u64;
    }
    dbg!(sum);
}
