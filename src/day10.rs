use bitvec::prelude::*;
use memchr;

use aoc_runner_derive::aoc;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_buffer(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    part2_buffer(&input[..input.len() - 1])
}

fn find_trailhead_rating_buffer(
    input: &[u8],
    start: usize,
    width: usize,
    height: usize,
    queue: &mut Vec<(usize, usize, usize, u8)>,
) -> u32 {
    let mut score = 0u32;
    queue.push((start, start % width, start / width, b'0'));
    while let Some((curr, curr_x, curr_y, depth)) = queue.pop() {
        // trail end reached
        if depth == b'9' {
            score += 1;
            continue;
        }
        if curr_x > 0 && input[curr - 1] == depth + 1 {
            queue.push((curr - 1, curr_x - 1, curr_y, depth + 1));
        }
        if curr_x < width - 2 && input[curr + 1] == depth + 1 {
            queue.push((curr + 1, curr_x + 1, curr_y, depth + 1));
        }
        if curr_y > 0 && input[curr - width] == depth + 1 {
            queue.push((curr - width, curr_x, curr_y - 1, depth + 1));
        }
        if curr_y < height && input[curr + width] == depth + 1 {
            queue.push((curr + width, curr_x, curr_y + 1, depth + 1));
        }
    }
    score
}

fn find_trailhead_rating(input: &[u8], start: usize, width: usize, height: usize) -> u32 {
    let mut queue = Vec::new();
    let mut score = 0u32;
    queue.push((start, start % width, start / width, b'0'));
    while let Some((curr, curr_x, curr_y, depth)) = queue.pop() {
        // trail end reached
        if depth == b'9' {
            score += 1;
            continue;
        }
        if curr_x > 0 && input[curr - 1] == depth + 1 {
            queue.push((curr - 1, curr_x - 1, curr_y, depth + 1));
        }
        if curr_x < width - 2 && input[curr + 1] == depth + 1 {
            queue.push((curr + 1, curr_x + 1, curr_y, depth + 1));
        }
        if curr_y > 0 && input[curr - width] == depth + 1 {
            queue.push((curr - width, curr_x, curr_y - 1, depth + 1));
        }
        if curr_y < height && input[curr + width] == depth + 1 {
            queue.push((curr + width, curr_x, curr_y + 1, depth + 1));
        }
    }
    score
}

fn find_trailhead_score_buffer(
    input: &[u8],
    start: usize,
    width: usize,
    height: usize,
    queue: &mut Vec<(usize, usize, usize, u8)>,
) -> u32 {
    let mut visited = bitvec![0; input.len()];
    let mut score = 0u32;
    queue.push((start, start % width, start / width, b'0'));
    visited.set(start, true);
    while let Some((curr, curr_x, curr_y, depth)) = queue.pop() {
        // trail end reached
        if depth == b'9' {
            score += 1;
            continue;
        }
        if curr_x > 0 && input[curr - 1] == depth + 1 && !visited[curr - 1] {
            visited.set(curr - 1, true);
            queue.push((curr - 1, curr_x - 1, curr_y, depth + 1));
        }
        if curr_x < width - 2 && input[curr + 1] == depth + 1 && !visited[curr + 1] {
            visited.set(curr + 1, true);
            queue.push((curr + 1, curr_x + 1, curr_y, depth + 1));
        }
        if curr_y > 0 && input[curr - width] == depth + 1 && !visited[curr - width] {
            visited.set(curr - width, true);
            queue.push((curr - width, curr_x, curr_y - 1, depth + 1));
        }
        if curr_y < height && input[curr + width] == depth + 1 && !visited[curr + width] {
            visited.set(curr + width, true);
            queue.push((curr + width, curr_x, curr_y + 1, depth + 1));
        }
    }
    score
}

fn find_trailhead_score(input: &[u8], start: usize, width: usize, height: usize) -> u32 {
    let mut queue = Vec::new();
    let mut visited = bitvec![0; input.len()];
    let mut score = 0u32;
    queue.push((start, start % width, start / width, b'0'));
    visited.set(start, true);
    while let Some((curr, curr_x, curr_y, depth)) = queue.pop() {
        // trail end reached
        if depth == b'9' {
            score += 1;
            continue;
        }
        if curr_x > 0 && input[curr - 1] == depth + 1 && !visited[curr - 1] {
            visited.set(curr - 1, true);
            queue.push((curr - 1, curr_x - 1, curr_y, depth + 1));
        }
        if curr_x < width - 2 && input[curr + 1] == depth + 1 && !visited[curr + 1] {
            visited.set(curr + 1, true);
            queue.push((curr + 1, curr_x + 1, curr_y, depth + 1));
        }
        if curr_y > 0 && input[curr - width] == depth + 1 && !visited[curr - width] {
            visited.set(curr - width, true);
            queue.push((curr - width, curr_x, curr_y - 1, depth + 1));
        }
        if curr_y < height && input[curr + width] == depth + 1 && !visited[curr + width] {
            visited.set(curr + width, true);
            queue.push((curr + width, curr_x, curr_y + 1, depth + 1));
        }
    }
    score
}

#[aoc(day10, part1, buffer)]
pub fn part1_buffer(input: &[u8]) -> u32 {
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = input.len() / width;
    let mut buffer = Vec::new();
    memchr::memrchr_iter(b'0', input)
        .map(|start| find_trailhead_score_buffer(input, start, width, height, &mut buffer))
        .sum()
}

#[aoc(day10, part1, first)]
pub fn part1_first(input: &[u8]) -> u32 {
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = input.len() / width;
    memchr::memrchr_iter(b'0', input)
        .map(|start| find_trailhead_score(input, start, width, height))
        .sum()
}

#[aoc(day10, part2, buffer)]
pub fn part2_buffer(input: &[u8]) -> u32 {
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = input.len() / width;
    let mut buffer = Vec::new();
    memchr::memrchr_iter(b'0', input)
        .map(|start| find_trailhead_rating_buffer(input, start, width, height, &mut buffer))
        .sum()
}

#[aoc(day10, part2, first)]
pub fn part2_first(input: &[u8]) -> u32 {
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = input.len() / width;
    memchr::memrchr_iter(b'0', input)
        .map(|start| find_trailhead_rating(input, start, width, height))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day10::{part1_buffer, part2_buffer};

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_buffer(
                b"0123
1234
8765
9876"
            ),
            1
        )
    }

    #[test]
    fn sample2_part1() {
        assert_eq!(
            part1_buffer(
                b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            36
        )
    }

    #[test]
    fn sample2_part2() {
        assert_eq!(
            part2_buffer(
                b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            81
        )
    }
}
