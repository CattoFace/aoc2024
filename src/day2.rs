use aoc_runner_derive::aoc;
use tinyvec::{array_vec, ArrayVec};

use crate::util::fast_parse;

pub fn part1(input: &str) -> u16 {
    part1_no_vec(input.as_bytes())
}

pub fn part2(input: &str) -> u16 {
    part2_single_pass(input.as_bytes())
}

fn check_pair(a: u8, b: u8, increasing: bool) -> bool {
    a != b && (a <= b) == increasing && a.abs_diff(b) <= 3
}
fn check_line(nums: &[u8], increasing: bool) -> bool {
    nums.windows(2).all(|n| check_pair(n[0], n[1], increasing))
}

// gets an input to consume and a buffer to fill with the parsed numbers, and returns the remainder
// of the input
fn parse_line<'a>(mut input: &'a [u8], buffer: &mut ArrayVec<[u8; 8]>) -> &'a [u8] {
    while !input.is_empty() {
        let (num, remainder) = fast_parse::<u8>(input);
        buffer.push(num);
        // EOF
        if remainder.is_empty() {
            input = remainder;
            break;
        }
        // check linebreak before skipping whitespace
        if remainder[0] == b'\n' {
            input = &remainder[1..]; // skip whitespace
            break;
        }
        input = &remainder[1..]; // skip whitespace
    }
    input
}

#[aoc(day2, part1, naive)]
pub fn part1_naive(input: &str) -> u16 {
    input
        .lines()
        .filter(|&line| {
            let nums: Vec<u8> = line
                .split_whitespace()
                .map(|str_num| str_num.parse::<u8>().unwrap())
                .collect();
            let direction = nums[0] < nums[1];
            check_line(&nums, direction)
        })
        .count() as u16
}

#[aoc(day2, part1, opt)]
pub fn part1_opt(mut input: &[u8]) -> u16 {
    let mut sum = 0u16;
    let mut buffer = array_vec!([u8; 8]);
    while !input.is_empty() {
        input = parse_line(input, &mut buffer);
        let direction = buffer[0] <= buffer[1];
        if buffer[0] != buffer[1]
            && buffer[0].abs_diff(buffer[1]) <= 3
            && check_line(&buffer[1..], direction)
        {
            sum += 1;
        }
        buffer.clear();
    }
    sum
}

#[aoc(day2, part1, no_vec)]
pub fn part1_no_vec(mut input: &[u8]) -> u16 {
    let mut sum = 0u16;
    loop {
        let mut first: u8;
        let mut second: u8;
        (first, input) = fast_parse::<u8>(input);
        (second, input) = fast_parse::<u8>(&input[1..]);
        let direction = first <= second;
        // first pair check
        if first != second && first.abs_diff(second) <= 3 {
            loop {
                // step to next pair
                first = second;
                (second, input) = fast_parse::<u8>(&input[1..]);
                // number breaks rule, bad line
                if first == second || first.abs_diff(second) > 3 || (first <= second) != direction {
                    // skip to next line, or finish if EOF
                    let skip_index = input.iter().position(|&c| c == b'\n');
                    match skip_index {
                        // end of line, bad line
                        Some(i) => {
                            input = &input[i + 1..];
                            break;
                        }
                        // EOF, bad line
                        None => {
                            return sum;
                        }
                    }
                } else if input.is_empty() {
                    //EOF, good line
                    sum += 1;
                    return sum;
                } else if input[0] == b'\n' {
                    // End of line, good line
                    input = &input[1..];
                    sum += 1;
                    break;
                }
            }
        } else {
            // first pair failed, skip line
            let skip_index = input.iter().position(|&c| c == b'\n');
            match skip_index {
                // end of line, bad line
                Some(j) => {
                    input = &input[j + 1..];
                }
                // EOF, bad line
                None => {
                    return sum;
                }
            }
        }
    }
}
fn check_line_allow_mistake(nums: &[u8], increasing: bool) -> bool {
    let mut prev_prev_num = 0u8;
    let mut prev_num = nums[0];
    for i in 1..nums.len() {
        let next_num = nums[i];
        let allowed = check_pair(prev_num, next_num, increasing);
        // mistake found, check "no mistakes allowed" with removing next or prev
        if !allowed {
            return (check_pair(prev_prev_num, next_num, increasing) // remove prev
                && check_line(&nums[i..], increasing))
                || (i < nums.len() - 1 // remove next
                    && check_pair(prev_num, nums[i + 1], increasing)
                    && check_line(&nums[i + 1..], increasing))
                || i == nums.len() - 1; // mistake at last number
        }
        prev_prev_num = prev_num;
        prev_num = next_num;
    }
    true // no mistakes found
}
// checks line for part 2, includes direction calculation
fn check_line_single_pass(nums: &[u8]) -> bool {
    check_line_allow_mistake(nums, true)
        || check_line_allow_mistake(nums, false)
        || check_line(&nums[1..], true)
        || check_line(&nums[1..], false)
}

// checks line for part 2, includes direction calculation
fn check_line_bruteforce(nums: &[u8]) -> bool {
    let direction = nums[0] <= nums[1];
    // no mistakes
    if check_line(nums, direction) {
        true
    } else {
        // removed 0 case
        let direction0 = nums[1] <= nums[2];
        if check_line(&nums[1..], direction0) {
            return true;
        }
        // removed 1 case
        let direction1 = nums[0] <= nums[2];
        if nums[0] != nums[2]
            && nums[0].abs_diff(nums[2]) <= 3
            && check_line(&nums[2..], direction1)
        {
            return true;
        }
        // removed last case
        if check_line(&nums[..nums.len() - 1], direction) {
            return true;
        }
        // rest of the cases
        // split into 2 line checks and a pair checks across the removed number
        for removed in 2..nums.len() - 1 {
            if check_line(&nums[..removed], direction)
                && check_pair(nums[removed - 1], nums[removed + 1], direction)
                && check_line(&nums[removed + 1..], direction)
            {
                return true;
            }
        }
        false
    }
}

#[aoc(day2, part2, opt)]
pub fn part2_opt(mut input: &[u8]) -> u16 {
    let mut sum = 0u16;
    let mut buffer = array_vec!([u8; 8]);
    while !input.is_empty() {
        input = parse_line(input, &mut buffer);
        if check_line_bruteforce(&buffer) {
            sum += 1;
        }
        buffer.clear();
    }
    sum
}

#[aoc(day2, part2, single_pass)]
pub fn part2_single_pass(mut input: &[u8]) -> u16 {
    let mut sum = 0u16;
    let mut buffer = array_vec!([u8; 8]);
    while !input.is_empty() {
        input = parse_line(input, &mut buffer);
        if check_line_single_pass(&buffer) {
            sum += 1;
        }
        buffer.clear();
    }
    sum
}

#[aoc(day2, part2, naive)]
pub fn part2_naive(input: &str) -> u16 {
    input
        .lines()
        .filter(|&line| {
            let nums: Vec<u8> = line
                .split_whitespace()
                .map(|str_num| str_num.parse::<u8>().unwrap())
                .collect();
            check_line_bruteforce(&nums)
        })
        .count() as u16
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1_opt, part2_naive};

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_opt(
                b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            2
        )
    }

    #[test]
    fn sample_part2() {
        assert_eq!(
            part2_naive(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            4
        )
    }
}
