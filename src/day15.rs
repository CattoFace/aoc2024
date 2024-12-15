use aoc_runner_derive::aoc;

const WIDTH: usize = 51;
const DWIDTH: usize = WIDTH * 2 - 1;
const HEIGHT: usize = 50;

pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    part1_second(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    part2_first(&input[..input.len() - 1])
}

fn try_push_right(to_push: usize, map: &mut [u8]) -> bool {
    let next = to_push + 1;
    let object_at_next = map[next];
    if object_at_next == b'.' || (object_at_next != b'#' && try_push_right(next, map)) {
        // empty cell or box was moved
        map[next] = map[to_push];
        map[to_push] = b'.';
        true
    } else {
        false
    }
}

fn try_push_left(to_push: usize, map: &mut [u8]) -> bool {
    let next = to_push - 1;
    let object_at_next = map[next];
    if object_at_next == b'.' || (object_at_next != b'#' && try_push_left(next, map)) {
        // empty cell or box was moved
        map[next] = map[to_push];
        map[to_push] = b'.';
        true
    } else {
        false
    }
}

fn try_push_down(to_push: usize, map: &mut [u8]) -> bool {
    let next = to_push + WIDTH;
    let object_at_next = map[next];
    if object_at_next == b'.' || (object_at_next != b'#' && try_push_down(next, map)) {
        // empty cell or box was moved
        map[next] = map[to_push];
        map[to_push] = b'.';
        true
    } else {
        false
    }
}

fn can_push_down_wide(to_push: usize, map: &[u8]) -> bool {
    let next = to_push + DWIDTH;
    match map[next] {
        b'.' => true,
        b'#' => false,
        b'[' => can_push_down_wide(next, map) && can_push_down_wide(next + 1, map),
        b']' => can_push_down_wide(next, map) && can_push_down_wide(next - 1, map),
        _ => unreachable!("not a map tile"),
    }
}

fn force_push_down_wide(to_push: usize, map: &mut [u8]) {
    let next = to_push + DWIDTH;
    if map[to_push] == b'.' {
        return;
    }
    match map[to_push] {
        b'[' => {
            force_push_down_wide(next, map);
            force_push_down_wide(next + 1, map);
        }
        b']' => {
            force_push_down_wide(next - 1, map);
            force_push_down_wide(next, map);
        }
        b'@' => {
            force_push_down_wide(next, map);
        }
        _ => unreachable!("not a moveable tile or force pushed into wall"),
    }
    // if there was a box in next it was moved
    match map[to_push] {
        b'[' => {
            map[next + 1] = map[to_push + 1];
            map[to_push + 1] = b'.';
        }
        b']' => {
            map[next - 1] = map[to_push - 1];
            map[to_push - 1] = b'.';
        }
        _ => {}
    }
    map[next] = map[to_push];
    map[to_push] = b'.';
}

fn try_push_down_wide(to_push: usize, map: &mut [u8]) -> bool {
    let next = to_push + DWIDTH;
    let object_at_next = map[next];
    if object_at_next == b'.' || (object_at_next != b'#' && can_push_down_wide(to_push, map)) {
        force_push_down_wide(to_push, map);
        true
    } else {
        false
    }
}

fn can_push_up_wide(to_push: usize, map: &[u8]) -> bool {
    let next = to_push - DWIDTH;
    match map[next] {
        b'.' => true,
        b'#' => false,
        b'[' => can_push_up_wide(next, map) && can_push_up_wide(next + 1, map),
        b']' => can_push_up_wide(next, map) && can_push_up_wide(next - 1, map),
        _ => unreachable!("not a map tile"),
    }
}

fn force_push_up_wide(to_push: usize, map: &mut [u8]) {
    let next = to_push - DWIDTH;
    if map[to_push] == b'.' {
        return;
    }
    match map[to_push] {
        b'[' => {
            force_push_up_wide(next, map);
            force_push_up_wide(next + 1, map);
        }
        b']' => {
            force_push_up_wide(next - 1, map);
            force_push_up_wide(next, map);
        }
        b'@' => {
            force_push_up_wide(next, map);
        }
        _ => unreachable!("not a movable tile or force pushed into wall"),
    }
    // if there was a box in next it was moved
    match map[to_push] {
        b'[' => {
            map[next + 1] = map[to_push + 1];
            map[to_push + 1] = b'.';
        }
        b']' => {
            map[next - 1] = map[to_push - 1];
            map[to_push - 1] = b'.';
        }
        _ => {}
    }
    map[next] = map[to_push];
    map[to_push] = b'.';
}

fn try_push_up_wide(to_push: usize, map: &mut [u8]) -> bool {
    let next = to_push - DWIDTH;
    let object_at_next = map[next];
    if object_at_next == b'.' || (object_at_next != b'#' && can_push_up_wide(to_push, map)) {
        force_push_up_wide(to_push, map);
        true
    } else {
        false
    }
}

fn try_push_up(to_push: usize, map: &mut [u8]) -> bool {
    let next = to_push - WIDTH;
    let object_at_next = map[next];
    if object_at_next == b'.' || (object_at_next != b'#' && try_push_up(next, map)) {
        // empty cell or box was moved
        map[next] = map[to_push];
        map[to_push] = b'.';
        true
    } else {
        false
    }
}

fn try_push_robot_wide(to_push: usize, instruction: u8, map: &mut [u8]) -> usize {
    match instruction {
        b'^' => {
            if try_push_up_wide(to_push, map) {
                to_push - DWIDTH
            } else {
                to_push
            }
        }
        b'v' => {
            if try_push_down_wide(to_push, map) {
                to_push + DWIDTH
            } else {
                to_push
            }
        }
        b'<' => {
            if try_push_left(to_push, map) {
                to_push - 1
            } else {
                to_push
            }
        }
        b'>' => {
            if try_push_right(to_push, map) {
                to_push + 1
            } else {
                to_push
            }
        }
        _ => unreachable!("read non-instruction"),
    }
}

fn try_push_robot(to_push: usize, instruction: u8, map: &mut [u8]) -> usize {
    match instruction {
        b'^' => {
            if try_push_up(to_push, map) {
                to_push - WIDTH
            } else {
                to_push
            }
        }
        b'v' => {
            if try_push_down(to_push, map) {
                to_push + WIDTH
            } else {
                to_push
            }
        }
        b'<' => {
            if try_push_left(to_push, map) {
                to_push - 1
            } else {
                to_push
            }
        }
        b'>' => {
            if try_push_right(to_push, map) {
                to_push + 1
            } else {
                to_push
            }
        }
        _ => unreachable!("read non-instruction"),
    }
}

fn try_push(to_push: usize, instruction: u8, map: &mut [u8]) -> (bool, usize) {
    let next = match instruction {
        b'^' => to_push - WIDTH,
        b'v' => to_push + WIDTH,
        b'<' => to_push - 1,
        b'>' => to_push + 1,
        _ => unreachable!("read non-instruction"),
    };
    let object_at_next = map[next];
    match object_at_next {
        b'#' => (false, to_push),
        b'.' => {
            // open space
            map[next] = map[to_push];
            map[to_push] = b'.';
            (true, next)
        }
        _ if try_push(next, instruction, map).0 => {
            // push the blocking box, at this point next is open space
            map[next] = map[to_push];
            map[to_push] = b'.';
            (true, next)
        }
        _ => {
            // push failed, no movement
            (false, to_push)
        }
    }
}

fn calc_score2(map: &[u8]) -> usize {
    let mut x_sum = 0usize;
    let mut y_sum = 0usize;
    for y in 1..HEIGHT - 1 {
        for x in 1..DWIDTH - 1 {
            if map[y * DWIDTH + x] == b'[' {
                x_sum += x;
                y_sum += y;
            }
        }
    }
    x_sum + 100 * y_sum
}

fn calc_score(map: &[u8]) -> usize {
    let mut x_sum = 0usize;
    let mut y_sum = 0usize;
    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            if map[y * WIDTH + x] == b'O' {
                x_sum += x;
                y_sum += y;
            }
        }
    }
    x_sum + 100 * y_sum
}

#[aoc(day15, part1, second)]
pub fn part1_second(input: &[u8]) -> usize {
    let mut map = input[..WIDTH * HEIGHT].to_owned();
    let mut robot = map.iter().position(|&c| c == b'@').unwrap();
    let instructions = &input[WIDTH * HEIGHT + 1..];
    instructions.iter().for_each(|&instruction| {
        if instruction != b'\n' {
            robot = try_push_robot(robot, instruction, &mut map);
        }
    });
    calc_score(&map)
}

#[aoc(day15, part1)]
pub fn part1_first(input: &[u8]) -> usize {
    let mut map = input[..WIDTH * HEIGHT].to_owned();
    let mut robot = map.iter().position(|&c| c == b'@').unwrap();
    let instructions = &input[WIDTH * HEIGHT + 1..];
    instructions.iter().for_each(|&instruction| {
        if instruction != b'\n' {
            (_, robot) = try_push(robot, instruction, &mut map);
        }
    });
    calc_score(&map)
}

#[aoc(day15, part2)]
pub fn part2_first(input: &[u8]) -> usize {
    let mut map = Vec::with_capacity(DWIDTH * HEIGHT);
    input[..WIDTH * HEIGHT].iter().for_each(|&c| match c {
        b'#' => {
            map.push(b'#');
            map.push(b'#');
        }
        b'.' => {
            map.push(b'.');
            map.push(b'.');
        }
        b'O' => {
            map.push(b'[');
            map.push(b']');
        }
        b'@' => {
            map.push(b'@');
            map.push(b'.');
        }
        b'\n' => map.push(b'\n'),
        _ => unreachable!("not a map sign"),
    });
    let mut robot = map.iter().position(|&c| c == b'@').unwrap();
    let instructions = &input[WIDTH * HEIGHT + 1..];
    instructions.iter().for_each(|&instruction| {
        if instruction != b'\n' {
            robot = try_push_robot_wide(robot, instruction, &mut map);
        }
    });
    calc_score2(&map)
}
