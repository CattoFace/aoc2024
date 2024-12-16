use std::collections::BinaryHeap;

use aoc_runner_derive::aoc;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_opt(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    part2_first(&input[..input.len() - 1])
}
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

#[derive(Eq, PartialEq, Debug)]
struct Step {
    pos: usize,
    dir: Direction,
    cost: u32,
}
#[derive(Eq, PartialEq, Debug)]
struct StepH {
    step: Step,
    history: Vec<usize>,
}

impl Ord for StepH {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.step.cost.cmp(&self.step.cost)
    }
}

impl PartialOrd for StepH {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day16, part2, reconstruct)]
pub fn part2_reconstruct(input: &[u8]) -> u32 {
    let mut queue = BinaryHeap::<Step>::new();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let end = 2 * width - 3;
    let start = input.len() - 2 * width + 2;
    let mut visited = vec![u32::MAX; input.len() * 2];
    queue.push(Step {
        pos: start,
        dir: Direction::Right,
        cost: 0,
    });
    let mut min_cost = u32::MAX;
    while let Some(step) = queue.pop() {
        if step.cost > min_cost
            || step.cost > visited[step.pos + input.len() * (step.dir as u8 & 1) as usize]
        {
            continue;
        }
        visited[step.pos + input.len() * (step.dir as u8 & 1) as usize] = step.cost;
        if step.pos == end {
            min_cost = step.cost;
            continue;
        }
        match step.dir {
            Direction::Right => {
                if input[step.pos + 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos + 1,
                        dir: Direction::Right,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos - width] != b'#' {
                    queue.push(Step {
                        pos: step.pos - width,
                        dir: Direction::Up,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos + width] != b'#' {
                    queue.push(Step {
                        pos: step.pos + width,
                        dir: Direction::Down,
                        cost: step.cost + 1001,
                    });
                }
            }
            Direction::Left => {
                if input[step.pos - 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos - 1,
                        dir: Direction::Left,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos - width] != b'#' {
                    queue.push(Step {
                        pos: step.pos - width,
                        dir: Direction::Up,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos + width] != b'#' {
                    queue.push(Step {
                        pos: step.pos + width,
                        dir: Direction::Down,
                        cost: step.cost + 1001,
                    });
                }
            }
            Direction::Up => {
                if input[step.pos - width] != b'#' {
                    queue.push(Step {
                        pos: step.pos - width,
                        dir: Direction::Up,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos + 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos + 1,
                        dir: Direction::Right,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos - 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos - 1,
                        dir: Direction::Left,
                        cost: step.cost + 1001,
                    });
                }
            }
            Direction::Down => {
                if input[step.pos + width] != b'#' {
                    queue.push(Step {
                        pos: step.pos + width,
                        dir: Direction::Down,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos + 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos + 1,
                        dir: Direction::Right,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos - 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos - 1,
                        dir: Direction::Left,
                        cost: step.cost + 1001,
                    });
                }
            }
        }
    }
    reconstruct_paths(visited, end, width)
}

fn reconstruct_paths(mut visited: Vec<u32>, end: usize, width: usize) -> u32 {
    // start at 1 because start position will not update counter
    let mut good_spots_count = 1u32;
    let mut queue = Vec::with_capacity(visited.len());
    let vertical_start = visited.len() / 2;
    let mut available_spots = vec![true; vertical_start];
    if visited[end] != u32::MAX {
        queue.push(end);
    }
    if visited[end + vertical_start] != u32::MAX {
        queue.push(end + vertical_start);
    }
    while let Some(pos) = queue.pop() {
        let curr_cost = std::mem::replace(&mut visited[pos], u32::MAX);
        if curr_cost == 0 {
            continue;
        }
        if pos >= vertical_start {
            // got to pos with vertical movement
            // add to good spots if not already
            good_spots_count +=
                std::mem::replace(&mut available_spots[pos - vertical_start], false) as u32;
            // up
            if visited[pos - width] == curr_cost - 1 {
                queue.push(pos - width);
            }
            // down
            if visited[pos + width] == curr_cost - 1 {
                queue.push(pos + width);
            }
            // left
            if visited[pos - width - vertical_start] == curr_cost - 1001 {
                queue.push(pos - width - vertical_start);
            }
            //right
            if visited[pos + width - vertical_start] == curr_cost - 1001 {
                queue.push(pos + width - vertical_start);
            }
        } else {
            // got to pos with horizontal movement
            // add to good spots if not already
            good_spots_count += std::mem::replace(&mut available_spots[pos], false) as u32;
            // up
            if visited[pos - 1 + vertical_start] == curr_cost - 1001 {
                queue.push(pos - 1 + vertical_start);
            }
            // down
            if visited[pos + 1 + vertical_start] == curr_cost - 1001 {
                queue.push(pos + 1 + vertical_start);
            }
            // left
            if visited[pos - 1] == curr_cost - 1 {
                queue.push(pos - 1);
            }
            //right
            if visited[pos + 1] == curr_cost - 1 {
                queue.push(pos + 1);
            }
        }
    }
    good_spots_count
}

#[aoc(day16, part2)]
pub fn part2_first(input: &[u8]) -> u32 {
    let mut queue = BinaryHeap::<StepH>::new();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let end = 2 * width - 3;
    let start = input.len() - 2 * width + 2;
    let mut visited = vec![u32::MAX; input.len() * 2];
    queue.push(StepH {
        step: Step {
            pos: start,
            dir: Direction::Right,
            cost: 0,
        },
        history: Default::default(),
    });
    let mut available_positions = vec![true; input.len()];
    let mut good_spots = 0u32;
    let mut min_cost = u32::MAX;
    while let Some(StepH { step, mut history }) = queue.pop() {
        history.push(step.pos);
        if step.pos == end {
            visited[step.pos + input.len() * (step.dir as u8 & 1) as usize] = step.cost;
            history.into_iter().for_each(|p| {
                good_spots += std::mem::replace(&mut available_positions[p], false) as u32
            });
            min_cost = step.cost;
            continue;
        }
        if step.cost >= min_cost
            || step.cost > visited[step.pos + input.len() * (step.dir as u8 & 1) as usize]
        {
            continue;
        }
        visited[step.pos + input.len() * (step.dir as u8 & 1) as usize] = step.cost;
        match step.dir {
            Direction::Right => {
                if input[step.pos + 1] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos + 1,
                            dir: Direction::Right,
                            cost: step.cost + 1,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos - width] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos - width,
                            dir: Direction::Up,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos + width] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos + width,
                            dir: Direction::Down,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
            }
            Direction::Left => {
                if input[step.pos - 1] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos - 1,
                            dir: Direction::Left,
                            cost: step.cost + 1,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos - width] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos - width,
                            dir: Direction::Up,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos + width] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos + width,
                            dir: Direction::Down,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
            }
            Direction::Up => {
                if input[step.pos - width] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos - width,
                            dir: Direction::Up,
                            cost: step.cost + 1,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos + 1] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos + 1,
                            dir: Direction::Right,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos - 1] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos - 1,
                            dir: Direction::Left,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
            }
            Direction::Down => {
                if input[step.pos + width] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos + width,
                            dir: Direction::Down,
                            cost: step.cost + 1,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos + 1] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos + 1,
                            dir: Direction::Right,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
                if input[step.pos - 1] != b'#' {
                    queue.push(StepH {
                        step: Step {
                            pos: step.pos - 1,
                            dir: Direction::Left,
                            cost: step.cost + 1001,
                        },
                        history: history.clone(),
                    });
                }
            }
        }
    }
    good_spots
}

#[aoc(day16, part1, opt)]
pub fn part1_opt(input: &[u8]) -> u32 {
    let mut queue = BinaryHeap::<Step>::new();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let end = 2 * width - 3;
    let mut visited = vec![false; input.len() * 2];
    let start = input.len() - 2 * width + 2;
    queue.push(Step {
        pos: start,
        dir: Direction::Right,
        cost: 0,
    });
    loop {
        let step = queue.pop().unwrap();
        if step.pos == end {
            return step.cost;
        }
        if visited[step.pos + input.len() * (step.dir as u8 & 1) as usize] {
            continue;
        }
        visited[step.pos + input.len() * (step.dir as u8 & 1) as usize] = true;
        match step.dir {
            Direction::Right => {
                if input[step.pos + 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos + 1,
                        dir: Direction::Right,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos - width] != b'#' {
                    queue.push(Step {
                        pos: step.pos - width,
                        dir: Direction::Up,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos + width] != b'#' {
                    queue.push(Step {
                        pos: step.pos + width,
                        dir: Direction::Down,
                        cost: step.cost + 1001,
                    });
                }
            }
            Direction::Left => {
                if input[step.pos - 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos - 1,
                        dir: Direction::Left,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos - width] != b'#' {
                    queue.push(Step {
                        pos: step.pos - width,
                        dir: Direction::Up,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos + width] != b'#' {
                    queue.push(Step {
                        pos: step.pos + width,
                        dir: Direction::Down,
                        cost: step.cost + 1001,
                    });
                }
            }
            Direction::Up => {
                if input[step.pos - width] != b'#' {
                    queue.push(Step {
                        pos: step.pos - width,
                        dir: Direction::Up,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos + 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos + 1,
                        dir: Direction::Right,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos - 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos - 1,
                        dir: Direction::Left,
                        cost: step.cost + 1001,
                    });
                }
            }
            Direction::Down => {
                if input[step.pos + width] != b'#' {
                    queue.push(Step {
                        pos: step.pos + width,
                        dir: Direction::Down,
                        cost: step.cost + 1,
                    });
                }
                if input[step.pos + 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos + 1,
                        dir: Direction::Right,
                        cost: step.cost + 1001,
                    });
                }
                if input[step.pos - 1] != b'#' {
                    queue.push(Step {
                        pos: step.pos - 1,
                        dir: Direction::Left,
                        cost: step.cost + 1001,
                    });
                }
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn part1_first(input: &[u8]) -> u32 {
    let mut queue = BinaryHeap::<Step>::new();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let end = 2 * width - 3;
    let mut visited = vec![false; input.len() * 4];
    let start = input.len() - 2 * width + 2;
    queue.push(Step {
        pos: start,
        dir: Direction::Right,
        cost: 0,
    });
    loop {
        let step = queue.pop().unwrap();
        if step.pos == end {
            return step.cost;
        }
        if visited[step.pos + input.len() * step.dir as usize] {
            continue;
        }
        visited[step.pos + input.len() * step.dir as usize] = true;
        if input[step.pos] == b'#' {
            continue;
        }
        match step.dir {
            Direction::Right => {
                queue.push(Step {
                    pos: step.pos + 1,
                    dir: Direction::Right,
                    cost: step.cost + 1,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Up,
                    cost: step.cost + 1000,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Down,
                    cost: step.cost + 1000,
                });
            }
            Direction::Left => {
                queue.push(Step {
                    pos: step.pos - 1,
                    dir: Direction::Left,
                    cost: step.cost + 1,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Up,
                    cost: step.cost + 1000,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Down,
                    cost: step.cost + 1000,
                });
            }
            Direction::Up => {
                queue.push(Step {
                    pos: step.pos - width,
                    dir: Direction::Up,
                    cost: step.cost + 1,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Right,
                    cost: step.cost + 1000,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Left,
                    cost: step.cost + 1000,
                });
            }
            Direction::Down => {
                queue.push(Step {
                    pos: step.pos + width,
                    dir: Direction::Down,
                    cost: step.cost + 1,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Right,
                    cost: step.cost + 1000,
                });
                queue.push(Step {
                    pos: step.pos,
                    dir: Direction::Left,
                    cost: step.cost + 1000,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::part1_first;

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_first(
                b"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            7036
        )
    }

    #[test]
    fn sample2_part1() {
        assert_eq!(
            part1_first(
                b"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            11048
        )
    }
}
