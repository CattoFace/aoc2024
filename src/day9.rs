use std::{cmp::Ordering, collections::VecDeque, iter::repeat_n};

use aoc_runner_derive::aoc;

const EMPTY_FILE: u32 = u32::MAX;

pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    part1_no_parse(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    part2_first(&input[..input.len() - 1])
}

fn sum_range(first: u64, last: u64) -> u64 {
    (first + last - 1) * (last - first) / 2
}
fn build_disk(input: &[u8]) -> Vec<u32> {
    let mut last_id = 0u32;
    // create disk
    let mut disk = input
        .array_chunks()
        .enumerate()
        .flat_map(|(id, pair): (usize, &[u8; 2])| {
            let id = id as u32;
            let filled = (pair[0] - b'0') as usize;
            let empty = (pair[1] - b'0') as usize;
            last_id = id;
            repeat_n(id, filled).chain(repeat_n(EMPTY_FILE, empty))
        })
        .collect::<Vec<u32>>();
    // adding the last block
    if input.len() % 2 != 0 {
        disk.extend(repeat_n(
            last_id + 1,
            (input[input.len() - 1] - b'0') as usize,
        ));
    }
    disk
}

fn compact_disk(disk: &mut [u32]) {
    let mut empty_location = disk.iter().position(|&id| id == EMPTY_FILE).unwrap();
    let mut filled_location = disk.len() - 1;
    while empty_location < filled_location {
        disk.swap(empty_location, filled_location);
        empty_location = disk.iter().position(|&id| id == EMPTY_FILE).unwrap();
        filled_location =
            disk.len() - 1 - disk.iter().rev().position(|&id| id != EMPTY_FILE).unwrap();
    }
}

fn checksum(disk: &[u32]) -> u64 {
    disk.iter()
        .take_while(|&&id| id != EMPTY_FILE)
        .enumerate()
        .map(|(index, &id)| index as u64 * id as u64)
        .sum::<u64>()
}

fn build_mini_disk_deque(input: &[u8]) -> VecDeque<(u32, u8)> {
    let mut last_id = 0u32;
    // build disk
    let mut disk = input
        .array_chunks()
        .enumerate()
        .flat_map(|(id, pair): (usize, &[u8; 2])| {
            let id = id as u32;
            let filled = pair[0] - b'0';
            let empty = pair[1] - b'0';
            last_id = id;
            [(id, filled), (EMPTY_FILE, empty)]
        })
        .filter(|chunk| chunk.1 != 0)
        .collect::<VecDeque<(u32, u8)>>();
    disk.push_back((last_id + 1, (input[input.len() - 1] - b'0')));
    disk
}

fn build_mini_disk(input: &[u8]) -> Vec<(u32, u8)> {
    let mut last_id = 0u32;
    // build disk
    let mut disk = input
        .array_chunks()
        .enumerate()
        .flat_map(|(id, pair): (usize, &[u8; 2])| {
            let id = id as u32;
            let filled = pair[0] - b'0';
            let empty = pair[1] - b'0';
            last_id = id;
            [(id, filled), (EMPTY_FILE, empty)]
        })
        .filter(|chunk| chunk.1 != 0)
        .collect::<Vec<(u32, u8)>>();
    disk.push((last_id + 1, (input[input.len() - 1] - b'0')));
    disk
}

#[aoc(day9, part1, no_parse)]
pub fn part1_no_parse(disk: &[u8]) -> u64 {
    let mut read = (disk[0] - b'0') as u64;
    let mut checksum = 0u64;
    let mut front_index = 1u32;
    let mut back_index = disk.len() as u32 - 1;
    let mut front = disk[1] - b'0';
    let mut back = disk[back_index as usize] - b'0';
    // the body enforces the front is always empty at the start of the loop
    loop {
        match front.cmp(&back) {
            // less empty than there is to fill
            Ordering::Less => {
                checksum += sum_range(read, read + front as u64) * (back_index / 2) as u64;
                read += front as u64;
                back -= front;
                // grab a new front number
                front_index += 1;
                if front_index < back_index {
                    // next is filled file
                    front = disk[front_index as usize] - b'0';
                    checksum += sum_range(read, read + front as u64) * (front_index / 2) as u64;
                    read += front as u64;
                    // next is empty
                    front_index += 1;
                    front = disk[front_index as usize] - b'0';
                } else {
                    checksum += sum_range(read, read + back as u64) * (back_index / 2) as u64;
                    return checksum;
                }
            }
            // exact size to fill
            Ordering::Equal => {
                checksum += sum_range(read, read + back as u64) * (back_index / 2) as u64;
                read += back as u64;
                // grab a new front number
                front_index += 1;
                if front_index < back_index {
                    // next is filled file
                    front = disk[front_index as usize] - b'0';
                    checksum += sum_range(read, read + front as u64) * (front_index / 2) as u64;
                    read += front as u64;
                    // next is empty
                    front_index += 1;
                    front = disk[front_index as usize] - b'0';
                } else {
                    return checksum;
                }
                // grab a new back number, skip empty files
                back_index -= 2;
                if front_index < back_index {
                    back = disk[back_index as usize] - b'0';
                } else {
                    return checksum;
                }
            }
            // more empty than there is to fill,
            // the only case reading a new file from the front is not needed
            Ordering::Greater => {
                checksum += sum_range(read, read + back as u64) * (back_index / 2) as u64;
                read += back as u64;
                front -= back;
                // grab a new back number, skip empty files
                back_index -= 2;
                if front_index < back_index {
                    back = disk[back_index as usize] - b'0';
                } else {
                    return checksum;
                }
            }
        }
    }
}

fn checksum_skip_compact(mut disk: VecDeque<(u32, u8)>) -> u64 {
    let mut checksum = 0u64;
    let mut read = 0u64;
    let mut front = disk.pop_front().unwrap();
    let mut back = disk.pop_back().unwrap();
    loop {
        if front.0 != EMPTY_FILE {
            checksum += (read..read + front.1 as u64).sum::<u64>() * front.0 as u64;
            read += front.1 as u64;
            front = if let Some(f) = disk.pop_front() {
                f
            } else {
                return checksum;
            }
        } else {
            match front.1.cmp(&back.1) {
                // less empty than there is to fill
                Ordering::Less => {
                    checksum += (read..read + front.1 as u64).sum::<u64>() * back.0 as u64;
                    read += front.1 as u64;
                    back.1 -= front.1;
                    front = if let Some(f) = disk.pop_front() {
                        f
                    } else {
                        checksum += (read..read + back.1 as u64).sum::<u64>() * back.0 as u64;
                        return checksum;
                    };
                }
                // exact size to fill
                Ordering::Equal => {
                    checksum += (read..read + back.1 as u64).sum::<u64>() * back.0 as u64;
                    read += back.1 as u64;
                    front = if let Some(f) = disk.pop_front() {
                        f
                    } else {
                        checksum += (read..read + back.1 as u64).sum::<u64>() * back.0 as u64;
                        return checksum;
                    };
                    back = if let Some(f) = disk.pop_back() {
                        f
                    } else {
                        return checksum;
                    };
                    while back.0 == EMPTY_FILE {
                        back = if let Some(f) = disk.pop_back() {
                            f
                        } else {
                            return checksum;
                        };
                    }
                }
                // more empty than there is to fill
                Ordering::Greater => {
                    checksum += (read..read + back.1 as u64).sum::<u64>() * back.0 as u64;
                    read += back.1 as u64;
                    front.1 -= back.1;
                    back = if let Some(f) = disk.pop_back() {
                        f
                    } else {
                        return checksum;
                    };
                    while back.0 == EMPTY_FILE {
                        back = if let Some(f) = disk.pop_back() {
                            f
                        } else {
                            return checksum;
                        };
                    }
                }
            }
        }
    }
}

fn compact_mini_disk2(disk: &mut Vec<(u32, u8)>) {
    // last file always not empty
    let mut read_head = disk.len() - 1;
    loop {
        // try to find empty location to move file at read_head
        if let Some(write_head) = disk[..read_head].iter().position(|potential_file| {
            potential_file.0 == EMPTY_FILE && disk[read_head].1 <= potential_file.1
        }) {
            // swap into empty file
            let empty_size = disk[write_head].1;
            let read_size = disk[read_head].1;
            disk.swap(read_head, write_head);
            disk[read_head].1 = read_size;
            // reinsert smaller empty file if needed
            if read_size < empty_size {
                disk.insert(write_head + 1, (EMPTY_FILE, empty_size - read_size));
                read_head += 1;
            }
        }
        // find new potential file to move
        if let Some(next_read) = disk[..read_head]
            .iter()
            .rev()
            .position(|file| file.0 != EMPTY_FILE)
        {
            read_head = read_head - 1 - next_read;
        } else {
            return;
        }
    }
}

fn checksum_mini(disk: &[(u32, u8)]) -> u64 {
    disk.iter()
        .fold((0u64, 0u64), |(location, checksum), &(id, size)| {
            let new_location = location + size as u64;
            let new_checksum = if id == EMPTY_FILE {
                checksum
            } else {
                checksum + sum_range(location, new_location) * id as u64
            };
            (new_location, new_checksum)
        })
        .1
}

#[aoc(day9, part2, first)]
pub fn part2_first(input: &[u8]) -> u64 {
    let mut mini_disk = build_mini_disk(input);
    compact_mini_disk2(&mut mini_disk);
    checksum_mini(&mini_disk)
}

#[aoc(day9, part1, skip)]
pub fn part1_skip(input: &[u8]) -> u64 {
    let mini_disk = build_mini_disk_deque(input);
    checksum_skip_compact(mini_disk)
}

// #[aoc(day9, part1, first)]
pub fn part1_first(input: &[u8]) -> u64 {
    let mut disk = build_disk(input);
    compact_disk(&mut disk);
    checksum(&disk)
}

#[cfg(test)]
mod tests {
    use crate::day9::{part1_no_parse, part2_first};

    #[test]
    fn sample_part1() {
        assert_eq!(part1_no_parse(b"2333133121414131402"), 1928)
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2_first(b"2333133121414131402"), 2858)
    }
}
