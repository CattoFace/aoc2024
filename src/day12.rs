use aoc_runner_derive::aoc;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_floodfill(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    part2_floodfill(&input[..input.len() - 1])
}

#[aoc(day12, part1, flood_fill)]
pub fn part1_floodfill(input: &[u8]) -> u32 {
    let mut buffer = Vec::<(usize, u32, u32)>::new();
    let width = input.iter().position(|&c| c == b'\n').unwrap() as u32 + 1;
    let height = input.len() as u32 / width + 1;
    let mut index_to_group_id = vec![0u8; input.len()];
    let mut cost_sum = 0u32;
    for index in 0..input.len() {
        if input[index] == b'\n' || index_to_group_id[index] != 0 {
            continue;
        }
        cost_sum += flood_fill(
            index,
            input,
            &mut index_to_group_id,
            width,
            height,
            &mut buffer,
        );
    }
    cost_sum
}

#[aoc(day12, part2, flood_fill)]
pub fn part2_floodfill(input: &[u8]) -> u32 {
    let mut buffer = Vec::<(usize, u32, u32)>::new();
    let width = input.iter().position(|&c| c == b'\n').unwrap() as u32 + 1;
    let height = input.len() as u32 / width + 1;
    let mut index_to_group_id = vec![0u8; input.len()];
    let mut cost_sum = 0u32;
    for index in 0..input.len() {
        if input[index] == b'\n' || index_to_group_id[index] != 0 {
            continue;
        }
        cost_sum += flood_fill2(
            index,
            input,
            &mut index_to_group_id,
            width,
            height,
            &mut buffer,
        );
    }
    cost_sum
}

fn flood_fill2(
    index: usize,
    input: &[u8],
    marked: &mut [u8],
    width: u32,
    height: u32,
    queue: &mut Vec<(usize, u32, u32)>,
) -> u32 {
    let plant_type = input[index];
    let mut area = 0u32;
    let mut corners = 0u32;
    queue.push((index, index as u32 % width, index as u32 / width));
    while let Some((index, x, y)) = queue.pop() {
        if marked[index] == plant_type {
            continue; // prevent double counting
        }
        marked[index] = plant_type;
        area += 1;
        let left = x > 0 && input[index - 1] == plant_type;
        if left && marked[index - 1] == 0 {
            queue.push((index - 1, x - 1, y));
        }
        let right = x < width - 2 && input[index + 1] == plant_type;
        if right && marked[index + 1] == 0 {
            queue.push((index + 1, x + 1, y));
        }
        let up = y > 0 && input[index - width as usize] == plant_type;
        if up && marked[index - width as usize] == 0 {
            queue.push((index - width as usize, x, y - 1));
        }
        let down = y < height - 1 && input[index + width as usize] == plant_type;
        if down && marked[index + width as usize] == 0 {
            queue.push((index + width as usize, x, y + 1));
        }
        let upleft = x > 0 && y > 0 && input[index - 1 - width as usize] == plant_type;
        let upright = x < width - 2 && y > 0 && input[index + 1 - width as usize] == plant_type;
        let downleft = x > 0 && y < height - 1 && input[index - 1 + width as usize] == plant_type;
        let downright =
            x < width - 2 && y < height - 1 && input[index + 1 + width as usize] == plant_type;
        if (up && right && !upright) || (!up && !right) {
            corners += 1;
        }
        if (up && left && !upleft) || (!up && !left) {
            corners += 1;
        }
        if (down && right && !downright) || (!down && !right) {
            corners += 1;
        }
        if (down && left && !downleft) || (!down && !left) {
            corners += 1;
        }
    }
    area * corners
}

fn flood_fill(
    index: usize,
    input: &[u8],
    marked: &mut [u8],
    width: u32,
    height: u32,
    queue: &mut Vec<(usize, u32, u32)>,
) -> u32 {
    let plant_type = input[index];
    let mut area = 0u32;
    let mut perimeter = 0u32;
    queue.push((index, index as u32 % width, index as u32 / width));
    while let Some((index, x, y)) = queue.pop() {
        if marked[index] == plant_type {
            continue; // prevent double counting
        }
        marked[index] = plant_type;
        area += 1;
        perimeter += 4;
        if x > 0 && input[index - 1] == plant_type {
            if marked[index - 1] == 0 {
                queue.push((index - 1, x - 1, y));
            } else {
                perimeter -= 2; // canceling that side for both this and the neighbour
            }
        }
        if x < width - 2 && input[index + 1] == plant_type {
            if marked[index + 1] == 0 {
                queue.push((index + 1, x + 1, y));
            } else {
                perimeter -= 2; // canceling that side for both this and the neighbour
            }
        }
        if y > 0 && input[index - width as usize] == plant_type {
            if marked[index - width as usize] == 0 {
                queue.push((index - width as usize, x, y - 1));
            } else {
                perimeter -= 2; // canceling that side for both this and the neighbour
            }
        }
        if y < height - 1 && input[index + width as usize] == plant_type {
            if marked[index + width as usize] == 0 {
                queue.push((index + width as usize, x, y + 1));
            } else {
                perimeter -= 2; // canceling that side for both this and the neighbour
            }
        }
    }
    area * perimeter
}

#[cfg(test)]
mod tests {
    use crate::day12::{part1_floodfill, part2_floodfill};

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_floodfill(
                b"AAAA
BBCD
BBCC
EEEC"
            ),
            140
        )
    }

    #[test]
    fn sample2_part1() {
        assert_eq!(
            part1_floodfill(
                b"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            ),
            772
        )
    }

    #[test]
    fn sample3_part1() {
        assert_eq!(
            part1_floodfill(
                b"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1930
        )
    }
    #[test]
    fn sample_part2() {
        assert_eq!(
            part2_floodfill(
                b"AAAA
BBCD
BBCC
EEEC"
            ),
            80
        )
    }

    #[test]
    fn sample2_part2() {
        assert_eq!(
            part2_floodfill(
                b"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            ),
            436
        )
    }

    #[test]
    fn sample3_part2() {
        assert_eq!(
            part2_floodfill(
                b"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            ),
            236
        )
    }

    #[test]
    fn sample4_part2() {
        assert_eq!(
            part2_floodfill(
                b"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            ),
            368
        )
    }

    #[test]
    fn sample5_part2() {
        assert_eq!(
            part2_floodfill(
                b"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            ),
            1206
        );
    }
}
