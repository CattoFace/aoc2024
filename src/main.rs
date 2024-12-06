use std::fs::read_to_string;

use aoc2024::day6;

fn main() {
    let mut sum = 0u64;
    let s = read_to_string("./input/2024/day6.txt").unwrap();
    let input = s.as_bytes();
    for _ in 0..1 {
        // sum += day6::part1_first(&input[..input.len() - 1]) as u64;
        sum += day6::part2_pass_visited(
            b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        ) as u64;
    }
    dbg!(sum);
}
