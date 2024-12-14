use std::fs::read_to_string;

use aoc2024::day14;

fn main() {
    let mut sum = 0u64;
    let _s = read_to_string("./input/2024/day13.txt").unwrap();
    for _ in 0..1 {
        // sum += day14::part1(&_s) as u64;
        sum += day14::part1_rem(
            b"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        ) as u64;
    }
    dbg!(sum);
}
