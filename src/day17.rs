use aoc_runner_derive::aoc;

use crate::util::fast_parse;

#[aoc(day17, part1)]
pub fn part1_first(input: &[u8]) -> String {
    String::from_utf8(part1_first_inner(input)).unwrap()
}

#[aoc(day17, part1, opt)]
pub fn part1_opt(input: &[u8]) -> String {
    String::from_utf8(part1_opt_inner(input)).unwrap()
}
pub fn part1_opt_inner(input: &[u8]) -> Vec<u8> {
    // setup
    let (reg_a, remainder) = fast_parse(&input[12..]);
    let mut output = Vec::new();
    let mut memory = [0usize, 1, 2, 3, reg_a, 0, 0];
    let instructions: Vec<usize> = remainder[39..]
        .iter()
        .filter_map(|&c| {
            if c != b',' {
                Some((c - b'0') as usize)
            } else {
                None
            }
        })
        .collect();
    while memory[4] != 0 {
        for &[opcode, operand] in instructions[..instructions.len() - 2].array_chunks() {
            match opcode {
                0 => memory[4] >>= memory[operand],
                1 => memory[5] ^= operand,
                2 => memory[5] = memory[operand] % 8,
                4 => memory[5] ^= memory[6],
                5 => {
                    output.push((memory[operand] % 8) as u8 + b'0');
                    output.push(b',');
                }
                6 => memory[5] = memory[4] >> memory[operand],
                7 => memory[6] = memory[4] >> memory[operand],
                _ => unreachable!("invalid program"),
            }
        }
    }
    output.pop(); // don't want last comma
    output
}

pub fn part1_first_inner(input: &[u8]) -> Vec<u8> {
    // setup
    let (reg_a, remainder) = fast_parse(&input[12..]);
    let mut output = Vec::new();
    let mut memory = [0usize, 1, 2, 3, reg_a, 0, 0];
    let instructions: Vec<usize> = remainder[39..]
        .iter()
        .filter_map(|&c| {
            if c != b',' {
                Some((c - b'0') as usize)
            } else {
                None
            }
        })
        .collect();
    let mut instruction_pointer = 0usize;
    // execution loop
    while instruction_pointer < instructions.len() {
        instruction_pointer = execute_instruction(
            instruction_pointer,
            instructions[instruction_pointer],
            instructions[instruction_pointer + 1],
            &mut memory,
            &mut output,
        );
    }
    output.pop(); // don't want last comma
    output
}

fn execute_instruction(
    instruction_pointer: usize,
    opcode: usize,
    operand: usize,
    memory: &mut [usize; 7],
    output: &mut Vec<u8>,
) -> usize {
    match opcode {
        0 => memory[4] >>= memory[operand],
        1 => memory[5] ^= operand,
        2 => memory[5] = memory[operand] % 8,
        3 => {
            if memory[4] != 0 {
                return operand;
            }
        }
        4 => memory[5] ^= memory[6],
        5 => {
            output.push((memory[operand] % 8) as u8 + b'0');
            output.push(b',');
        }
        6 => memory[5] = memory[4] >> memory[operand],
        7 => memory[6] = memory[4] >> memory[operand],
        _ => unreachable!("invalid program"),
    }
    instruction_pointer + 2
}

const TABLE_SIZE: usize = 2 << 11;

#[aoc(day17, part2, no_table)]
pub fn part2_no_table(input: &[u8]) -> usize {
    // setup
    let remainder = input.iter().position(|&c| c == b'\n').unwrap();
    let instructions: Vec<usize> = input[remainder + 39..]
        .iter()
        .filter_map(|&c| {
            if c != b',' {
                Some((c - b'0') as usize)
            } else {
                None
            }
        })
        .collect();
    let mut possible_a: Vec<_> = (0..TABLE_SIZE)
        .filter(|&a| {
            let mut memory = [0usize, 1, 2, 3, a, 0, 0];
            for &[opcode, operand] in instructions[..instructions.len() - 2].array_chunks() {
                match opcode {
                    0 => memory[4] >>= memory[operand],
                    1 => memory[5] ^= operand,
                    2 => memory[5] = memory[operand] % 8,
                    4 => memory[5] ^= memory[6],
                    5 => return memory[operand] % 8 == instructions[instructions.len() - 1],
                    6 => memory[5] = memory[4] >> memory[operand],
                    7 => memory[6] = memory[4] >> memory[operand],
                    _ => unreachable!("invalid program"),
                }
            }
            unreachable!()
        })
        .collect();
    for &to_output in instructions[..instructions.len() - 1].iter().rev() {
        possible_a = possible_a
            .into_iter()
            .flat_map(|full_a| {
                (full_a * 8..(full_a + 1) * 8).filter(|&a| {
                    let mut memory = [0usize, 1, 2, 3, a, 0, 0];
                    for &[opcode, operand] in instructions[..instructions.len() - 2].array_chunks()
                    {
                        match opcode {
                            0 => memory[4] >>= memory[operand],
                            1 => memory[5] ^= operand,
                            2 => memory[5] = memory[operand] % 8,
                            4 => memory[5] ^= memory[6],
                            5 => return memory[operand] % 8 == to_output,
                            6 => memory[5] = memory[4] >> memory[operand],
                            7 => memory[6] = memory[4] >> memory[operand],
                            _ => unreachable!("invalid program"),
                        }
                    }
                    unreachable!()
                })
            })
            .collect();
    }
    possible_a[0]
}

#[aoc(day17, part2, table)]
pub fn part2_table(input: &[u8]) -> usize {
    // setup
    let remainder = input.iter().position(|&c| c == b'\n').unwrap();
    let instructions: Vec<usize> = input[remainder + 39..]
        .iter()
        .filter_map(|&c| {
            if c != b',' {
                Some((c - b'0') as usize)
            } else {
                None
            }
        })
        .collect();
    let output_table: Vec<_> = (0usize..TABLE_SIZE)
        .map(|a| {
            let mut memory = [0usize, 1, 2, 3, a, 0, 0];
            for &[opcode, operand] in instructions[..instructions.len() - 2].array_chunks() {
                match opcode {
                    0 => memory[4] >>= memory[operand],
                    1 => memory[5] ^= operand,
                    2 => memory[5] = memory[operand] % 8,
                    4 => memory[5] ^= memory[6],
                    5 => return memory[operand] % 8,
                    6 => memory[5] = memory[4] >> memory[operand],
                    7 => memory[6] = memory[4] >> memory[operand],
                    _ => unreachable!("invalid program"),
                }
            }
            unreachable!()
        })
        .collect();
    let mut possible_a: Vec<_> = output_table
        .iter()
        .enumerate()
        .filter_map(|(a, &out)| {
            if out == instructions[instructions.len() - 1] {
                Some(a)
            } else {
                None
            }
        })
        .collect();
    for &to_output in instructions[..instructions.len() - 1].iter().rev() {
        possible_a = possible_a
            .into_iter()
            .flat_map(|full_a| {
                let prev_low_a = full_a % (TABLE_SIZE / 8);
                output_table[prev_low_a * 8..(prev_low_a + 1) * 8]
                    .iter()
                    .enumerate()
                    .filter_map(move |(new_a, &output)| {
                        if output == to_output {
                            Some(full_a * 8 + new_a % 8)
                        } else {
                            None
                        }
                    })
            })
            .collect();
    }
    possible_a[0]
}
