use std::{
    array::from_fn,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

use aoc_runner_derive::aoc;

use bitvec::prelude::*;

// real input is 131x130(including line break), example is 11x10
const WIDTH: usize = 131;
// const WIDTH: usize = 11;
const HEIGHT: usize = WIDTH - 1;
const TOTAL_SIZE: usize = WIDTH * HEIGHT;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[aoc(day6, part1, bitvec)]
pub fn part1_bitvec(input: &[u8]) -> u32 {
    let mut count = 1u32;
    let mut visited: BitArr!(for TOTAL_SIZE) = BitArray::ZERO;
    let mut location = input.iter().position(|&c| c == b'^').unwrap();
    let mut direction = Direction::Up;
    visited.set(location, true);
    loop {
        // the guard will either continue to new_location, or turn to new_direction
        let (new_location, new_direction) = match direction {
            Direction::Up => {
                if location <= WIDTH {
                    return count;
                }
                (location - WIDTH, Direction::Right)
            }
            Direction::Down => {
                if location >= TOTAL_SIZE - WIDTH {
                    return count;
                }
                (location + WIDTH, Direction::Left)
            }
            Direction::Left => {
                if location % WIDTH == 0 {
                    return count;
                }
                (location - 1, Direction::Up)
            }
            Direction::Right => {
                if location % WIDTH == WIDTH - 1 {
                    return count;
                }
                (location + 1, Direction::Down)
            }
        };
        if input[new_location] == b'#' {
            direction = new_direction;
        } else {
            location = new_location;
            let mut v = visited.get_mut(location).unwrap();
            count += !*v as u32;
            *v = true;
        }
    }
}

#[aoc(day6, part1, first)]
pub fn part1_first(input: &[u8]) -> u32 {
    let mut count = 1u32;
    let mut visited = [false; TOTAL_SIZE];
    let mut location = input.iter().position(|&c| c == b'^').unwrap();
    let mut direction = Direction::Up;
    visited[location] = true;
    loop {
        // the guard will either continue to new_location, or turn to new_direction
        let (new_location, new_direction) = match direction {
            Direction::Up => {
                if location <= WIDTH {
                    return count;
                }
                (location - WIDTH, Direction::Right)
            }
            Direction::Down => {
                if location >= TOTAL_SIZE - WIDTH {
                    return count;
                }
                (location + WIDTH, Direction::Left)
            }
            Direction::Left => {
                if location % WIDTH == 0 {
                    return count;
                }
                (location - 1, Direction::Up)
            }
            Direction::Right => {
                if location % WIDTH == WIDTH - 1 {
                    return count;
                }
                (location + 1, Direction::Down)
            }
        };
        if input[new_location] == b'#' {
            direction = new_direction;
        } else {
            location = new_location;
            if !visited[location] {
                visited[location] = true;
                count += 1;
            }
        }
    }
}

fn check_loop_pass_visited(
    input: &[u8],
    mut location: usize,
    mut direction: Direction,
    new_obstacle: usize,
    outer_visited: BitArr!(for TOTAL_SIZE*4),
) -> bool {
    let mut visited: BitArr!(for TOTAL_SIZE*4) = BitArray::ZERO;
    loop {
        let (new_location, new_direction) = match direction {
            Direction::Up => {
                if location <= WIDTH {
                    return false;
                }
                (location - WIDTH, Direction::Right)
            }
            Direction::Down => {
                if location >= TOTAL_SIZE - WIDTH {
                    return false;
                }
                (location + WIDTH, Direction::Left)
            }
            Direction::Left => {
                if location % WIDTH == 0 {
                    return false;
                }
                (location - 1, Direction::Up)
            }
            Direction::Right => {
                if location % WIDTH == WIDTH - 1 {
                    return false;
                }
                (location + 1, Direction::Down)
            }
        };
        if (new_location == new_obstacle) || (input[new_location] == b'#') {
            direction = new_direction;
        } else {
            location = new_location;
            if outer_visited[location * 4 + direction as usize] {
                return true;
            } else {
                let mut v = visited.get_mut(location * 4 + direction as usize).unwrap();
                if *v {
                    return true;
                } else {
                    *v = true;
                }
            }
        }
    }
}

fn check_loop(
    input: &[u8],
    mut location: usize,
    mut direction: Direction,
    new_obstacle: usize,
) -> bool {
    let mut visited: BitArr!(for TOTAL_SIZE*4) = BitArray::ZERO;
    loop {
        let (new_location, new_direction) = match direction {
            Direction::Up => {
                if location <= WIDTH {
                    return false;
                }
                (location - WIDTH, Direction::Right)
            }
            Direction::Down => {
                if location >= TOTAL_SIZE - WIDTH {
                    return false;
                }
                (location + WIDTH, Direction::Left)
            }
            Direction::Left => {
                if location % WIDTH == 0 {
                    return false;
                }
                (location - 1, Direction::Up)
            }
            Direction::Right => {
                if location % WIDTH == WIDTH - 1 {
                    return false;
                }
                (location + 1, Direction::Down)
            }
        };
        if (new_location == new_obstacle) || (input[new_location] == b'#') {
            direction = new_direction;
        } else {
            let mut v = visited
                .get_mut(new_location * 4 + direction as usize)
                .unwrap();
            if *v {
                return true;
            } else {
                location = new_location;
                *v = true;
            }
        }
    }
}

#[aoc(day6, part2, rayon)]
pub fn part2_rayon(input: &[u8]) -> u32 {
    let cant_place: &[AtomicBool; TOTAL_SIZE] = &from_fn(|_| AtomicBool::new(false));
    let count = &AtomicU32::new(0);
    let mut direction = Direction::Up;
    let mut location = input.iter().position(|&c| c == b'^').unwrap();
    let mut visited: BitArr!(for TOTAL_SIZE) = BitArray::ZERO;
    rayon::scope(|s| {
        visited.set(location, true);
        loop {
            let (new_location, new_direction) = match direction {
                Direction::Up => {
                    if location <= WIDTH {
                        break;
                    }
                    (location - WIDTH, Direction::Right)
                }
                Direction::Down => {
                    if location >= TOTAL_SIZE - WIDTH {
                        break;
                    }
                    (location + WIDTH, Direction::Left)
                }
                Direction::Left => {
                    if location % WIDTH == 0 {
                        break;
                    }
                    (location - 1, Direction::Up)
                }
                Direction::Right => {
                    if location % WIDTH == WIDTH - 1 {
                        break;
                    }
                    (location + 1, Direction::Down)
                }
            };
            if input[new_location] == b'#' {
                direction = new_direction;
            } else {
                location = new_location;
            }
            visited.set(location, true);
            if let Some(new_obstacle) = match direction {
                Direction::Up if location >= WIDTH => Some(location - WIDTH),
                Direction::Down if location < TOTAL_SIZE - WIDTH => Some(location + WIDTH),
                Direction::Left if location % WIDTH != 0 => Some(location - 1),
                Direction::Right if location % WIDTH != WIDTH - 1 => Some(location + 1),
                _ => None,
            } {
                if !visited[new_obstacle] && !cant_place[new_obstacle].load(Ordering::Relaxed) {
                    s.spawn(move |_| {
                        if check_loop(input, location, direction, new_obstacle)
                            && !cant_place[new_obstacle].swap(true, Ordering::Relaxed)
                        {
                            count.fetch_add(1, Ordering::Relaxed);
                        }
                    });
                }
            }
        }
    });
    count.load(Ordering::Relaxed)
}

#[aoc(day6, part2, bitvec)]
pub fn part2_bitvec(input: &[u8]) -> u32 {
    let mut cant_place: BitArr!(for TOTAL_SIZE) = BitArray::ZERO;
    let mut count = 0u32;
    let mut location = input.iter().position(|&c| c == b'^').unwrap();
    cant_place.set(location, true);
    let mut direction = Direction::Up;
    loop {
        let (new_location, new_direction) = match direction {
            Direction::Up => {
                if location <= WIDTH {
                    return count;
                }
                (location - WIDTH, Direction::Right)
            }
            Direction::Down => {
                if location >= TOTAL_SIZE - WIDTH {
                    return count;
                }
                (location + WIDTH, Direction::Left)
            }
            Direction::Left => {
                if location % WIDTH == 0 {
                    return count;
                }
                (location - 1, Direction::Up)
            }
            Direction::Right => {
                if location % WIDTH == WIDTH - 1 {
                    return count;
                }
                (location + 1, Direction::Down)
            }
        };
        if input[new_location] == b'#' {
            direction = new_direction;
        } else {
            location = new_location;
        }
        cant_place.set(location, true);
        if let Some(new_obstacle) = match direction {
            Direction::Up if location >= WIDTH => Some(location - WIDTH),
            Direction::Down if location < TOTAL_SIZE - WIDTH => Some(location + WIDTH),
            Direction::Left if location % WIDTH != 0 => Some(location - 1),
            Direction::Right if location % WIDTH != WIDTH - 1 => Some(location + 1),
            _ => None,
        } {
            if !cant_place[new_obstacle] && check_loop(input, location, direction, new_obstacle) {
                cant_place.set(new_obstacle, true);
                count += 1;
            }
        }
    }
}

#[aoc(day6, part2, pass_visited)]
pub fn part2_pass_visited(input: &[u8]) -> u32 {
    let mut placed: BitArr!(for TOTAL_SIZE) = BitArray::ZERO;
    let mut visited: BitArr!(for TOTAL_SIZE*4) = BitArray::ZERO;
    let mut count = 0u32;
    let mut location = input.iter().position(|&c| c == b'^').unwrap();
    let mut direction = Direction::Up;
    visited.set(location * 4 + direction as usize, true);
    loop {
        let (new_location, new_direction) = match direction {
            Direction::Up => {
                if location <= WIDTH {
                    return count;
                }
                (location - WIDTH, Direction::Right)
            }
            Direction::Down => {
                if location >= TOTAL_SIZE - WIDTH {
                    return count;
                }
                (location + WIDTH, Direction::Left)
            }
            Direction::Left => {
                if location % WIDTH == 0 {
                    return count;
                }
                (location - 1, Direction::Up)
            }
            Direction::Right => {
                if location % WIDTH == WIDTH - 1 {
                    return count;
                }
                (location + 1, Direction::Down)
            }
        };
        if input[new_location] == b'#' {
            direction = new_direction;
        } else {
            location = new_location;
        }
        visited.set(location * 4 + direction as usize, true);
        if let Some(new_obstacle) = match direction {
            Direction::Up if location >= WIDTH => Some(location - WIDTH),
            Direction::Down if location < TOTAL_SIZE - WIDTH => Some(location + WIDTH),
            Direction::Left if location % WIDTH != 0 => Some(location - 1),
            Direction::Right if location % WIDTH != WIDTH - 1 => Some(location + 1),
            _ => None,
        } {
            if visited[new_obstacle * 4..(new_obstacle + 1) * 4].not_any()
                && !placed[new_obstacle]
                && check_loop_pass_visited(input, location, direction, new_obstacle, visited)
            {
                placed.set(new_obstacle, true);
                count += 1;
            }
        }
    }
}

#[aoc(day6, part2, first)]
pub fn part2_first(input: &[u8]) -> u32 {
    let mut cant_place = [false; TOTAL_SIZE];
    let mut count = 0u32;
    let mut location = input.iter().position(|&c| c == b'^').unwrap();
    cant_place[location] = true;
    let mut direction = Direction::Up;
    loop {
        let (new_location, new_direction) = match direction {
            Direction::Up => {
                if location <= WIDTH {
                    return count;
                }
                (location - WIDTH, Direction::Right)
            }
            Direction::Down => {
                if location >= TOTAL_SIZE - WIDTH {
                    return count;
                }
                (location + WIDTH, Direction::Left)
            }
            Direction::Left => {
                if location % WIDTH == 0 {
                    return count;
                }
                (location - 1, Direction::Up)
            }
            Direction::Right => {
                if location % WIDTH == WIDTH - 1 {
                    return count;
                }
                (location + 1, Direction::Down)
            }
        };
        if input[new_location] == b'#' {
            direction = new_direction;
        } else {
            location = new_location;
        }
        cant_place[location] = true;
        if let Some(new_obstacle) = match direction {
            Direction::Up if location >= WIDTH => Some(location - WIDTH),
            Direction::Down if location < TOTAL_SIZE - WIDTH => Some(location + WIDTH),
            Direction::Left if location % WIDTH != 0 => Some(location - 1),
            Direction::Right if location % WIDTH != WIDTH - 1 => Some(location + 1),
            _ => None,
        } {
            if !cant_place[new_obstacle] && check_loop(input, location, direction, new_obstacle) {
                cant_place[new_obstacle] = true;
                count += 1;
            }
        }
    }
}
