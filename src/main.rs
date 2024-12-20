use std::fs::read_to_string;

use aoc2024::day20;

fn main() {
    let mut sum = 0u64;
    let _s = read_to_string("./input/2024/day20.txt").unwrap();
    let b = _s.as_bytes();
    for _ in 0..10 {
        // println!("{}", day18::part2(&_s));
        //
        sum += day20::part2_first(&b[..b.len() - 1]) as u64;
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
    dbg!(sum);
}
