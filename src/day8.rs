use bitvec::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::array::from_fn;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

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
        impl Iterator<Item = Position>,
        impl Iterator<Item = Position>,
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
    fn get_index(self, width: i8) -> usize {
        self.y as usize * width as usize + self.x as usize
    }
}

pub fn part1(input: &str) -> u32 {
    part1_grid(input.as_bytes())
}

pub fn part2(input: &str) -> u32 {
    part2_grid(input.as_bytes())
}

#[allow(dead_code)]
fn lookup_index(c: u8) -> usize {
    let index = c as usize;
    const LUT: [usize; 256] = {
        let mut lut = [0usize; 256];
        lut[b'0' as usize] = 0;
        lut[b'1' as usize] = 1;
        lut[b'2' as usize] = 2;
        lut[b'3' as usize] = 3;
        lut[b'4' as usize] = 4;
        lut[b'5' as usize] = 5;
        lut[b'6' as usize] = 6;
        lut[b'7' as usize] = 7;
        lut[b'8' as usize] = 8;
        lut[b'9' as usize] = 9;
        lut[b'A' as usize] = 10;
        lut[b'B' as usize] = 11;
        lut[b'C' as usize] = 12;
        lut[b'D' as usize] = 13;
        lut[b'E' as usize] = 14;
        lut[b'F' as usize] = 15;
        lut[b'G' as usize] = 16;
        lut[b'H' as usize] = 17;
        lut[b'I' as usize] = 18;
        lut[b'J' as usize] = 19;
        lut[b'K' as usize] = 20;
        lut[b'L' as usize] = 21;
        lut[b'M' as usize] = 22;
        lut[b'N' as usize] = 23;
        lut[b'O' as usize] = 24;
        lut[b'P' as usize] = 25;
        lut[b'Q' as usize] = 26;
        lut[b'R' as usize] = 27;
        lut[b'S' as usize] = 28;
        lut[b'T' as usize] = 29;
        lut[b'U' as usize] = 30;
        lut[b'V' as usize] = 31;
        lut[b'W' as usize] = 32;
        lut[b'W' as usize] = 33;
        lut[b'X' as usize] = 34;
        lut[b'Y' as usize] = 35;
        lut[b'Z' as usize] = 36;
        lut[b'a' as usize] = 37;
        lut[b'b' as usize] = 38;
        lut[b'c' as usize] = 39;
        lut[b'd' as usize] = 40;
        lut[b'e' as usize] = 41;
        lut[b'f' as usize] = 42;
        lut[b'g' as usize] = 43;
        lut[b'h' as usize] = 44;
        lut[b'i' as usize] = 45;
        lut[b'j' as usize] = 46;
        lut[b'k' as usize] = 47;
        lut[b'l' as usize] = 48;
        lut[b'm' as usize] = 49;
        lut[b'n' as usize] = 50;
        lut[b'o' as usize] = 51;
        lut[b'p' as usize] = 52;
        lut[b'q' as usize] = 53;
        lut[b'r' as usize] = 54;
        lut[b's' as usize] = 55;
        lut[b't' as usize] = 56;
        lut[b'u' as usize] = 57;
        lut[b'v' as usize] = 58;
        lut[b'w' as usize] = 59;
        lut[b'w' as usize] = 60;
        lut[b'x' as usize] = 61;
        lut[b'y' as usize] = 62;
        lut[b'z' as usize] = 63;
        lut
    };
    LUT[index]
}

// b'z'-b'0'=80
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

#[aoc(day8, part1, grid_par)]
pub fn part1_grid_par(input: &[u8]) -> u32 {
    let grid = (0..input.len())
        .map(|_| AtomicBool::new(false))
        .collect::<Vec<AtomicBool>>();
    let (antennas, width, height) = find_antennas(input);
    antennas
        .par_iter()
        .flat_map_iter(|freq| {
            freq.iter()
                .tuple_combinations()
                .flat_map(|(&antenna1, &antenna2)| antenna1.resonate(antenna2))
        })
        .filter(|&p| {
            if p.x >= 0 && p.x < width && p.y >= 0 && p.y < height {
                let index = p.y as usize * width as usize + p.x as usize;
                if !grid[index].load(Ordering::Relaxed) {
                    grid[index].store(true, Ordering::Relaxed);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .count() as u32
}

#[aoc(day8, part1, grid)]
pub fn part1_grid(input: &[u8]) -> u32 {
    let mut grid = bitvec![0;input.len()];
    let mut count = 0u32;
    let (antennas, width, height) = find_antennas(input);
    antennas.iter().for_each(|freq| {
        freq.iter()
            .tuple_combinations()
            .for_each(|(&antenna1, &antenna2)| {
                let [antinode1, antinode2] = antenna1.resonate(antenna2);
                if antinode1.x >= 0
                    && antinode1.x < width
                    && antinode1.y >= 0
                    && antinode1.y < height
                {
                    let index = antinode1.y as usize * width as usize + antinode1.x as usize;
                    if !grid[index] {
                        grid.set(index, true);
                        count += 1;
                    }
                }
                if antinode2.x >= 0
                    && antinode2.x < width
                    && antinode2.y >= 0
                    && antinode2.y < height
                {
                    let index = antinode2.y as usize * width as usize + antinode2.x as usize;
                    if !grid[index] {
                        grid.set(index, true);
                        count += 1;
                    }
                }
            })
    });
    count
}

#[aoc(day8, part1, unique_par)]
pub fn part1_unique_par(input: &[u8]) -> u32 {
    let (antennas, width, height) = find_antennas(input);
    let antinodes: HashSet<Position> = antennas
        .par_iter()
        .flat_map_iter(|freq| {
            freq.iter()
                .tuple_combinations()
                .flat_map(|(&antenna1, &antenna2)| antenna1.resonate(antenna2))
        })
        .filter(|&p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
        .collect();
    antinodes.len() as u32
}

#[aoc(day8, part1, unique)]
pub fn part1_unique(input: &[u8]) -> u32 {
    let (antennas, width, height) = find_antennas(input);
    antennas
        .iter()
        .flat_map(|freq| {
            freq.iter()
                .tuple_combinations()
                .flat_map(|(&antenna1, &antenna2)| antenna1.resonate(antenna2))
        })
        .filter(|&p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
        .unique()
        .count() as u32
}

#[aoc(day8, part2, grid_par)]
pub fn part2_grid_par(input: &[u8]) -> u32 {
    let grid = (0..input.len())
        .map(|_| AtomicBool::new(false))
        .collect::<Vec<AtomicBool>>();
    let (antennas, width, height) = find_antennas(input);
    antennas
        .par_iter()
        .flat_map_iter(|freq| {
            freq.iter()
                .tuple_combinations()
                .flat_map(|(&antenna1, &antenna2)| {
                    let (res1, res2) = antenna1.infinite_resonation(antenna2);
                    [antenna1, antenna2]
                        .into_iter()
                        .chain(
                            res1.take_while(|&p| {
                                p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
                            }),
                        )
                        .chain(
                            res2.take_while(|&p| {
                                p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
                            }),
                        )
                })
        })
        .filter(|&p| {
            let index = p.y as usize * width as usize + p.x as usize;
            if !grid[index].load(Ordering::Relaxed) {
                grid[index].store(true, Ordering::Relaxed);
                true
            } else {
                false
            }
        })
        .count() as u32
}

#[aoc(day8, part2, grid)]
pub fn part2_grid(input: &[u8]) -> u32 {
    let mut grid: BitVec = bitvec![0;input.len()];
    let mut count = 0u32;
    let (antennas, width, height) = find_antennas(input);
    antennas.iter().for_each(|freq| {
        freq.iter().for_each(|p| {
            grid.set(p.get_index(width), true);
        });
        count += freq.len() as u32;
    });
    antennas.iter().for_each(|freq| {
        freq.iter()
            .tuple_combinations()
            .for_each(|(antenna1, antenna2)| {
                let (res1, res2) = antenna1.infinite_resonation(*antenna2);
                res1.take_while(|&p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
                    .for_each(|p| {
                        let index = p.get_index(width);
                        if !grid[index] {
                            grid.set(p.get_index(width), true);
                            count += 1;
                        }
                    });
                res2.take_while(|&p| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height)
                    .for_each(|p| {
                        let index = p.get_index(width);
                        if !grid[index] {
                            grid.set(p.get_index(width), true);
                            count += 1;
                        }
                    });
            })
    });
    count
}

#[aoc(day8, part2, unique_par)]
pub fn part2_unique_par(input: &[u8]) -> u32 {
    let (antennas, width, height) = find_antennas(input);
    let antinodes: HashSet<Position> =
        antennas
            .par_iter()
            .flat_map_iter(|freq| {
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
    antinodes.len() as u32
}

#[aoc(day8, part2, unique)]
pub fn part2_unique(input: &[u8]) -> u32 {
    let (antennas, width, height) = find_antennas(input);
    antennas
        .iter()
        .flat_map(|freq| {
            freq.iter()
                .tuple_combinations()
                .flat_map(|(&antenna1, &antenna2)| {
                    let (res1, res2) = antenna1.infinite_resonation(antenna2);
                    [antenna1, antenna2]
                        .into_iter()
                        .chain(
                            res1.take_while(|&p| {
                                p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
                            }),
                        )
                        .chain(
                            res2.take_while(|&p| {
                                p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
                            }),
                        )
                })
        })
        .unique()
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day8::part1_unique;

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_unique(
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
            part1_unique(
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
            part1_unique(
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
