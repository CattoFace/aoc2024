use std::{array::from_fn, collections::HashSet};

use aoc_runner_derive::aoc;
use itertools::Itertools;
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i8,
    y: i8,
}
impl Position {
    fn infinite_resonation(
        self,
        other: Position,
    ) -> (
        std::iter::Map<std::ops::RangeFrom<i8>, impl FnMut(i8) -> Position>,
        std::iter::Map<std::ops::RangeFrom<i8>, impl FnMut(i8) -> Position>,
    ) {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        (
            (1..).map(move |res_mul| Position {
                x: self.x + x_diff * res_mul,
                y: self.y + y_diff * res_mul,
            }),
            (1..).map(move |res_mul| -> Position {
                Position {
                    x: other.x - x_diff * res_mul,
                    y: other.y - y_diff * res_mul,
                }
            }),
        )
    }
    fn resonate(self, other: Position) -> [Position; 2] {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        [
            Position {
                x: self.x + x_diff,
                y: self.y + y_diff,
            },
            Position {
                x: other.x - x_diff,
                y: other.y - y_diff,
            },
        ]
    }
}

fn find_antennas(input: &[u8]) -> ([Vec<Position>; 80], i8, i8) {
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.len() / width;
    let mut antennas: [Vec<Position>; 80] = from_fn(|_| Vec::new());
    input.iter().enumerate().for_each(|(index, &c)| {
        if c != b'.' && c != b'\n' {
            antennas[(c - b'0') as usize].push(Position {
                x: (index % (width + 1)) as i8,
                y: (index / (width + 1)) as i8,
            })
        }
    });
    (antennas, width as i8, height as i8)
}

#[aoc(day8, part1, first)]
pub fn part1_first(input: &[u8]) -> u32 {
    let (antennas, width, height) = find_antennas(input);
    let resonate_positions: HashSet<Position> = antennas
        .iter()
        .flat_map(|freq| {
            freq.iter()
                .tuple_combinations()
                .flat_map(|(&antenna1, &antenna2)| antenna1.resonate(antenna2))
        })
        .filter(|&p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
        .collect();
    resonate_positions.len() as u32
}

#[aoc(day8, part2, first)]
pub fn part2_first(input: &[u8]) -> u32 {
    let (antennas, width, height) = find_antennas(input);
    // dbg!(width, height);
    let resonate_positions: HashSet<Position> =
        antennas
            .iter()
            .flat_map(|freq| {
                freq.iter()
                    .tuple_combinations()
                    .flat_map(|(&antenna1, &antenna2)| {
                        let (res1, res2) = antenna1.infinite_resonation(antenna2);
                        [antenna1, antenna2]
                            .into_iter()
                            .chain(res1.take_while(|&p| {
                                p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
                            }))
                            .chain(res2.take_while(|&p| {
                                p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
                            }))
                    })
            })
            .collect();
    resonate_positions.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::day8::part1_first;

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_first(
                b"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            ),
            2
        )
    }

    #[test]
    fn sample2_part1() {
        assert_eq!(
            part1_first(
                b"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."
            ),
            4
        )
    }
    #[test]
    fn sample3_part1() {
        assert_eq!(
            part1_first(
                b"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            14
        )
    }
}
