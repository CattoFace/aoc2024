use aoc_runner_derive::aoc;
use memchr::memmem::{self, Finder};

use crate::util::fast_parse_backwards;

pub fn part1(input: &str) -> u32 {
    part1_memchr(input.as_bytes())
}
pub fn part2(input: &str) -> u32 {
    part2_memchr(input.as_bytes())
}

fn fast_parse(input: &[u8]) -> (u16, &[u8]) {
    match input.first() {
        None => (0, input),
        Some(&i) => {
            let i = i as u16;
            match input.get(1) {
                None => (i, &input[1..]),
                Some(&j) => {
                    let j = j as u16;
                    match input.get(2) {
                        None => (i * 10 + j, &input[2..]),
                        Some(&k) => (i * 100 + j * 10 + k as u16, &input[3..]),
                    }
                }
            }
        }
    }
}

#[aoc(day3, part1, naive)]
// naive solution that searches for the next mul instruction
pub fn part1_naive(mut input: &[u8]) -> u32 {
    const MINIMUM_INSTRUCTION_LENGTH: usize = 7;
    let mut sum = 0u32;
    while let Some(i) = input.array_windows().position(|s| s.eq(b"mul(")) {
        // no room for the rest of the instruction
        if i + MINIMUM_INSTRUCTION_LENGTH >= input.len() {
            break;
        }
        let start_index = i + 4;
        let (right_num, remainder): (u16, &[u8]) = fast_parse(&input[start_index..]);
        // check for separating ','
        if remainder.len() == input.len() - start_index || remainder[0] != b',' {
            input = &remainder[1..];
            continue;
        }
        let (left_num, remainder): (u16, &[u8]) = fast_parse(&remainder[1..]);
        // check for ending ')'
        if remainder[0] == b')' {
            sum += left_num as u32 * right_num as u32;
        }
        input = &remainder[1..];
    }
    sum
}

// parses and processes mul instructions using memchr::memmem
fn sum_muls<'a>(mut input: &'a [u8], finder: &Finder) -> (u32, &'a [u8]) {
    const MINIMUM_INSTRUCTION_LENGTH: usize = 7;
    let mut sum = 0u32;
    while let Some(i) = finder.find(input) {
        // no more room for instructions
        if i + MINIMUM_INSTRUCTION_LENGTH >= input.len() {
            break;
        }
        let start_index = i + 4;
        let (right_num, remainder): (u16, &[u8]) = fast_parse(&input[start_index..]);
        if remainder.len() == input.len() - start_index || remainder[0] != b',' {
            input = &remainder[1..];
            continue;
        }
        let (left_num, remainder): (u16, &[u8]) = fast_parse(&remainder[1..]);
        if remainder[0] == b')' {
            sum += left_num as u32 * right_num as u32;
        }
        input = &remainder[1..];
    }
    (sum, input)
}

#[aoc(day3, part1, memchr)]
// optimized version of part1_naive that uses memchr::memmem instead of iter::position
pub fn part1_memchr(input: &[u8]) -> u32 {
    let finder = memmem::Finder::new(b"mul(");
    sum_muls(input, &finder).0
}

#[aoc(day3, part1, backwards)]
// find instructions using jumps inspired by the Boyer–Moore algorithm
pub fn part1_backwards(mut input: &[u8]) -> u32 {
    const MINIMUM_SKIP: usize = 7;
    let mut sum = 0u32;
    loop {
        // no more room for instructions
        if input.len() < MINIMUM_SKIP {
            return sum;
        }
        match input[MINIMUM_SKIP..].iter().position(|&c| c == b')') {
            Some(i) => {
                // try to parse mul instruction
                let end_index = i + MINIMUM_SKIP;
                let (right_num, backwards_skip): (u16, usize) =
                    fast_parse_backwards(&input[..end_index]);
                // separating ','
                let mut read_head = end_index - backwards_skip - 1;
                if backwards_skip == 0 || input[read_head] != b',' {
                    input = &input[end_index + 1..];
                    continue;
                }
                let (left_num, backwards_skip): (u16, usize) =
                    fast_parse_backwards(&input[..read_head]);
                read_head -= backwards_skip;
                // verify its a mul
                if input[read_head - 4..read_head].eq(b"mul(") {
                    sum += left_num as u32 * right_num as u32;
                }
                input = &input[end_index + 1..];
            }
            None => return sum, // EOF
        }
    }
}

#[aoc(day3, part2, memchr)]
// parse in sections between do() and dont() instructions.
pub fn part2_memchr(mut input: &[u8]) -> u32 {
    const DO_SIZE: usize = 4;
    const DONT_SIZE: usize = 7;
    let mut sum = 0u32;
    let mul_finder = Finder::new("mul(");
    let do_finder = Finder::new("do()");
    let dont_finder = Finder::new("don't()");
    loop {
        match dont_finder.find(input) {
            Some(dont_idx) => {
                let (s, _) = sum_muls(&input[..dont_idx], &mul_finder);
                sum += s;
                let remainder = &input[DONT_SIZE + dont_idx..];
                match do_finder.find(remainder) {
                    Some(do_idx) => input = &remainder[DO_SIZE + do_idx..],
                    None => return sum,
                }
            }
            None => {
                let (s, _) = sum_muls(input, &mul_finder);
                sum += s;
                return sum;
            }
        }
    }
}

#[aoc(day3, part2, backwards)]
// find instructions using jumps inspired by the Boyer–Moore algorithm
pub fn part2_backwards(mut input: &[u8]) -> u32 {
    const MINIMUM_SKIP_DISABLED: usize = 3;
    const MINIMUM_SKIP_ENABLED: usize = 6;
    let mut sum = 0u32;
    let mut enabled = true;
    loop {
        // no more room for instructions
        if input.len() < MINIMUM_SKIP_ENABLED {
            return sum;
        }
        if enabled {
            // search for disable or mul instructions
            match input[MINIMUM_SKIP_ENABLED..]
                .iter()
                .position(|&c| c == b')')
            {
                Some(i) => {
                    let end_index: usize = i + MINIMUM_SKIP_ENABLED;
                    // check for disable instruction
                    if input[end_index - 6..end_index].eq(b"don't(") {
                        enabled = false;
                    } else {
                        // try to parse mul instruction
                        let (right_num, backwards_skip): (u16, usize) =
                            fast_parse_backwards(&input[..end_index]);
                        // separating ','
                        let mut read_head = end_index - backwards_skip - 1;
                        if backwards_skip == 0 || input[read_head] != b',' {
                            input = &input[end_index + 1..];
                            continue;
                        }
                        let (left_num, backwards_skip): (u16, usize) =
                            fast_parse_backwards(&input[..read_head]);
                        read_head -= backwards_skip;
                        // verify its a mul
                        if input[read_head - 4..read_head].eq(b"mul(") {
                            sum += left_num as u32 * right_num as u32;
                        }
                        input = &input[end_index + 1..];
                    }
                }
                None => return sum, // EOF
            }
        } else {
            // search for enable instructions only
            match input[MINIMUM_SKIP_DISABLED..]
                .iter()
                .position(|&c| c == b')')
            {
                Some(i) => {
                    let end_index = i + MINIMUM_SKIP_DISABLED;
                    if input[end_index - 3..end_index].eq(b"do(") {
                        enabled = true;
                    }
                    input = &input[end_index + 1..];
                }
                None => return sum, // EOF
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::{part1_memchr, part2_memchr};

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_memchr(
                b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            161
        )
    }

    #[test]
    fn sample_part2() {
        assert_eq!(
            part2_memchr(
                b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            48
        )
    }
}
