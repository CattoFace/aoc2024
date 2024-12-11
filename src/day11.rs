use std::{array::from_fn, mem::swap};

use aoc_runner_derive::aoc;
use fxhash::{FxBuildHasher, FxHashMap};

use crate::util::fast_parse;

pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    part1_cached(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    part2_cached(&input[..input.len() - 1])
}
#[allow(dead_code)]
fn step(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::<u64>::with_capacity(stones.len());
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let tenpow = 10u64.pow(digits / 2);
                new_stones.push(stone / tenpow);
                new_stones.push(stone % tenpow);
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }
    new_stones
}

fn multi_step_multicache1(stone: u64, steps: u8, cache: &mut [FxHashMap<u64, u64>; 25]) -> u64 {
    if steps == 0 {
        return 1;
    }
    if let Some(expanded_amount) = cache[(steps - 1) as usize].get(&stone) {
        *expanded_amount
    } else {
        let amount = if stone == 0 {
            multi_step_multicache1(1, steps - 1, cache)
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let tenpow = 10u64.pow(digits / 2);
                let amount1 = multi_step_multicache1(stone / tenpow, steps - 1, cache);
                let amount2 = multi_step_multicache1(stone % tenpow, steps - 1, cache);
                amount1 + amount2
            } else {
                multi_step_multicache1(stone * 2024, steps - 1, cache)
            }
        };
        cache[(steps - 1) as usize].insert(stone, amount);
        amount
    }
}
fn multi_step_multicache2(stone: u64, steps: u8, cache: &mut [FxHashMap<u64, u64>; 75]) -> u64 {
    if steps == 0 {
        return 1;
    }
    if let Some(expanded_amount) = cache[(steps - 1) as usize].get(&stone) {
        *expanded_amount
    } else {
        let amount = if stone == 0 {
            multi_step_multicache2(1, steps - 1, cache)
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let tenpow = 10u64.pow(digits / 2);
                let amount1 = multi_step_multicache2(stone / tenpow, steps - 1, cache);
                let amount2 = multi_step_multicache2(stone % tenpow, steps - 1, cache);
                amount1 + amount2
            } else {
                multi_step_multicache2(stone * 2024, steps - 1, cache)
            }
        };
        cache[(steps - 1) as usize].insert(stone, amount);
        amount
    }
}

fn multi_step(stone: u64, steps: u8, cache: &mut FxHashMap<(u64, u8), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }
    if let Some(expanded_amount) = cache.get(&(stone, steps)) {
        *expanded_amount
    } else {
        let amount = if stone == 0 {
            multi_step(1, steps - 1, cache)
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let tenpow = 10u64.pow(digits / 2);
                let amount1 = multi_step(stone / tenpow, steps - 1, cache);
                let amount2 = multi_step(stone % tenpow, steps - 1, cache);
                amount1 + amount2
            } else {
                multi_step(stone * 2024, steps - 1, cache)
            }
        };
        cache.insert((stone, steps), amount);
        amount
    }
}

fn step_grouped(mut stones: FxHashMap<u64, u64>, steps: u8) -> u64 {
    let mut next_stones =
        FxHashMap::<u64, u64>::with_capacity_and_hasher(stones.len(), FxBuildHasher::new());
    for _ in 0..steps {
        // advance all groups one step
        for (&stone, &count) in &stones {
            if stone == 0 {
                next_stones
                    .entry(1)
                    .and_modify(|existing_count| *existing_count += count)
                    .or_insert(count);
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let tenpow = 10u64.pow(digits / 2);
                    next_stones
                        .entry(stone / tenpow)
                        .and_modify(|existing_count| *existing_count += count)
                        .or_insert(count);
                    next_stones
                        .entry(stone % tenpow)
                        .and_modify(|existing_count| *existing_count += count)
                        .or_insert(count);
                } else {
                    next_stones
                        .entry(stone * 2024)
                        .and_modify(|existing_count| *existing_count += count)
                        .or_insert(count);
                }
            };
        }
        // swap the maps, double buffer approach
        stones.clear();
        swap(&mut stones, &mut next_stones);
    }
    stones.into_values().sum()
}

#[aoc(day11, part1, grouped)]
pub fn part1_grouped(mut input: &[u8]) -> u64 {
    let mut stones = FxHashMap::<u64, u64>::with_capacity_and_hasher(500, FxBuildHasher::new());
    loop {
        let (num, remainder) = fast_parse(input);
        stones.entry(num).and_modify(|s| *s += 1).or_insert(1);
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    step_grouped(stones, 25)
}

#[aoc(day11, part2, grouped)]
pub fn part2_grouped(mut input: &[u8]) -> u64 {
    let mut stones = FxHashMap::<u64, u64>::with_capacity_and_hasher(5000, FxBuildHasher::new());
    loop {
        let (num, remainder) = fast_parse(input);
        stones.entry(num).and_modify(|s| *s += 1).or_insert(1);
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    step_grouped(stones, 75)
}

#[aoc(day11, part1, cached_multicache)]
pub fn part1_cached_multicache(input: &[u8]) -> u64 {
    stone_stepper_cached_multicache1(input)
}

#[aoc(day11, part1, cached)]
pub fn part1_cached(input: &[u8]) -> u64 {
    stone_stepper_cached(input, 25)
}

#[aoc(day11, part1, naive)]
pub fn part1_naive(input: &[u8]) -> u64 {
    stone_stepper_naive(input, 25)
}

#[aoc(day11, part2, cached_multicache)]
pub fn part2_cached_multicache(input: &[u8]) -> u64 {
    stone_stepper_cached_multicache2(input)
}

#[aoc(day11, part2, cached)]
pub fn part2_cached(input: &[u8]) -> u64 {
    stone_stepper_cached(input, 75)
}

pub fn stone_stepper_cached_multicache1(mut input: &[u8]) -> u64 {
    let mut cache: [FxHashMap<u64, u64>; 25] =
        from_fn(|_| FxHashMap::with_capacity_and_hasher(500, FxBuildHasher::new()));
    let mut sum = 0u64;
    loop {
        let (stone, remainder) = fast_parse(input);
        sum += multi_step_multicache1(stone, 25, &mut cache);
        if remainder.is_empty() {
            return sum;
        }
        input = &remainder[1..];
    }
}

pub fn stone_stepper_cached_multicache2(mut input: &[u8]) -> u64 {
    let mut cache: [FxHashMap<u64, u64>; 75] =
        from_fn(|_| FxHashMap::with_capacity_and_hasher(5000, FxBuildHasher::new()));
    let mut sum = 0u64;
    loop {
        let (stone, remainder) = fast_parse(input);
        sum += multi_step_multicache2(stone, 75, &mut cache);
        if remainder.is_empty() {
            return sum;
        }
        input = &remainder[1..];
    }
}

pub fn stone_stepper_cached(mut input: &[u8], steps: u8) -> u64 {
    let mut cache =
        FxHashMap::<(u64, u8), u64>::with_capacity_and_hasher(150000, FxBuildHasher::new());
    let mut sum = 0u64;
    loop {
        let (stone, remainder) = fast_parse(input);
        sum += multi_step(stone, steps, &mut cache);
        if remainder.is_empty() {
            return sum;
        }
        input = &remainder[1..];
    }
}

pub fn stone_stepper_naive(mut input: &[u8], steps: u8) -> u64 {
    let mut stones = Vec::<u64>::with_capacity(16);
    loop {
        let (num, remainder) = fast_parse(input);
        stones.push(num);
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    for _ in 0..steps {
        stones = step(stones);
    }
    stones.len() as u64
}

#[cfg(test)]
mod tests {
    use crate::day11::stone_stepper_cached;

    #[test]
    fn sample_part1() {
        assert_eq!(stone_stepper_cached(b"0 1 10 99 999", 1), 7)
    }

    #[test]
    fn sample2_part1() {
        assert_eq!(stone_stepper_cached(b"125 17", 6), 22)
    }

    #[test]
    fn sample3_part1() {
        assert_eq!(stone_stepper_cached(b"125 17", 25), 55312)
    }
}
