use std::fs::read_to_string;

use aoc2024::day4;

fn main() {
    let mut sum = 0u64;
    let s = read_to_string("./input/2024/day4.txt").unwrap();
    let input = s.as_bytes();
    for _ in 0..10000 {
        sum += day4::part1(&input[..input.len() - 1]) as u64;
        //         sum += day4::part1(
        //             b"MMMSXXMASM
        // MSAMXMSMSA
        // AMXSXMAAMM
        // MSAMASMSMX
        // XMASAMXAMM
        // XXAMMXXAMA
        // SMSMSASXSS
        // SAXAMASAAA
        // MAMMMXMMMM
        // MXMXAXMASX",
        //         ) as u64;
    }
    dbg!(sum);
}
