use aoc_runner_derive::aoc;

use crate::util::fast_parse;

// const SIZE: usize = 7;
const SIZE: usize = 71;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_first(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> String {
    let input = input.as_bytes();
    part2_better(&input[..input.len() - 1])
}
fn can_reach_end(map: [bool; SIZE * SIZE]) -> bool {
    let mut queue = Vec::new();
    let mut queue_next = Vec::new();
    queue.push((0usize, 0usize, 0usize));
    let mut visited = [false; SIZE * SIZE];
    visited[0] = true;
    loop {
        while let Some((pos, pos_x, pos_y)) = queue.pop() {
            if pos == SIZE * SIZE - 1 {
                return true;
            }
            if pos_x > 0 && !map[pos - 1] && !visited[pos - 1] {
                visited[pos - 1] = true;
                queue_next.push((pos - 1, pos_x - 1, pos_y));
            }
            if pos_x < SIZE - 1 && !map[pos + 1] && !visited[pos + 1] {
                visited[pos + 1] = true;
                queue_next.push((pos + 1, pos_x + 1, pos_y));
            }
            if pos_y > 0 && !map[pos - SIZE] && !visited[pos - SIZE] {
                visited[pos - SIZE] = true;
                queue_next.push((pos - SIZE, pos_x, pos_y - 1));
            }
            if pos_y < SIZE - 1 && !map[pos + SIZE] && !visited[pos + SIZE] {
                visited[pos + SIZE] = true;
                queue_next.push((pos + SIZE, pos_x, pos_y + 1));
            }
        }
        if queue_next.is_empty() {
            return false;
        }
        std::mem::swap(&mut queue, &mut queue_next);
    }
}

#[aoc(day18, part2, better)]
pub fn part2_better(input: &[u8]) -> String {
    let (x, y) = part2_better_inner(input);
    format!("{x},{y}")
}

#[aoc(day18, part2)]
pub fn part2_first(input: &[u8]) -> String {
    let (x, y) = part2_first_inner(input);
    format!("{x},{y}")
}

fn part2_better_inner(mut input: &[u8]) -> (usize, usize) {
    let mut map = [u32::MAX; SIZE * SIZE];
    let mut reachable_with = [1024u32; SIZE * SIZE];
    let mut order = Vec::new();
    for i in 0u32.. {
        let (x, rem) = fast_parse::<usize>(input);
        let (y, rem) = fast_parse::<usize>(&rem[1..]);
        map[y * SIZE + x] = i;
        order.push((x, y));
        if rem.is_empty() {
            break;
        }
        input = &rem[1..];
    }
    let mut max_reachable = 1024u32;
    let mut queue = Vec::new();
    queue.push((0usize, 0usize, 0usize, u32::MAX));
    while let Some((pos, pos_x, pos_y, min_on_path)) = queue.pop() {
        if pos == SIZE * SIZE - 1 {
            max_reachable = max_reachable.max(min_on_path);
            continue;
        }
        if pos_x > 0 && min_on_path > reachable_with[pos - 1] {
            reachable_with[pos - 1] = min_on_path;
            queue.push((pos - 1, pos_x - 1, pos_y, min_on_path.min(map[pos - 1])));
        }
        if pos_x < SIZE - 1 && min_on_path > reachable_with[pos + 1] {
            reachable_with[pos + 1] = min_on_path;
            queue.push((pos + 1, pos_x + 1, pos_y, min_on_path.min(map[pos + 1])));
        }
        if pos_y > 0 && min_on_path > reachable_with[pos - SIZE] {
            reachable_with[pos - SIZE] = min_on_path;
            queue.push((
                pos - SIZE,
                pos_x,
                pos_y - 1,
                min_on_path.min(map[pos - SIZE]),
            ));
        }
        if pos_y < SIZE - 1 && min_on_path > reachable_with[pos + SIZE] {
            reachable_with[pos + SIZE] = min_on_path;
            queue.push((
                pos + SIZE,
                pos_x,
                pos_y + 1,
                min_on_path.min(map[pos + SIZE]),
            ));
        }
    }
    order[max_reachable as usize]
}

fn part2_first_inner(mut input: &[u8]) -> (usize, usize) {
    let mut map = [false; SIZE * SIZE];
    let mut x = 0;
    let mut y = 0;
    for _ in 0..1024 {
        (x, input) = fast_parse::<usize>(input);
        (y, input) = fast_parse::<usize>(&input[1..]);
        map[y * SIZE + x] = true;
        input = &input[1..];
    }
    while can_reach_end(map) {
        (x, input) = fast_parse::<usize>(input);
        (y, input) = fast_parse::<usize>(&input[1..]);
        map[y * SIZE + x] = true;
        input = &input[1..];
    }
    (x, y)
}

#[aoc(day18, part1)]
pub fn part1_first(mut input: &[u8]) -> u32 {
    let mut map = [false; SIZE * SIZE];
    for _ in 0..1024 {
        let (x, rem) = fast_parse::<usize>(input);
        let (y, rem) = fast_parse::<usize>(&rem[1..]);
        map[y * SIZE + x] = true;
        input = &rem[1..];
    }
    let mut queue = Vec::new();
    let mut queue_next = Vec::new();
    queue.push((0usize, 0usize, 0usize));
    map[0] = true;
    let mut curr_distance = 0;
    loop {
        while let Some((pos, pos_x, pos_y)) = queue.pop() {
            if pos == SIZE * SIZE - 1 {
                return curr_distance;
            }
            if pos_x > 0 && !map[pos - 1] {
                map[pos - 1] = true;
                queue_next.push((pos - 1, pos_x - 1, pos_y));
            }
            if pos_x < SIZE - 1 && !map[pos + 1] {
                map[pos + 1] = true;
                queue_next.push((pos + 1, pos_x + 1, pos_y));
            }
            if pos_y > 0 && !map[pos - SIZE] {
                map[pos - SIZE] = true;
                queue_next.push((pos - SIZE, pos_x, pos_y - 1));
            }
            if pos_y < SIZE - 1 && !map[pos + SIZE] {
                map[pos + SIZE] = true;
                queue_next.push((pos + SIZE, pos_x, pos_y + 1));
            }
        }
        curr_distance += 1;
        std::mem::swap(&mut queue, &mut queue_next);
    }
}
