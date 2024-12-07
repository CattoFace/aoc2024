use aoc_runner_derive::aoc;
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use tinyvec::ArrayVec;

use crate::util::fast_parse;

pub fn part1(input: &str) -> u64 {
    part1_mt(input.as_bytes())
}

pub fn part2(input: &str) -> u64 {
    part2_mt(input.as_bytes())
}
#[aoc(day7, part2, rem)]
pub fn part2_rem(mut input: &[u8]) -> u64 {
    let mut sum = 0u64;
    let mut buffer = ArrayVec::<[(u64, u64); 32]>::new();
    loop {
        let (total, remainder) = fast_parse::<u64>(input);
        // skip :
        input = &remainder[1..];
        while !input.is_empty() && input[0] != b'\n' {
            let (num, remainder) = fast_parse(&input[1..]);
            buffer.push((num, 10u64.pow(num.ilog10() + 1)));
            input = remainder;
        }
        if check_equation_recursive_part2_rem(total, &buffer) {
            sum += total;
        }
        if input.is_empty() {
            return sum;
        } else {
            input = &input[1..];
        }
        buffer.clear();
    }
}

#[aoc(day7, part2, mt)]
pub fn part2_mt(input: &[u8]) -> u64 {
    input
        .par_split(|&c| c == b'\n')
        .filter_map(|line| {
            let mut buffer = ArrayVec::<[(u64, u64); 16]>::new();
            let (total, mut remainder) = fast_parse::<u64>(line);
            // skip :
            remainder = &remainder[1..];
            while !remainder.is_empty() {
                let (num, r) = fast_parse(&remainder[1..]);
                buffer.push((num, 10u64.pow(num.ilog10() + 1)));
                remainder = r;
            }
            if check_equation_recursive_part2_rem(total, &buffer) {
                Some(total)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part1, mt)]
pub fn part1_mt(input: &[u8]) -> u64 {
    input
        .par_split(|&c| c == b'\n')
        .filter_map(|line| {
            let mut buffer = ArrayVec::<[u64; 16]>::new();
            let (total, mut remainder) = fast_parse::<u64>(line);
            // skip :
            remainder = &remainder[1..];
            while !remainder.is_empty() {
                let (num, r) = fast_parse(&remainder[1..]);
                buffer.push(num);
                remainder = r;
            }
            if check_equation_iterative_rem_own(total, buffer) {
                Some(total)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part1, rec)]
pub fn part1_rec(mut input: &[u8]) -> u64 {
    let mut sum = 0u64;
    let mut buffer = ArrayVec::<[u64; 16]>::new();
    loop {
        let (total, remainder) = fast_parse::<u64>(input);
        // skip :
        input = &remainder[1..];
        while !input.is_empty() && input[0] != b'\n' {
            let (num, remainder) = fast_parse(&input[1..]);
            buffer.push(num);
            input = remainder;
        }
        if check_equation_recursive(total, &buffer) {
            sum += total;
        }
        if input.is_empty() {
            return sum;
        } else {
            input = &input[1..];
        }
        buffer.clear();
    }
}

#[aoc(day7, part1, iter)]
pub fn part1_iter(mut input: &[u8]) -> u64 {
    let mut sum = 0u64;
    let mut buffer = ArrayVec::<[u64; 16]>::new();
    loop {
        let (total, remainder) = fast_parse::<u64>(input);
        // skip :
        input = &remainder[1..];
        while !input.is_empty() && input[0] != b'\n' {
            let (num, remainder) = fast_parse(&input[1..]);
            buffer.push(num);
            input = remainder;
        }
        if check_equation_iterative_rem(total, &buffer) {
            sum += total;
        }
        if input.is_empty() {
            return sum;
        } else {
            input = &input[1..];
        }
        buffer.clear();
    }
}

fn check_equation_iterative_rem_own(total: u64, buffer: ArrayVec<[u64; 16]>) -> bool {
    check_equation_iterative_rem(total, &buffer)
}

fn check_equation_iterative_rem(total: u64, buffer: &[u64]) -> bool {
    let mut queue = ArrayVec::<[(usize, u64); 16]>::new();
    // - buffer.len() for the 1 cases where mul is smaller
    let lower_bound: u64 = buffer.iter().sum::<u64>() - buffer.len() as u64;
    let upper_bound: u64 = buffer.iter().product();
    if (total < lower_bound) || (total > upper_bound) {
        return false;
    }
    let last = buffer[buffer.len() - 1];
    if total % last == 0 {
        queue.push((buffer.len() - 2, total / last));
    }
    queue.push((buffer.len() - 2, total - last));
    while !queue.is_empty() {
        let (i, remainder) = queue.pop().unwrap();
        if i == 0 {
            if remainder == buffer[0] {
                return true;
            }
        } else {
            let num = buffer[i];
            if remainder % num == 0 {
                queue.push((i - 1, remainder / num));
            }
            queue.push((i - 1, remainder - num));
        }
    }
    false
}

fn check_equation_recursive(remainder: u64, buffer: &[u64]) -> bool {
    check_equation_recursive_inner(remainder, buffer, buffer.len() - 1)
}

fn check_equation_recursive_inner(remainder: u64, buffer: &[u64], index: usize) -> bool {
    if index == 0 {
        remainder == buffer[0]
    } else {
        let num = buffer[index];
        check_equation_recursive_inner(remainder - num, buffer, index - 1)
            || (remainder % num == 0
                && check_equation_recursive_inner(remainder / num, buffer, index - 1))
    }
}

fn check_equation_recursive_part2_rem(remainder: u64, buffer: &[(u64, u64)]) -> bool {
    check_equation_recursive_inner_part2_rem(remainder, buffer, buffer.len() - 1)
}

fn check_equation_recursive_inner_part2_rem(
    remainder: u64,
    buffer: &[(u64, u64)],
    index: usize,
) -> bool {
    if index == 0 {
        remainder == buffer[0].0
    } else {
        let num = buffer[index];
        check_equation_recursive_inner_part2_rem(remainder - num.0, buffer, index - 1)
            || (remainder % num.0 == 0
                && check_equation_recursive_inner_part2_rem(remainder / num.0, buffer, index - 1))
            || (remainder % num.1 == num.0
                && check_equation_recursive_inner_part2_rem(remainder / num.1, buffer, index - 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::{part1_rec, part2_rem};

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_rec(
                b"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            3749
        )
    }

    #[test]
    fn sample_part2() {
        assert_eq!(
            part2_rem(
                b"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            11387
        )
    }
}
