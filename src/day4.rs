use std::thread::{self, ScopedJoinHandle};

use aoc_runner_derive::aoc;

fn find_surrounding_mas(input: &[u8], i: usize, line_len: usize) -> u32 {
    // LEFT
    (i>=3 && &input[i-3..i]==b"SAM") as u32+
    // RIGHT
    (i<=input.len()-3 && &input[i+1..i+4]==b"MAS") as u32+
    // UP
    (i>=3*line_len
        && input[i-3*line_len] == b'S'
        && input[i-2*line_len] == b'A'
        && input[i-line_len] == b'M') as u32 +
    // UP+RIGHT
    (i+3>=3*line_len
    && input[i+3-3*line_len] == b'S'
    && input[i+2-2*line_len] == b'A'
    && input[i+1-line_len] == b'M') as u32 +
    // UP+LEFT
    (i>=3*line_len+3
        && input[i-3*line_len-3] == b'S'
        && input[i-2*line_len-2] == b'A'
        && input[i-line_len-1] == b'M') as u32 +
    //DOWN
    (i+3*line_len<input.len()
        && input[i+3*line_len] == b'S'
        && input[i+2*line_len] == b'A'
        && input[i+line_len] == b'M') as u32 +
    //DOWN+RIGHT
    (i+3*line_len+3<input.len()
        && input[i+3*line_len+3] == b'S'
        && input[i+2*line_len+2] == b'A'
        && input[i+line_len+1] == b'M') as u32 +
    // DOWN+LEFT
    (i+3*line_len-3<input.len()
        && input[i+3*line_len-3] == b'S'
        && input[i+2*line_len-2] == b'A'
        && input[i+line_len-1] == b'M') as u32
}

fn is_x(input: &[u8], i: usize, line_len: usize) -> bool {
    // UPLEFT+DOWNRIGHT
    ((input.get(i - line_len - 1) == Some(&b'M') && input.get(i + line_len + 1) == Some(&b'S'))
        || (input.get(i - line_len - 1) == Some(&b'S') && input.get(i + line_len + 1) == Some(&b'M'))) &&
    // DOWNLEFT+UPRIGHT
    ((input.get(i + line_len - 1) == Some(&b'M') && input.get(i - line_len + 1) == Some(&b'S'))
        || (input.get(i + line_len - 1) == Some(&b'S') && input.get(i - line_len + 1) == Some(&b'M')))
}

pub fn part1(input: &str) -> u32 {
    part1_naive(input.as_bytes())
}

pub fn part2(input: &str) -> u32 {
    part2_naive(input.as_bytes())
}

#[aoc(day4, part1, mt)]
pub fn part1_mt(input: &[u8]) -> u32 {
    const THREAD_COUNT: usize = 2usize;
    let line_len = memchr::memchr(b'\n', input).unwrap() + 1;
    let chunk_size = input.len() / THREAD_COUNT;
    thread::scope(|s| {
        let threads: Vec<ScopedJoinHandle<u32>> = (0..THREAD_COUNT)
            .map(|tid| s.spawn(move || part1_chunk(input, line_len, tid, chunk_size)))
            .collect();
        let local_res = memchr::memchr_iter(b'X', &input[THREAD_COUNT * chunk_size..])
            .map(|i| find_surrounding_mas(input, i + THREAD_COUNT * chunk_size, line_len))
            .sum::<u32>();
        local_res + threads.into_iter().map(|t| t.join().unwrap()).sum::<u32>()
    })
}
fn part1_chunk(input: &[u8], line_len: usize, tid: usize, chunk_size: usize) -> u32 {
    memchr::memchr_iter(b'X', &input[tid * chunk_size..(tid + 1) * chunk_size])
        .map(|i| find_surrounding_mas(input, i + tid * chunk_size, line_len))
        .sum::<u32>()
}

#[aoc(day4, part1, naive)]
pub fn part1_naive(input: &[u8]) -> u32 {
    let line_len = memchr::memchr(b'\n', input).unwrap() + 1;
    memchr::memchr_iter(b'X', input)
        .map(|i| find_surrounding_mas(input, i, line_len))
        .sum::<u32>()
}

#[aoc(day4, part2, naive)]
pub fn part2_naive(input: &[u8]) -> u32 {
    let line_len = memchr::memchr(b'\n', input).unwrap() + 1;
    // no point searching in the first and last line
    // there's also no point searching the first and last column but that's not worth the effort to skip
    memchr::memchr_iter(b'A', &input[line_len..input.len() - line_len])
        .filter(|&i| is_x(input, i + line_len, line_len))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day4::{part1_naive, part2_naive};

    #[test]
    fn sample_part1_naive() {
        assert_eq!(
            part1_naive(
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
            18
        )
    }

    #[test]
    fn sample_part2_naive() {
        assert_eq!(
            part2_naive(
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
