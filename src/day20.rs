use aoc_runner_derive::aoc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_first(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    part2_first(&input[..input.len() - 1])
}
fn compute_distances_to(
    input: &[u8],
    side_size: usize,
    end: usize,
    min_shortcut_size: u32,
) -> Vec<u32> {
    let mut distance_to = vec![u32::MAX - min_shortcut_size; input.len()];
    let mut queue = vec![end];
    let mut queue_next = Vec::<usize>::new();
    for step in 0u32.. {
        while let Some(pos) = queue.pop() {
            if distance_to[pos] <= step {
                continue;
            }
            distance_to[pos] = step;
            if input[pos - 1] != b'#' {
                queue_next.push(pos - 1);
            }
            if input[pos + 1] != b'#' {
                queue_next.push(pos + 1);
            }
            if input[pos - side_size] != b'#' {
                queue_next.push(pos - side_size);
            }
            if input[pos + side_size] != b'#' {
                queue_next.push(pos + side_size);
            }
        }
        if queue_next.is_empty() {
            return distance_to;
        }
        std::mem::swap(&mut queue, &mut queue_next);
    }
    unreachable!()
}

fn check_specific_shortcut_start(
    distance_to_end: &[u32],
    start_pos: usize,
    min_shortcut_size: u32,
    side_size: usize,
) -> u32 {
    let start_distance = distance_to_end[start_pos];
    let start_x = start_pos % side_size;
    let start_y = start_pos / side_size;
    let mut count = 0u32;
    for vertical_difference in -20i32..21 {
        let test_y = start_y as i32 + vertical_difference;
        if test_y > 0 && test_y < side_size as i32 - 1 {
            let max_horizontal_difference = 20 - vertical_difference.abs();
            for horizontal_difference in -max_horizontal_difference..max_horizontal_difference + 1 {
                let test_x = start_x as i32 + horizontal_difference;
                if test_x > 0
                    && test_x < side_size as i32 - 1
                    && distance_to_end[(test_y * side_size as i32 + test_x) as usize]
                        + min_shortcut_size
                        <= start_distance
                            - vertical_difference.unsigned_abs()
                            - horizontal_difference.unsigned_abs()
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_long_shortcuts_rayon(
    input: &[u8],
    distance_to_end: &[u32],
    start: usize,
    side_size: usize,
    min_shortcut_size: u32,
) -> u32 {
    let mut visited = vec![false; input.len()];
    visited[start] = true;
    let mut queue = vec![start];
    let mut queue_next = Vec::<usize>::new();
    let count = &AtomicU32::new(0);
    rayon::scope(|s| {
        for _step in 0u32..(distance_to_end[start] - min_shortcut_size) {
            while let Some(pos) = queue.pop() {
                s.spawn(move |_| {
                    count.fetch_add(
                        check_specific_shortcut_start(
                            distance_to_end,
                            pos,
                            min_shortcut_size,
                            side_size,
                        ),
                        Relaxed,
                    );
                });
                if !visited[pos - 1] && input[pos - 1] != b'#' {
                    visited[pos - 1] = true;
                    queue_next.push(pos - 1);
                }
                if !visited[pos + 1] && input[pos + 1] != b'#' {
                    visited[pos + 1] = true;
                    queue_next.push(pos + 1);
                }
                if !visited[pos - side_size] && input[pos - side_size] != b'#' {
                    visited[pos - side_size] = true;
                    queue_next.push(pos - side_size);
                }
                if !visited[pos + side_size] && input[pos + side_size] != b'#' {
                    visited[pos + side_size] = true;
                    queue_next.push(pos + side_size);
                }
            }
            std::mem::swap(&mut queue, &mut queue_next);
        }
    });
    count.load(Relaxed)
}

fn find_long_shortcuts(
    input: &[u8],
    distance_to_end: &[u32],
    start: usize,
    side_size: usize,
    min_shortcut_size: u32,
) -> u32 {
    let mut visited = vec![false; input.len()];
    visited[start] = true;
    let mut queue = vec![start];
    let mut queue_next = Vec::<usize>::new();
    let mut count = 0u32;
    for _step in 0u32..(distance_to_end[start] - min_shortcut_size) {
        while let Some(pos) = queue.pop() {
            count +=
                check_specific_shortcut_start(distance_to_end, pos, min_shortcut_size, side_size);
            if !visited[pos - 1] && input[pos - 1] != b'#' {
                visited[pos - 1] = true;
                queue_next.push(pos - 1);
            }
            if !visited[pos + 1] && input[pos + 1] != b'#' {
                visited[pos + 1] = true;
                queue_next.push(pos + 1);
            }
            if !visited[pos - side_size] && input[pos - side_size] != b'#' {
                visited[pos - side_size] = true;
                queue_next.push(pos - side_size);
            }
            if !visited[pos + side_size] && input[pos + side_size] != b'#' {
                visited[pos + side_size] = true;
                queue_next.push(pos + side_size);
            }
        }
        std::mem::swap(&mut queue, &mut queue_next);
    }
    count
}

fn find_shortcuts(
    input: &[u8],
    distance_to_end: &[u32],
    start: usize,
    side_size: usize,
    min_shortcut_size: u32,
) -> u32 {
    let mut visited = vec![false; input.len()];
    visited[start] = true;
    let mut queue = vec![start];
    let mut queue_next = Vec::<usize>::new();
    let mut count = 0u32;
    for _step in 0u32..(distance_to_end[start] - min_shortcut_size) {
        while let Some(pos) = queue.pop() {
            let curr_distance = distance_to_end[pos];
            // "peek" through walls
            if pos >= 2 * side_size
                && distance_to_end[pos - 2 * side_size] + min_shortcut_size <= curr_distance
            {
                count += 1;
            }
            if distance_to_end.len() >= pos + 2 * side_size
                && distance_to_end[pos + 2 * side_size] + min_shortcut_size <= curr_distance
            {
                count += 1;
            }
            if distance_to_end[pos - 2] + min_shortcut_size <= curr_distance {
                count += 1;
            }
            if distance_to_end[pos + 2] + min_shortcut_size <= curr_distance {
                count += 1;
            }
            // continue path finding
            if !visited[pos - 1] && input[pos - 1] != b'#' {
                visited[pos - 1] = true;
                queue_next.push(pos - 1);
            }
            if !visited[pos + 1] && input[pos + 1] != b'#' {
                visited[pos + 1] = true;
                queue_next.push(pos + 1);
            }
            if !visited[pos - side_size] && input[pos - side_size] != b'#' {
                visited[pos - side_size] = true;
                queue_next.push(pos - side_size);
            }
            if !visited[pos + side_size] && input[pos + side_size] != b'#' {
                visited[pos + side_size] = true;
                queue_next.push(pos + side_size);
            }
        }
        std::mem::swap(&mut queue, &mut queue_next);
    }
    count
}

#[aoc(day20, part2, rayon)]
pub fn part2_rayon(input: &[u8]) -> u32 {
    part2_rayon_inner(input, 100)
}

#[aoc(day20, part2)]
pub fn part2_first(input: &[u8]) -> u32 {
    part2_first_inner(input, 100)
}

#[aoc(day20, part1)]
pub fn part1_first(input: &[u8]) -> u32 {
    part1_first_inner(input, 100)
}

pub fn part2_rayon_inner(input: &[u8], min_shortcut_size: u32) -> u32 {
    let side_size = memchr::memchr(b'\n', input).unwrap() + 1;
    let start = memchr::memchr(b'S', input).unwrap();
    let end = memchr::memchr(b'E', input).unwrap();
    let distance_to_end = compute_distances_to(input, side_size, end, min_shortcut_size);
    find_long_shortcuts_rayon(input, &distance_to_end, start, side_size, min_shortcut_size)
}

pub fn part2_first_inner(input: &[u8], min_shortcut_size: u32) -> u32 {
    let side_size = memchr::memchr(b'\n', input).unwrap() + 1;
    let start = memchr::memchr(b'S', input).unwrap();
    let end = memchr::memchr(b'E', input).unwrap();
    let distance_to_end = compute_distances_to(input, side_size, end, min_shortcut_size);
    find_long_shortcuts(input, &distance_to_end, start, side_size, min_shortcut_size)
}

fn part1_first_inner(input: &[u8], min_shortcut_size: u32) -> u32 {
    let min_shortcut_size = min_shortcut_size + 2;
    let side_size = memchr::memchr(b'\n', input).unwrap() + 1;
    let start = memchr::memchr(b'S', input).unwrap();
    let end = memchr::memchr(b'E', input).unwrap();
    let distance_to_end = compute_distances_to(input, side_size, end, min_shortcut_size);
    find_shortcuts(input, &distance_to_end, start, side_size, min_shortcut_size)
}

#[cfg(test)]
mod tests {
    use crate::day20::{part1_first_inner, part2_first_inner};
    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_first_inner(
                b"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
                10
            ),
            10
        );
    }
    #[test]
    fn sample_part2() {
        assert_eq!(
            part2_first_inner(
                b"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
                50
            ),
            285
        );
    }
}
