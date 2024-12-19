use std::sync::RwLock;

use aoc_runner_derive::aoc;
use fxhash::FxHashMap;
use memchr::memchr_iter;
use rayon::prelude::*;

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    part1_recursive(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    part2_recursive(&input[..input.len() - 1])
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

fn count_potential_pattern_rayon(
    potential_pattern: &[u8],
    patterns: &Vec<&[u8]>,
    cache: &mut [Option<usize>],
) -> usize {
    let ans = patterns
        .iter()
        .map(|&pattern| {
            if potential_pattern.starts_with(pattern) {
                let sub_pattern = &potential_pattern[pattern.len()..];
                if let Some(r) = cache[sub_pattern.len()] {
                    r
                } else {
                    count_potential_pattern_rayon(sub_pattern, patterns, cache)
                }
            } else {
                0
            }
        })
        .sum();
    cache[potential_pattern.len()] = Some(ans);
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
    if potential_pattern.is_empty() {
        cache.write().unwrap().insert(potential_pattern, true);
        return true;
    }
    let ans = patterns.iter().any(|&pattern| {
        potential_pattern.starts_with(pattern) && {
            let sub_pattern = &potential_pattern[pattern.len()..];
            let cached_res = cache.read().unwrap().get(sub_pattern).copied();
            if let Some(r) = cached_res {
                r
            } else {
                verify_potential_pattern_rwlock(sub_pattern, patterns, cache)
            }
        }
    });
    cache.write().unwrap().insert(potential_pattern, ans);
    ans
}

fn verify_potential_pattern_rayon(
    potential_pattern: &[u8],
    patterns: &Vec<&[u8]>,
    cache: &mut [Option<bool>],
) -> bool {
    let ans = patterns.iter().any(|&pattern| {
        potential_pattern.starts_with(pattern) && {
            let sub_pattern = &potential_pattern[pattern.len()..];
            if let Some(r) = cache[sub_pattern.len()] {
                r
            } else {
                verify_potential_pattern_rayon(sub_pattern, patterns, cache)
            }
        }
    });
    cache[potential_pattern.len()] = Some(ans);
    ans
}

fn verify_potential_pattern<'a>(
    potential_pattern: &'a [u8],
    patterns: &Vec<&[u8]>,
    cache: &mut FxHashMap<&'a [u8], bool>,
) -> bool {
    if potential_pattern.is_empty() {
        cache.insert(potential_pattern, true);
        return true;
    }
    let ans = patterns.iter().any(|&pattern| {
        potential_pattern.starts_with(pattern) && {
            let sub_pattern = &potential_pattern[pattern.len()..];
            let cached_res = cache.get(sub_pattern);
            if let Some(&r) = cached_res {
                r
            } else {
                verify_potential_pattern(sub_pattern, patterns, cache)
            }
        }
    });
    cache.insert(potential_pattern, ans);
    ans
}

#[aoc(day19, part2)]
pub fn part2_recursive(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    let mut cache = Default::default();
    input[patterns_end + 2..]
        .split(|&c| c == b'\n')
        .map(|potential_pattern| count_potential_pattern(potential_pattern, &patterns, &mut cache))
        .sum()
}

#[aoc(day19, part1)]
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

#[aoc(day19, part1, rayon)]
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

#[aoc(day19, part2, rayon)]
pub fn part2_rayon(input: &[u8]) -> usize {
    let patterns_end = memchr::memchr(b'\n', input).unwrap();
    let patterns = parse_available_pattern(&input[..patterns_end]);
    input[patterns_end + 2..]
        .par_split(|&c| c == b'\n')
        .map(|potential_pattern| {
            let mut cache = vec![None; potential_pattern.len() + 1];
            cache[0] = Some(1);
            count_potential_pattern_rayon(potential_pattern, &patterns, &mut cache)
        })
        .sum()
}

#[aoc(day19, part1, rayon_rw)]
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

#[aoc(day19, part2, rayon_rw)]
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
