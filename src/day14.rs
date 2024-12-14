use std::cmp::Ordering;

use aoc_runner_derive::aoc;

use crate::util::{fast_parse, fast_parsei};

enum Quadrant {
    UpLeft = 0,
    UpRight = 1,
    DownLeft = 2,
    DownRight = 3,
    Middle = 4,
}

struct Robot {
    p_x: i32,
    p_y: i32,
    v_x: i32,
    v_y: i32,
}

const WIDTH: i32 = 101;
const TREE_HEIGHT: u32 = 33;
const HEIGHT: i32 = 103;
const TREE_WIDTH: u32 = 31;
const HALF_WIDTH: i32 = WIDTH / 2;
const HALF_HEIGHT: i32 = HEIGHT / 2;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_rem(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> i32 {
    let input = input.as_bytes();
    part2_hough(&input[..input.len() - 1])
}

fn parse_robot(input: &[u8]) -> (Robot, &[u8]) {
    let (p_x, remainder) = fast_parse::<i32>(&input[2..]);
    let (p_y, remainder) = fast_parse::<i32>(&remainder[1..]);
    let (v_x, remainder) = fast_parsei::<i32>(&remainder[3..]);
    let (v_y, remainder) = fast_parsei::<i32>(&remainder[1..]);
    (Robot { p_x, p_y, v_x, v_y }, remainder)
}

fn parse_and_move_robot(input: &[u8]) -> (Quadrant, &[u8]) {
    let (p_x, remainder) = fast_parse::<i32>(&input[2..]);
    let (p_y, remainder) = fast_parse::<i32>(&remainder[1..]);
    let (v_x, remainder) = fast_parsei::<i32>(&remainder[3..]);
    let (v_y, remainder) = fast_parsei::<i32>(&remainder[1..]);
    let final_x = (p_x + v_x * 100).rem_euclid(WIDTH);
    let final_y = (p_y + v_y * 100).rem_euclid(HEIGHT);
    (
        match (final_x.cmp(&HALF_WIDTH), final_y.cmp(&HALF_HEIGHT)) {
            (Ordering::Less, Ordering::Less) => Quadrant::UpLeft,
            (Ordering::Less, Ordering::Greater) => Quadrant::DownLeft,
            (Ordering::Greater, Ordering::Less) => Quadrant::UpRight,
            (Ordering::Greater, Ordering::Greater) => Quadrant::DownRight,
            (_, Ordering::Equal) | (Ordering::Equal, _) => Quadrant::Middle,
        },
        remainder,
    )
}

#[aoc(day14, part2, hough)]
pub fn part2_hough(mut input: &[u8]) -> i32 {
    let mut robots = Vec::new();
    loop {
        let (robot, remainder) = parse_robot(input);
        robots.push(robot);
        if remainder.is_empty() {
            break;
        }
        input = &remainder[1..];
    }
    for step in 0i32.. {
        let mut vertical_lines = [0u32; WIDTH as usize];
        let mut horizontal_lines = [0u32; HEIGHT as usize];
        robots.iter().for_each(|robot| {
            vertical_lines[(robot.p_x + robot.v_x * step).rem_euclid(WIDTH) as usize] += 1;
            horizontal_lines[(robot.p_y + robot.v_y * step).rem_euclid(HEIGHT) as usize] += 1;
        });
        if vertical_lines
            .iter()
            .filter(|&&line| line >= TREE_HEIGHT)
            .count()
            >= 2
            && horizontal_lines
                .iter()
                .filter(|&&line| line >= TREE_WIDTH)
                .count()
                >= 2
        {
            return step;
        }
    }
    unreachable!("The loop never breaks")
}

#[aoc(day14, part1, rem)]
pub fn part1_rem(mut input: &[u8]) -> u32 {
    let mut robots_at_quadrants = [0u32; 5]; // 5th for middle area, unused
    loop {
        let (quadrant, remainder) = parse_and_move_robot(input);
        robots_at_quadrants[quadrant as usize] += 1;
        if remainder.is_empty() {
            return robots_at_quadrants[0]
                * robots_at_quadrants[1]
                * robots_at_quadrants[2]
                * robots_at_quadrants[3];
        } else {
            input = &remainder[1..];
        }
    }
}
