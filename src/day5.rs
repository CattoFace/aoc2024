use std::cmp::Ordering;

use aoc_runner_derive::aoc;
use tinyvec::ArrayVec;

pub fn part1(input: &str) -> u32 {
    part1_opt(input.as_bytes())
}

pub fn part2(input: &str) -> u32 {
    part2_single_parse(input.as_bytes())
}

fn parse_rules_vec(input: &[u8]) -> ([ArrayVec<[u8; 128]>; 100], &[u8]) {
    let mut rules: [ArrayVec<[u8; 128]>; 100] = std::array::from_fn(|_| ArrayVec::new());
    let mut size = 0usize;
    input
        .array_chunks()
        .take_while(|line| line[0] != b'\n')
        .for_each(|line: &[u8; 6]| {
            // all numbers are 2 digits, easiest way to parse
            let first = (line[0] - b'0') * 10 + line[1] - b'0';
            let second = (line[3] - b'0') * 10 + line[4] - b'0';
            rules[first as usize].push(second);
            size += 1;
        });
    let remainder_index = size * 6 + 1;
    (rules, &input[remainder_index..])
}

fn parse_rules(input: &[u8]) -> ([[bool; 100]; 100], &[u8]) {
    let mut rules = [[false; 100]; 100];
    let mut size = 0usize;
    input
        .array_chunks()
        .take_while(|line| line[0] != b'\n')
        .for_each(|line: &[u8; 6]| {
            // all numbers are 2 digits, easiest way to parse
            let first = (line[0] - b'0') * 10 + line[1] - b'0';
            let second = (line[3] - b'0') * 10 + line[4] - b'0';
            rules[first as usize][second as usize] = true;
            size += 1;
        });
    let remainder_index = size * 6 + 1;
    (rules, &input[remainder_index..])
}

fn line_predicate_vec(line: &[u8], rules: &[ArrayVec<[u8; 128]>; 100]) -> u8 {
    let mut seen = [false; 100];
    let line_valid = line.chunks(3).all(|chunk: &[u8]| {
        let first = (chunk[0] - b'0') * 10 + chunk[1] - b'0';
        // verify all rules of this number have been seen
        if rules[first as usize]
            .iter()
            .all(|&second| !seen[second as usize])
        {
            seen[first as usize] = true;
            true
        } else {
            false
        }
    });
    if line_valid {
        // cant eliminate the 3s because of rounding
        let middle_num_start = line.len() / 3 / 2 * 3;
        (line[middle_num_start] - b'0') * 10 + line[middle_num_start + 1] - b'0'
    } else {
        0
    }
}

fn line_predicate_sort(numbers: &[u8], rules: &[[bool; 100]; 100]) -> bool {
    numbers.is_sorted_by(|&x, &y| rules[x as usize][y as usize])
}
fn line_fix_sort(mut numbers: ArrayVec<[u8; 64]>, rules: &[[bool; 100]; 100]) -> u8 {
    numbers.sort_unstable_by(|&x, &y| {
        if rules[x as usize][y as usize] {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    numbers[numbers.len() / 2]
}

fn line_predicate2(numbers: &[u8], rules: &[[bool; 100]; 100]) -> bool {
    let mut seen = ArrayVec::<[u8; 64]>::new();
    numbers.iter().all(|&first| {
        // verify all rules of this number have been seen
        if seen
            .iter()
            .all(|&second| !rules[first as usize][second as usize])
        {
            seen.push(first);
            true
        } else {
            false
        }
    })
}

fn line_predicate(line: &[u8], rules: &[[bool; 100]; 100]) -> u8 {
    let num_count = line.len() / 3;
    let mut seen = ArrayVec::<[u8; 64]>::new();
    // let mut seen = ArrayVec::<[u8; 64]>::new();
    let line_valid = line.chunks(3).all(|chunk: &[u8]| {
        let first = (chunk[0] - b'0') * 10 + chunk[1] - b'0';
        // verify all rules of this number were not seen
        if seen
            .iter()
            .all(|&second| !rules[first as usize][second as usize])
        {
            seen.push(first);
            true
        } else {
            false
        }
    });
    if line_valid {
        let middle_num_start = num_count / 2 * 3;
        (line[middle_num_start] - b'0') * 10 + line[middle_num_start + 1] - b'0'
    } else {
        0
    }
}

#[aoc(day5, part1, vec)]
pub fn part1_vec(input: &[u8]) -> u32 {
    let (rules, remainder) = parse_rules_vec(input);
    remainder
        .split_inclusive(|&c| c == b'\n')
        .map(|line| line_predicate_vec(line, &rules) as u32)
        .sum()
}

#[aoc(day5, part1, sort)]
pub fn part1_sort(input: &[u8]) -> u32 {
    let mut sum = 0u32;
    let (rules, mut remainder) = parse_rules(input);
    let mut numbers: ArrayVec<[u8; 64]> = ArrayVec::new();
    loop {
        match remainder.get(2) {
            Some(b',') => numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0'),
            Some(b'\n') => {
                numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0');
                if line_predicate_sort(&numbers, &rules) {
                    sum += numbers[numbers.len() / 2] as u32;
                }
                numbers.clear();
            }
            None => {
                numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0');
                if line_predicate_sort(&numbers, &rules) {
                    sum += numbers[numbers.len() / 2] as u32;
                }
                return sum;
            }
            _ => unreachable!(),
        }
        remainder = &remainder[3..];
    }
}

#[aoc(day5, part1, rewrite)]
pub fn part1_opt(input: &[u8]) -> u32 {
    let (rules, remainder) = parse_rules(input);
    remainder
        .split_inclusive(|&c| c == b'\n')
        .map(|line| line_predicate(line, &rules) as u32)
        .sum()
}

fn line_fix_single_rules(line: &[u8], rules: &[ArrayVec<[u8; 128]>; 100]) -> u8 {
    let mut count = 0usize;
    let middle_count = line.len() / 6 + 1;
    let mut nums: ArrayVec<[u8; 128]> = line
        .chunks(3)
        .map(|chunk: &[u8]| (chunk[0] - b'0') * 10 + chunk[1] - b'0')
        .collect();
    loop {
        let to_insert = nums
            .iter()
            .position(|&candidate| {
                rules[candidate as usize]
                    .iter()
                    .all(|other| !nums.contains(other))
            })
            .unwrap();
        let num = nums.swap_remove(to_insert);
        count += 1;
        if count == middle_count {
            return num;
        }
    }
}

fn line_fix_preparsed(mut numbers: ArrayVec<[u8; 64]>, rules: &[[bool; 100]; 100]) -> u8 {
    let middle_count = numbers.len() / 2;
    let mut inserted = ArrayVec::<[u8; 64]>::new();
    loop {
        let to_insert = numbers
            .iter()
            .position(|&first| {
                numbers
                    .iter()
                    .all(|&second| !rules[first as usize][second as usize])
            })
            .unwrap();
        let num = numbers.swap_remove(to_insert);
        if inserted.len() == middle_count {
            return num;
        }
        {
            inserted.push(num);
        }
    }
}

fn line_fix(line: &[u8], rules: &[[bool; 100]; 100]) -> u8 {
    let number_count = line.len() / 3;
    let middle_count = number_count / 2;
    let mut inserted = ArrayVec::<[u8; 64]>::new();
    let mut nums: Vec<u8> = line
        .chunks(3)
        .map(|chunk: &[u8]| (chunk[0] - b'0') * 10 + chunk[1] - b'0')
        .collect();
    loop {
        let to_insert = nums
            .iter()
            .position(|&first| {
                nums.iter()
                    .all(|&second| !rules[first as usize][second as usize])
            })
            .unwrap();
        let num = nums.swap_remove(to_insert);
        if inserted.len() == middle_count {
            return num;
        }
        {
            inserted.push(num);
        }
    }
}

#[aoc(day5, part2, single_rules)]
pub fn part2_single_rules(input: &[u8]) -> u32 {
    let (rules, remainder) = parse_rules_vec(input);
    remainder
        .split_inclusive(|&c| c == b'\n')
        .map(|line| {
            if line_predicate_vec(line, &rules) != 0 {
                0
            } else {
                line_fix_single_rules(line, &rules) as u32
            }
        })
        .sum()
}

#[aoc(day5, part2, sort)]
pub fn part2_single_sort(input: &[u8]) -> u32 {
    let mut sum = 0u32;
    let (rules, mut remainder) = parse_rules(input);
    let mut numbers: ArrayVec<[u8; 64]> = ArrayVec::new();
    loop {
        match remainder.get(2) {
            Some(b',') => numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0'),
            Some(b'\n') => {
                numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0');
                if !line_predicate_sort(&numbers, &rules) {
                    sum += line_fix_sort(numbers, &rules) as u32
                }
                numbers.clear();
            }
            None => {
                numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0');
                if !line_predicate_sort(&numbers, &rules) {
                    sum += line_fix_sort(numbers, &rules) as u32
                }
                return sum;
            }
            _ => unreachable!(),
        }
        remainder = &remainder[3..];
    }
}
#[aoc(day5, part2, single_parse)]
pub fn part2_single_parse(input: &[u8]) -> u32 {
    let mut sum = 0u32;
    let (rules, mut remainder) = parse_rules(input);
    let mut numbers: ArrayVec<[u8; 64]> = ArrayVec::new();
    loop {
        match remainder.get(2) {
            Some(b',') => numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0'),
            Some(b'\n') => {
                numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0');
                if !line_predicate2(&numbers, &rules) {
                    sum += line_fix_preparsed(numbers, &rules) as u32
                }
                numbers.clear();
            }
            None => {
                numbers.push((remainder[0] - b'0') * 10 + remainder[1] - b'0');
                if !line_predicate2(&numbers, &rules) {
                    sum += line_fix_preparsed(numbers, &rules) as u32
                }
                return sum;
            }
            _ => unreachable!(),
        }
        remainder = &remainder[3..];
    }
}

#[aoc(day5, part2, rewrite)]
pub fn part2_first(input: &[u8]) -> u32 {
    let (rules, remainder) = parse_rules(input);
    remainder
        .split_inclusive(|&c| c == b'\n')
        .map(|line| {
            if line_predicate(line, &rules) != 0 {
                0
            } else {
                line_fix(line, &rules) as u32
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day5::{part1_opt, part2_first};

    #[test]
    fn sample_part1_naive() {
        assert_eq!(
            part1_opt(
                b"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            143
        )
    }

    #[test]
    fn sample_part2_naive() {
        assert_eq!(
            part2_first(
                b"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            9
        )
    }
}
