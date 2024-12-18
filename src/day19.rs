use std::sync::RwLock;

use aoc_runner_derive::aoc;
use fxhash::{FxHashMap, FxHashSet};
use memchr::memchr_iter;
use rayon::prelude::*;

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    part1_hashset(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    part2_rewrite_hashset(&input[..input.len() - 1])
}

fn parse_available_pattern_hashset(patterns_string: &[u8]) -> (FxHashSet<&[u8]>, usize) {
    let mut last_needle = 0usize;
    let mut max_length = 0usize;
    let mut patterns: FxHashSet<_> = memchr_iter(b' ', patterns_string)
        .map(|needle_pos| {
            let res = &patterns_string[last_needle..needle_pos - 1];
            last_needle = needle_pos + 1;
            max_length = max_length.max(res.len());
            res
        })
        .collect();
    patterns.insert(&patterns_string[last_needle..]);
    (patterns, max_length)
}

fn parse_available_pattern(patterns_string: &[u8]) -> Vec<&[u8]> {
    let mut last_needle = 0usize;
    let mut patterns: Vec<&[u8]> = memchr_iter(b' ', patterns_string)
        .map(|needle_pos| {
            let res = &patterns_string[last_needle..needle_pos - 1];
            last_needle = needle_pos + 1;
            res
        })
        .collect();
    patterns.push(&patterns_string[last_needle..]);
    patterns
}

fn count_potential_pattern_rwlock<'a>(
    potential_pattern: &'a [u8],
    patterns: &Vec<&[u8]>,
    cache: &RwLock<FxHashMap<&'a [u8], usize>>,
) -> usize {
    if potential_pattern.is_empty() {
        cache.write().unwrap().insert(potential_pattern, 1);
        return 1;
    }
    let ans = patterns
        .iter()
        .map(|&pattern| {
            if potential_pattern.starts_with(pattern) {
                let sub_pattern = &potential_pattern[pattern.len()..];
                let cached_res = cache.read().unwrap().get(sub_pattern).copied();
                if let Some(r) = cached_res {
                    r
                } else {
                    count_potential_pattern_rwlock(sub_pattern, patterns, cache)
                }
            } else {
                0
            }
        })
        .sum();
    cache.write().unwrap().insert(potential_pattern, ans);
    ans
}

fn count_potential_pattern<'a>(
    potential_pattern: &'a [u8],
    patterns: &Vec<&[u8]>,
    cache: &mut FxHashMap<&'a [u8], usize>,
) -> usize {
    if potential_pattern.is_empty() {
        cache.insert(potential_pattern, 1);
        return 1;
    }
    let ans = patterns
        .iter()
        .map(|&pattern| {
            if potential_pattern.starts_with(pattern) {
                let sub_pattern = &potential_pattern[pattern.len()..];
                let cached_res = cache.get(sub_pattern);
                if let Some(&r) = cached_res {
                    r
                } else {
                    count_potential_pattern(sub_pattern, patterns, cache)
                }
            } else {
                0
            }
        })
        .sum();
    cache.insert(potential_pattern, ans);
    ans
}

fn verify_potential_pattern_rwlock<'a>(
    potential_pattern: &'a [u8],
    patterns: &Vec<&[u8]>,
    cache: &RwLock<FxHashMap<&'a [u8], bool>>,
) -> bool {
    for &pattern in patterns {
        if potential_pattern.starts_with(pattern) {
            if potential_pattern.len() == pattern.len() {
                cache.write().unwrap().insert(potential_pattern, true);
                return true;
            }
            let sub_pattern = &potential_pattern[pattern.len()..];
            let possible_cache = cache.read().unwrap().get(sub_pattern).copied();
            if let Some(c) = possible_cache {
                if c {
                    cache.write().unwrap().insert(potential_pattern, true);
                    return true;
                }
            } else if verify_potential_pattern_rwlock(sub_pattern, patterns, cache) {
                cache.write().unwrap().insert(potential_pattern, true);
                return true;
            }
        }
    }
    cache.write().unwrap().insert(potential_pattern, false);
    false
}

fn verify_potential_pattern_rayon(
    potential_pattern: &[u8],
    patterns: &Vec<&[u8]>,
    cache: &mut [Option<bool>],
) -> bool {
    for &pattern in patterns {
        if potential_pattern.starts_with(pattern) {
            let sub_pattern = &potential_pattern[pattern.len()..];
            if let Some(c) = cache[sub_pattern.len()] {
                if c {
                    cache[potential_pattern.len()] = Some(true);
                    return true;
                }
            } else if verify_potential_pattern_rayon(sub_pattern, patterns, cache) {
                cache[potential_pattern.len()] = Some(true);
                return true;
            }
        }
    }
    cache[potential_pattern.len()] = Some(false);
    false
}

fn verify_potential_pattern_hashset_rayon(
    potential_pattern: &[u8],
    patterns: &FxHashSet<&[u8]>,
    max_length: usize,
    cache: &mut [Option<bool>],
) -> bool {
    for l in 1..potential_pattern.len().min(max_length) + 1 {
        if patterns.contains(&potential_pattern[..l]) {
            if potential_pattern.len() == l {
                cache[potential_pattern.len()] = Some(true);
                return true;
            }
            let sub_pattern = &potential_pattern[l..];
            if let Some(c) = cache[sub_pattern.len()] {
                if c {
                    cache[potential_pattern.len()] = Some(true);
                    return true;
                }
            } else if verify_potential_pattern_hashset_rayon(
                sub_pattern,
                patterns,
                max_length,
                cache,
            ) {
                cache[potential_pattern.len()] = Some(true);
                return true;
            }
        }
    }
    cache[potential_pattern.len()] = Some(false);
    false
}

fn verify_potential_pattern_hashset<'a>(
    potential_pattern: &'a [u8],
    patterns: &FxHashSet<&[u8]>,
    max_length: usize,
    cache: &mut FxHashMap<&'a [u8], bool>,
) -> bool {
    for l in 1..potential_pattern.len().min(max_length) + 1 {
        if patterns.contains(&potential_pattern[..l]) {
            if potential_pattern.len() == l {
                cache.insert(potential_pattern, true);
                return true;
            }
            let sub_pattern = &potential_pattern[l..];
            if let Some(&c) = cache.get(sub_pattern) {
                if c {
                    cache.insert(potential_pattern, true);
                    return true;
                }
            } else if verify_potential_pattern_hashset(sub_pattern, patterns, max_length, cache) {
                cache.insert(potential_pattern, true);
                return true;
            }
        }
    }
    cache.insert(potential_pattern, false);
    false
}

fn verify_potential_pattern<'a>(
    potential_pattern: &'a [u8],
    patterns: &Vec<&[u8]>,
    cache: &mut FxHashMap<&'a [u8], bool>,
) -> bool {
    for &pattern in patterns {
        if potential_pattern.starts_with(pattern) {
            if potential_pattern.len() == pattern.len() {
                cache.insert(potential_pattern, true);
                return true;
            }
            let sub_pattern = &potential_pattern[pattern.len()..];
            if let Some(&c) = cache.get(sub_pattern) {
                if c {
                    cache.insert(potential_pattern, true);
                    return true;
                }
            } else if verify_potential_pattern(sub_pattern, patterns, cache) {
                cache.insert(potential_pattern, true);
                return true;
            }
        }
    }
    cache.insert(potential_pattern, false);
    false
}

// #[aoc(day19, part2)]
pub fn part2_recursive(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    let mut cache = Default::default();
    input[patterns_end + 2..]
        .split(|&c| c == b'\n')
        .map(|potential_pattern| count_potential_pattern(potential_pattern, &patterns, &mut cache))
        .sum()
}

// #[aoc(day19, part1)]
pub fn part1_recursive(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    let mut cache = Default::default();
    input[patterns_end + 2..]
        .split(|&c| c == b'\n')
        .filter(|&potential_pattern| {
            verify_potential_pattern(potential_pattern, &patterns, &mut cache)
        })
        .count()
}

#[aoc(day19, part1, hashset_rayon)]
pub fn part1_hashset_rayon(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let (patterns, max_length) = parse_available_pattern_hashset(&input[..patterns_end]);
    input[patterns_end + 2..]
        .par_split(|&c| c == b'\n')
        .filter(|&potential_pattern| {
            let mut cache = vec![None; potential_pattern.len() + 1];
            cache[0] = Some(true);
            verify_potential_pattern_hashset_rayon(
                potential_pattern,
                &patterns,
                max_length,
                &mut cache,
            )
        })
        .count()
}

// #[aoc(day19, part1, rayon)]
pub fn part1_rayon(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    input[patterns_end + 2..]
        .par_split(|&c| c == b'\n')
        .filter(|&potential_pattern| {
            let mut cache = vec![None; potential_pattern.len() + 1];
            cache[0] = Some(true);
            verify_potential_pattern_rayon(potential_pattern, &patterns, &mut cache)
        })
        .count()
}

#[aoc(day19, part2, hashset_rayon)]
pub fn part2_hashset_rayon(input: &[u8]) -> u64 {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let (patterns, max_length) = parse_available_pattern_hashset(&input[..patterns_end]);
    input[patterns_end + 2..]
        .par_split(|&c| c == b'\n')
        .map(|potential_pattern| {
            count_potential_pattern_rewrite_hashset(potential_pattern, &patterns, max_length)
        })
        .sum()
}

// #[aoc(day19, part1, rayon_rw)]
pub fn part1_rayon_rwlock(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    let cache = Default::default();
    input[patterns_end + 2..]
        .par_split(|&c| c == b'\n')
        .filter(|&potential_pattern| {
            verify_potential_pattern_rwlock(potential_pattern, &patterns, &cache)
        })
        .count()
}

// #[aoc(day19, part2, rayon_rw)]
pub fn part2_rayon_rwlock(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    let cache = Default::default();
    input[patterns_end + 2..]
        .par_split(|&c| c == b'\n')
        .map(|potential_pattern| {
            count_potential_pattern_rwlock(potential_pattern, &patterns, &cache)
        })
        .sum()
}

#[aoc(day19, part2, rewrite_hashset)]
pub fn part2_rewrite_hashset(input: &[u8]) -> u64 {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let (patterns, max_length) = parse_available_pattern_hashset(&input[..patterns_end]);
    input[patterns_end + 2..]
        .split(|&c| c == b'\n')
        .map(|potential_pattern| {
            count_potential_pattern_rewrite_hashset(potential_pattern, &patterns, max_length)
        })
        .sum()
}

// #[aoc(day19, part2, rewrite)]
pub fn part2_rewrite(input: &[u8]) -> u64 {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    input[patterns_end + 2..]
        .split(|&c| c == b'\n')
        .map(|potential_pattern| count_potential_pattern_rewrite(potential_pattern, &patterns))
        .sum()
}

#[aoc(day19, part1, hashset)]
pub fn part1_hashset(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let (patterns, max_length) = parse_available_pattern_hashset(&input[..patterns_end]);
    let mut cache = Default::default();
    input[patterns_end + 2..]
        .split(|&c| c == b'\n')
        .filter(|potential_pattern| {
            verify_potential_pattern_hashset(potential_pattern, &patterns, max_length, &mut cache)
        })
        .count()
}

// #[aoc(day19, part1, rewrite)]
pub fn part1_rewrite(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    input[patterns_end + 2..]
        .split(|&c| c == b'\n')
        .filter(|potential_pattern| verify_potential_pattern_rewrite(potential_pattern, &patterns))
        .count()
}

fn count_potential_pattern_rewrite_hashset(
    potential_pattern: &[u8],
    patterns: &FxHashSet<&[u8]>,
    max_length: usize,
) -> u64 {
    let mut reachable = vec![0u64; potential_pattern.len() + 1];
    reachable[0] = 1;
    for start in 0..potential_pattern.len() {
        let max_length = max_length.min(potential_pattern.len() - start);
        let reachable_current = reachable[start];
        if reachable_current == 0 {
            continue;
        }
        for end in start..start + max_length + 1 {
            if patterns.contains(&&potential_pattern[start..end]) {
                reachable[end] += reachable_current;
            }
        }
    }
    reachable[potential_pattern.len()]
}

fn count_potential_pattern_rewrite(potential_pattern: &[u8], patterns: &Vec<&[u8]>) -> u64 {
    let mut reachable = vec![0u64; potential_pattern.len() + 1];
    reachable[0] = 1;
    for start in 0..potential_pattern.len() {
        let reachable_current = reachable[start];
        if reachable_current == 0 {
            continue;
        }
        patterns.iter().for_each(|&p| {
            if potential_pattern[start..].starts_with(p) {
                reachable[start + p.len()] += reachable_current;
            }
        });
    }
    reachable[potential_pattern.len()]
}

fn verify_potential_pattern_rewrite(potential_pattern: &[u8], patterns: &Vec<&[u8]>) -> bool {
    let mut reachable = vec![false; potential_pattern.len() + 1];
    reachable[0] = true;
    for start in 0..potential_pattern.len() {
        let reachable_current = reachable[start];
        if !reachable_current {
            continue;
        }
        patterns.iter().for_each(|&p| {
            if potential_pattern[start..].starts_with(p) {
                reachable[start + p.len()] = true;
            }
        });
    }
    reachable[potential_pattern.len()]
}
