use std::{
    collections::HashMap,
    simd::{num::SimdUint, u32x8, u8x8},
};

use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{iterator, opt},
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::util::fast_parse;

// simple sort, zip, map to get the required sum(abs(a-b))
fn part1_solve(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right)
        .map(|(&l, r)| (l as i32 - r as i32).abs())
        .sum::<i32>() as u32
}

#[aoc(day1, part1, nom)]
// parses the input using nom
pub fn part1_nom(input: &[u8]) -> u32 {
    let mut it = iterator(
        input,
        terminated(
            separated_pair(complete::u32, tag("   "), complete::u32),
            opt(newline),
        ),
    );
    let (left, right) = it.collect::<(Vec<u32>, Vec<u32>)>();
    debug_assert!({
        let res: IResult<_, _> = it.finish();
        res.is_ok()
    });
    part1_solve(left, right)
}

#[aoc(day1, part1, naive)]
// parses the input using naive line splitting
pub fn part1_naive(input: &str) -> u32 {
    let mut left_col: Vec<u32> = Vec::new();
    let mut right_col: Vec<u32> = Vec::new();
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left_col.push(parts.next().unwrap().parse().unwrap());
        right_col.push(parts.next().unwrap().parse().unwrap());
    });
    part1_solve(left_col, right_col)
}
#[aoc(day1, part1, universal)]
// parses the input using hand rolled parsing
pub fn part1(mut input: &[u8]) -> u32 {
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();
    loop {
        let (left_num, remainder) = fast_parse(input);
        // 3 spaces between numbers
        let (right_num, remainder) = fast_parse(&remainder[3..]);
        left_col.push(left_num);
        right_col.push(right_num);
        if remainder.len() <= 1 {
            break;
        }
        input = &remainder[1..];
    }
    part1_solve(left_col, right_col)
}
fn parse_line_simd(line: &[u8]) -> (u32, u32) {
    const WEIGHTS: u32x8 = u32x8::from_slice(&[10000u32, 1000u32, 100u32, 10u32, 1u32, 0, 0, 0]);
    const ZERO: u32x8 = u32x8::from_slice(&[b'0' as u32; 8]);
    let left_simd: u32x8 = u8x8::load_or_default(&line[..5]).cast();
    let right_simd: u32x8 = u8x8::load_or_default(&line[8..13]).cast();
    (
        ((left_simd - ZERO) * WEIGHTS).reduce_sum(),
        ((right_simd - ZERO) * WEIGHTS).reduce_sum(),
    )
}
fn parse_line_fast(line: &[u8]) -> (u32, u32) {
    assert!(line.len() >= 13);
    let left_num = (line[0] - b'0') as u32 * 10000u32
        + (line[1] - b'0') as u32 * 1000u32
        + (line[2] - b'0') as u32 * 100u32
        + (line[3] - b'0') as u32 * 10u32
        + (line[4] - b'0') as u32;
    let right_num = (line[8] - b'0') as u32 * 10000u32
        + (line[9] - b'0') as u32 * 1000u32
        + (line[10] - b'0') as u32 * 100u32
        + (line[11] - b'0') as u32 * 10u32
        + (line[12] - b'0') as u32;
    (left_num, right_num)
}
#[aoc(day1, part1)]
// parsing the input optimised for the real input shape
pub fn part1_fast(input: &[u8]) -> u32 {
    let (left_col, right_col) = input.chunks(14).map(parse_line_fast).unzip();
    part1_solve(left_col, right_col)
}

#[aoc(day1, part1, simd)]
// parsing the input optimised for the real input shape
pub fn part1_simd(input: &[u8]) -> u32 {
    let (left_col, right_col) = input.chunks(14).map(parse_line_simd).unzip();
    part1_solve(left_col, right_col)
}
#[aoc(day1, part2, naive)]
pub fn part2_naive(input: &str) -> u32 {
    let mut left_col = Vec::<u32>::new();
    let mut right_col = HashMap::<u32, u16>::new();
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left_col.push(parts.next().unwrap().parse().unwrap());
        right_col
            .entry(parts.next().unwrap().parse().unwrap())
            .and_modify(|r| *r += 1)
            .or_insert(1);
    });
    left_col
        .iter()
        .map(|num| num * *right_col.get(num).unwrap_or(&0u16) as u32)
        .sum()
}
#[aoc(day1, part2, universal)]
pub fn part2(mut input: &[u8]) -> u32 {
    let mut left_col = Vec::<u32>::new();
    let mut right_col =
        fxhash::FxHashMap::<u32, u16>::with_capacity_and_hasher(1000, Default::default());
    loop {
        let (left_num, remainder) = fast_parse(input);
        // 3 spaces between numbers
        let (right_num, remainder) = fast_parse(&remainder[3..]);
        left_col.push(left_num);
        right_col
            .entry(right_num)
            .and_modify(|r| *r += 1)
            .or_insert(1);
        if remainder.len() <= 1 {
            break;
        }
        input = &remainder[1..];
    }
    left_col
        .iter()
        .map(|num| num * *right_col.get(num).unwrap_or(&0u16) as u32)
        .sum()
}

// parsing the input optimised for the real input shape
#[aoc(day1, part2)]
pub fn part2_fast(input: &[u8]) -> u32 {
    let mut left_col = Vec::<u32>::new();
    let mut right_col =
        fxhash::FxHashMap::<u32, u16>::with_capacity_and_hasher(1000, Default::default());
    input.chunks(14).for_each(|line| {
        let (l, r) = parse_line_fast(line);
        left_col.push(l);
        right_col.entry(r).and_modify(|r| *r += 1).or_insert(1);
    });
    left_col
        .iter()
        .map(|num| num * *right_col.get(num).unwrap_or(&0u16) as u32)
        .sum()
}
// parsing the input optimised for the real input shape using SIMD
#[aoc(day1, part2, simd)]
pub fn part2_simd(input: &[u8]) -> u32 {
    let mut left_col = Vec::<u32>::new();
    let mut right_col =
        fxhash::FxHashMap::<u32, u16>::with_capacity_and_hasher(1000, Default::default());
    input.chunks(14).for_each(|line| {
        let (l, r) = parse_line_simd(line);
        left_col.push(l);
        right_col.entry(r).and_modify(|r| *r += 1).or_insert(1);
    });
    left_col
        .iter()
        .map(|num| num * *right_col.get(num).unwrap_or(&0u16) as u32)
        .sum()
}
#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1(
                b"3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11
        )
    }

    #[test]
    fn sample_part2() {
        assert_eq!(
            part2(
                b"3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            31
        )
    }
}
