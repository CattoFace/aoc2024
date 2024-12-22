use aoc_runner_derive::aoc;

use crate::{day21_extra_cursed_2, day21_extra_cursed_25};

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_table(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    part2_table(&input[..input.len() - 1])
}
#[aoc(day21, part1)]
pub fn part1_table(input: &[u8]) -> u32 {
    let num1 = (input[0] - b'0') as usize * 100
        + (input[1] - b'0') as usize * 10
        + (input[2] - b'0') as usize;
    let num2 = (input[5] - b'0') as usize * 100
        + (input[6] - b'0') as usize * 10
        + (input[7] - b'0') as usize;
    let num3 = (input[10] - b'0') as usize * 100
        + (input[11] - b'0') as usize * 10
        + (input[12] - b'0') as usize;
    let num4 = (input[15] - b'0') as usize * 100
        + (input[16] - b'0') as usize * 10
        + (input[17] - b'0') as usize;
    let num5 = (input[20] - b'0') as usize * 100
        + (input[21] - b'0') as usize * 10
        + (input[22] - b'0') as usize;
    day21_extra_cursed_2::part2_single_lookup(num1)
        + day21_extra_cursed_2::part2_single_lookup(num2)
        + day21_extra_cursed_2::part2_single_lookup(num3)
        + day21_extra_cursed_2::part2_single_lookup(num4)
        + day21_extra_cursed_2::part2_single_lookup(num5)
}

#[aoc(day21, part2)]
pub fn part2_table(input: &[u8]) -> u64 {
    let num1 = (input[0] - b'0') as usize * 100
        + (input[1] - b'0') as usize * 10
        + (input[2] - b'0') as usize;
    let num2 = (input[5] - b'0') as usize * 100
        + (input[6] - b'0') as usize * 10
        + (input[7] - b'0') as usize;
    let num3 = (input[10] - b'0') as usize * 100
        + (input[11] - b'0') as usize * 10
        + (input[12] - b'0') as usize;
    let num4 = (input[15] - b'0') as usize * 100
        + (input[16] - b'0') as usize * 10
        + (input[17] - b'0') as usize;
    let num5 = (input[20] - b'0') as usize * 100
        + (input[21] - b'0') as usize * 10
        + (input[22] - b'0') as usize;
    day21_extra_cursed_25::part2_single_lookup(num1)
        + day21_extra_cursed_25::part2_single_lookup(num2)
        + day21_extra_cursed_25::part2_single_lookup(num3)
        + day21_extra_cursed_25::part2_single_lookup(num4)
        + day21_extra_cursed_25::part2_single_lookup(num5)
}
