use std::fs::read_to_string;

use aoc2024::day18;

fn main() {
    let mut sum = 0u64;
    let _s = read_to_string("./input/2024/day18.txt").unwrap();
    for _ in 0..100 {
        println!("{}", day18::part2(&_s));
        // sum += day16::part2(&_s) as u64;
        //         sum += day18::part1_first(
        //             b"5,4
        // 4,2
        // 4,5
        // 3,0
        // 2,1
        // 6,3
        // 2,4
        // 1,5
        // 0,6
        // 3,3
        // 2,6
        // 5,1",
        //         ) as u64;
    }
    dbg!(sum);
}
