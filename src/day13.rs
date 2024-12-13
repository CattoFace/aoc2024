use aoc_runner_derive::aoc;

use crate::util::fast_parse;

pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();
    part1_equation(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> i64 {
    let input = input.as_bytes();
    part2_equation(&input[..input.len() - 1])
}

fn find_cost64(a_x: i64, a_y: i64, b_x: i64, b_y: i64, x: i64, y: i64) -> Option<i64> {
    let a_presses_numerator = x * b_y - b_x * y;
    let b_presses_numerator = a_x * y - x * a_y;
    let divisor = a_x * b_y - b_x * a_y;
    if divisor == 0 {
        return None;
    }
    if a_presses_numerator % divisor == 0 && b_presses_numerator % divisor == 0 {
        Some(a_presses_numerator / divisor * 3 + b_presses_numerator / divisor)
    } else {
        None
    }
}

fn find_cost(a_x: i32, a_y: i32, b_x: i32, b_y: i32, x: i32, y: i32) -> Option<i32> {
    let a_presses_numerator = x * b_y - b_x * y;
    let b_presses_numerator = a_x * y - x * a_y;
    let divisor = a_x * b_y - b_x * a_y;
    if divisor == 0 {
        return None;
    }
    if a_presses_numerator % divisor == 0 && b_presses_numerator % divisor == 0 {
        Some(a_presses_numerator / divisor * 3 + b_presses_numerator / divisor)
    } else {
        None
    }
}

fn process_machine_far(input: &[u8]) -> (Option<i64>, &[u8]) {
    let a_x = ((input[12] - b'0') * 10 + (input[13] - b'0')) as i64;
    let a_y = ((input[18] - b'0') * 10 + (input[19] - b'0')) as i64;
    let b_x = ((input[33] - b'0') * 10 + (input[34] - b'0')) as i64;
    let b_y = ((input[39] - b'0') * 10 + (input[40] - b'0')) as i64;
    let (x, remainder) = fast_parse::<i32>(&input[51..]);
    let (y, remainder) = fast_parse::<i32>(&remainder[4..]);
    let far_x = x as i64 + 10000000000000;
    let far_y = y as i64 + 10000000000000;
    let next_machine = if remainder.is_empty() {
        remainder
    } else {
        &remainder[2..]
    };
    (find_cost64(a_x, a_y, b_x, b_y, far_x, far_y), next_machine)
}

fn process_machine(input: &[u8]) -> (Option<i32>, &[u8]) {
    let a_x = ((input[12] - b'0') * 10 + (input[13] - b'0')) as i32;
    let a_y = ((input[18] - b'0') * 10 + (input[19] - b'0')) as i32;
    let b_x = ((input[33] - b'0') * 10 + (input[34] - b'0')) as i32;
    let b_y = ((input[39] - b'0') * 10 + (input[40] - b'0')) as i32;
    let (x, remainder) = fast_parse(&input[51..]);
    let (y, remainder) = fast_parse(&remainder[4..]);
    let next_machine = if remainder.is_empty() {
        remainder
    } else {
        &remainder[2..]
    };
    (find_cost(a_x, a_y, b_x, b_y, x, y), next_machine)
}

#[aoc(day13, part1, equation)]
pub fn part1_equation(mut input: &[u8]) -> i32 {
    let mut sum = 0;
    while !input.is_empty() {
        let (machine_result, remainder) = process_machine(input);
        if let Some(price) = machine_result {
            sum += price;
        }
        input = remainder
    }
    sum
}

#[aoc(day13, part2, equation)]
pub fn part2_equation(mut input: &[u8]) -> i64 {
    let mut sum = 0;
    while !input.is_empty() {
        let (machine_result, remainder) = process_machine_far(input);
        if let Some(price) = machine_result {
            sum += price;
        }
        input = remainder
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day13::part1_equation;

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_equation(
                b"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=1027"
            ),
            480
        )
    }
}
