use std::fs::read_to_string;

use aoc2024::{day22, day23};

fn main() {
    let mut sum1 = 0u32;
    let _s = read_to_string("./input/2024/day23.txt").unwrap();
    let b = _s.as_bytes();
    for _ in 0..10000 {
        println!("{}", day23::part2_brute_adj(&b[..b.len() - 1]));
        // sum += day23::part2_brute_force(&b[..b.len() - 1]) as u64;
        //         sum += day20::part2_first_inner(
        //             b"###############
        // #...#...#.....#
        // #.#.#.#.#.###.#
        // #S#...#.#.#...#
        // #######.#.#.###
        // #######.#.#...#
        // #######.#.###.#
        // ###..E#...#...#
        // ###.#######.###
        // #...###...#...#
        // #.#####.#.###.#
        // #.#...#.#.#...#
        // #.#.#.#.#.#.###
        // #...#...#...###
        // ###############",
        //             50,
        //         ) as u64;
        //         sum += day19::part2_recursive(
        //             b"r, wr, b, g, bwu, rb, gb, br
        //
        // rrbgbr",
        //         ) as u64;
    }
}
