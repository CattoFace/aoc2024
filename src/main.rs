use std::fs::read_to_string;

use aoc2024::day21_cursed::Numpad::*;
use aoc2024::day21_cursed::{self, code_to_amount};
use aoc2024::day21_extra_cursed_25;
fn main() {
    // let mut sum1 = 0u32;
    // let _s = read_to_string("./input/2024/day20.txt").unwrap();
    // let b = _s.as_bytes();

    // println!("{}", day21_cursed::code_to_amount(Zero, Two, Nine));
    // println!("{}", day21_cursed::code_to_amount(Nine, Eight, Zero));
    // println!("{}", day21_cursed::code_to_amount(One, Seven, Nine));
    // println!("{}", day21_cursed::code_to_amount(Four, Five, Six));
    // println!("{}", day21_cursed::code_to_amount(Three, Seven, Nine));
    let s = 29 * day21_cursed::code_to_amount(Zero, Two, Nine, 25)
        + 980 * day21_cursed::code_to_amount(Nine, Eight, Zero, 25)
        + 179 * day21_cursed::code_to_amount(One, Seven, Nine, 25)
        + 456 * day21_cursed::code_to_amount(Four, Five, Six, 25)
        + 379 * day21_cursed::code_to_amount(Three, Seven, Nine, 25);
    println!("{s}");
    // let s2 = day21_extra_cursed_25::part2_single_lookup(29)
    //     + day21_extra_cursed_25::part2_single_lookup(980)
    //     + day21_extra_cursed_25::part2_single_lookup(179)
    //     + day21_extra_cursed_25::part2_single_lookup(456)
    //     + day21_extra_cursed_25::part2_single_lookup(379);
    // println!("{s2}");
    for i1 in 0u64..10 {
        for i2 in 0u64..10 {
            for i3 in 0u64..10 {
                let num = i1 * 100 + i2 * 10 + i3;
                let n1 = day21_cursed::i2n(i1);
                let n2 = day21_cursed::i2n(i2);
                let n3 = day21_cursed::i2n(i3);
                let ans = num * code_to_amount(n1, n2, n3, 2);
                println!("c_table[{num}] = {ans};");
            }
        }
    }
    return;

    // for _ in 0..10 {
    //     // println!("{}", day18::part2(&_s));
    //     //
    //     sum += day20::part2_first(&b[..b.len() - 1]) as u64;
    //     //         sum += day20::part2_first_inner(
    //     //             b"###############
    //     // #...#...#.....#
    //     // #.#.#.#.#.###.#
    //     // #S#...#.#.#...#
    //     // #######.#.#.###
    //     // #######.#.#...#
    //     // #######.#.###.#
    //     // ###..E#...#...#
    //     // ###.#######.###
    //     // #...###...#...#
    //     // #.#####.#.###.#
    //     // #.#...#.#.#...#
    //     // #.#.#.#.#.#.###
    //     // #...#...#...###
    //     // ###############",
    //     //             50,
    //     //         ) as u64;
    //     //         sum += day19::part2_recursive(
    //     //             b"r, wr, b, g, bwu, rb, gb, br
    //     //
    //     // rrbgbr",
    //     //         ) as u64;
    // }
}
