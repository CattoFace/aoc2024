use std::{
    array::from_fn,
    simd::{num::SimdUint, u32x32},
    sync::atomic::{AtomicU32, Ordering::Relaxed},
};

use aoc_runner_derive::aoc;
use bitvec::bitarr;
use fxhash::{FxHashMap, FxHashSet};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::ParallelSlice,
};

use crate::util::fast_parse;

pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    part1_simd_rayon(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    part2_table_rayon(&input[..input.len() - 1])
}
fn step_secret(mut secret: u32) -> u32 {
    const PRUNE: u32 = (1 << 24) - 1;
    let step1 = secret << 6;
    secret ^= step1;
    secret &= PRUNE;
    let step2 = secret >> 5;
    secret ^= step2;
    let step3 = secret << 11;
    secret ^= step3;
    secret &= PRUNE;
    secret
}

fn multi_step_secret_simd_chunk(secret: &[u32], steps: u32) -> u32 {
    let prune = u32x32::splat((1 << 24) - 1);
    let mut secret_simd = u32x32::from_slice(secret);
    for _step in 0..steps {
        let step1 = secret_simd << 6;
        secret_simd ^= step1;
        secret_simd &= prune;
        let step2 = secret_simd >> 5;
        secret_simd ^= step2;
        let step3 = secret_simd << 11;
        secret_simd ^= step3;
        secret_simd &= prune;
    }
    secret_simd.reduce_sum()
}

fn multi_step_secret_simd(secret: &[u32; 32], steps: u32) -> u32 {
    let prune = u32x32::splat((1 << 24) - 1);
    let mut secret_simd = u32x32::from_slice(secret);
    for _step in 0..steps {
        let step1 = secret_simd << 6;
        secret_simd ^= step1;
        secret_simd &= prune;
        let step2 = secret_simd >> 5;
        secret_simd ^= step2;
        let step3 = secret_simd << 11;
        secret_simd ^= step3;
        secret_simd &= prune;
    }
    secret_simd.reduce_sum()
}

fn multi_step_secret(secret: u32, steps: u32) -> u32 {
    (0..steps).fold(secret, |s, _| step_secret(s))
}

#[aoc(day22, part1, rayon)]
pub fn part1_rayon(mut input: &[u8]) -> u64 {
    let mut seeds = Vec::new();
    loop {
        let (seed, remainder) = fast_parse(input);
        seeds.push(seed);
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    seeds
        .into_par_iter()
        .map(|s| multi_step_secret(s, 2000) as u64)
        .sum()
}

#[aoc(day22, part1, simd_rayon)]
pub fn part1_simd_rayon(mut input: &[u8]) -> u64 {
    let mut seeds = Vec::new();
    loop {
        let (seed, remainder) = fast_parse(input);
        seeds.push(seed);
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    let mut sum = seeds
        .par_chunks_exact(32)
        .map(|chunk| multi_step_secret_simd_chunk(chunk, 2000) as u64)
        .sum();
    sum += seeds
        .par_chunks_exact(32)
        .remainder()
        .iter()
        .map(|&s| multi_step_secret(s, 2000) as u64)
        .sum::<u64>();
    sum
}

#[aoc(day22, part1, simd)]
pub fn part1_simd(mut input: &[u8]) -> u64 {
    let mut seeds = Vec::new();
    loop {
        let (seed, remainder) = fast_parse(input);
        seeds.push(seed);
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    let mut sum = seeds
        .array_chunks()
        .map(|s: &[u32; 32]| multi_step_secret_simd(s, 2000) as u64)
        .sum();
    sum += seeds
        .array_chunks::<32>()
        .remainder()
        .iter()
        .map(|&s| multi_step_secret(s, 2000) as u64)
        .sum::<u64>();
    sum
}

#[aoc(day22, part1)]
pub fn part1_first(mut input: &[u8]) -> u64 {
    let mut sum = 0u64;
    loop {
        let (first_secret, remainder) = fast_parse(input);
        sum += multi_step_secret(first_secret, 2000) as u64;
        if remainder.is_empty() {
            return sum;
        }
        input = &remainder[1..];
    }
}

#[aoc(day22, part2, table_rayon)]
pub fn part2_table_rayon(mut input: &[u8]) -> u32 {
    let profits: &[AtomicU32; 19 * 19 * 19 * 19] = &from_fn(|_| AtomicU32::new(0));
    rayon::scope(|s| loop {
        let (secret1, remainder) = fast_parse(input);
        s.spawn(move |_| {
            let price1 = secret1 % 10;
            let secret2 = step_secret(secret1);
            let price2 = secret2 % 10;
            let secret3 = step_secret(secret2);
            let diff1 = price2 + 9 - price1;
            let price3 = secret3 % 10;
            let diff2 = price3 + 9 - price2;
            let secret4 = step_secret(secret3);
            let price4 = secret4 % 10;
            let diff3 = price4 + 9 - price3;
            let mut seen_pattern = bitarr![0; 19 * 19 * 19 * 19];
            (0..1997).fold(
                (secret4, price4, diff1 * 19 * 19 + diff2 * 19 + diff3),
                |(secret, price, history), _| {
                    let next_secret = step_secret(secret);
                    let next_price = next_secret % 10;
                    let diff = next_price + 9 - price;
                    let next_history = (history * 19) % (19 * 19 * 19 * 19) + diff;
                    if !seen_pattern.replace(next_history as usize, true) {
                        profits[next_history as usize].fetch_add(next_price, Relaxed);
                    }
                    (next_secret, next_price, next_history)
                },
            );
        });
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    });
    profits.iter().map(|p| p.load(Relaxed)).max().unwrap()
}

#[aoc(day22, part2, table)]
pub fn part2_table(mut input: &[u8]) -> u32 {
    let mut profits = [0u32; 19 * 19 * 19 * 19];
    loop {
        let (secret1, remainder) = fast_parse(input);
        let price1 = secret1 % 10;
        let secret2 = step_secret(secret1);
        let price2 = secret2 % 10;
        let diff1 = price2 + 9 - price1;
        let secret3 = step_secret(secret2);
        let price3 = secret3 % 10;
        let diff2 = price3 + 9 - price2;
        let secret4 = step_secret(secret3);
        let price4 = secret4 % 10;
        let diff3 = price4 + 9 - price3;
        let mut seen_pattern = bitarr![0; 19 * 19 * 19 * 19];
        (0..1997).fold(
            (secret4, price4, diff1 * 19 * 19 + diff2 * 19 + diff3),
            |(secret, price, history), _| {
                let next_secret = step_secret(secret);
                let next_price = next_secret % 10;
                let diff = next_price + 9 - price;
                let next_history = (history * 19) % (19 * 19 * 19 * 19) + diff;
                if !seen_pattern.replace(next_history as usize, true) {
                    profits[next_history as usize] += next_price;
                }
                (next_secret, next_price, next_history)
            },
        );
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    profits.into_iter().max().unwrap()
}

#[aoc(day22, part2)]
pub fn part2_first(mut input: &[u8]) -> u32 {
    let mut profits: FxHashMap<(i8, i8, i8, i8), u32> = Default::default();
    loop {
        let (secret1, remainder) = fast_parse(input);
        // the first 4 prices don't have a full pattern and are not a possible option
        let price1 = (secret1 % 10) as i8;
        let secret2 = step_secret(secret1);
        let price2 = (secret2 % 10) as i8;
        let diff1 = price2 - price1;
        let secret3 = step_secret(secret2);
        let price3 = (secret3 % 10) as i8;
        let diff2 = price3 - price2;
        let secret4 = step_secret(secret3);
        let price4 = (secret4 % 10) as i8;
        let diff3 = price4 - price3;
        let mut seen_pattern: FxHashSet<(i8, i8, i8, i8)> = Default::default();
        // the rest of the steps
        (0..1997).fold(
            (secret4, price4, (0, diff1, diff2, diff3)),
            |(secret, price, history), _| {
                let next_secret = step_secret(secret);
                let next_price = (next_secret % 10) as i8;
                let diff = next_price - price;
                let next_history = (history.1, history.2, history.3, diff);
                // only possible pattern if it wasn't seen already
                // if it was seen already, the monkey would have stopped there
                if seen_pattern.insert(next_history) {
                    *profits.entry(next_history).or_insert(0) += next_price as u32;
                }
                (next_secret, next_price, next_history)
            },
        );
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    profits.into_values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day22::part2_first;
    #[test]
    fn sample_part2() {
        assert_eq!(
            part2_first(
                b"1
2
3
2024",
            ),
            23
        );
    }
}
