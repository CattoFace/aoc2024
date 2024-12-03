use aoc_runner_derive::aoc;

use crate::util::fast_parse_backwards;

#[aoc(day3, part1)]
pub fn part1(mut input: &[u8]) -> u32 {
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

#[aoc(day3, part2)]
pub fn part2(mut input: &[u8]) -> u32 {
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
    use crate::day3::{part1, part2};

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1(b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        )
    }

    #[test]
    fn sample_part2() {
        assert_eq!(
            part2(b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        )
    }
}
