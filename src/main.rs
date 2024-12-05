use std::fs::read_to_string;

use aoc2024::day5;

fn main() {
    let mut sum = 0u64;
    let s = read_to_string("./input/2024/day5.txt").unwrap();
    let input = s.as_bytes();
    for _ in 0..10000 {
        sum += day5::part2_single_parse(&input[..input.len() - 1]) as u64;
        //         sum += day5::part2_single_parse(
        //             b"47|53
        // 97|13
        // 97|61
        // 97|47
        // 75|29
        // 61|13
        // 75|53
        // 29|13
        // 97|29
        // 53|29
        // 61|53
        // 97|53
        // 61|29
        // 47|13
        // 75|47
        // 97|75
        // 47|61
        // 75|61
        // 47|29
        // 75|13
        // 53|13
        //
        // 75,47,61,53,29
        // 97,61,53,29,13
        // 75,29,13
        // 75,97,47,61,53
        // 61,13,29
        // 97,13,75,29,47",
        //         ) as u64;
    }
    dbg!(sum);
}
