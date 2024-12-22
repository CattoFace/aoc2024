#[repr(u8)]
pub enum DirKeypad {
    Up,
    Down,
    Left,
    Right,
    A,
}

pub const fn i2d(i: usize) -> DirKeypad {
    use DirKeypad::*;
    match i {
        0 => Up,
        1 => Down,
        2 => Left,
        3 => Right,
        4 => A,
        _ => unreachable!(),
    }
}

pub const fn p2i(start: DirKeypad, end: DirKeypad) -> usize {
    (start as u8 * 5 + end as u8) as usize
}

pub const fn i2n(i: u64) -> Numpad {
    use Numpad::*;
    match i {
        0 => Zero,
        1 => One,
        2 => Two,
        3 => Three,
        4 => Four,
        5 => Five,
        6 => Six,
        7 => Seven,
        8 => Eight,
        9 => Nine,
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone)]
pub enum Numpad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

pub fn code_to_amount(d1: Numpad, d2: Numpad, d3: Numpad, robots: u8) -> u64 {
    let mut sum = 0u64;
    let table = generate_multirobot_table(robots);
    sum += numpad_robot_with_table(Numpad::A, d1, table);
    sum += numpad_robot_with_table(d1, d2, table);
    sum += numpad_robot_with_table(d2, d3, table);
    sum += numpad_robot_with_table(d3, Numpad::A, table);
    sum
}

fn robot_with_table(start: DirKeypad, end: DirKeypad, table: [u64; 25]) -> u64 {
    use DirKeypad::*;
    match (start, end) {
        (Up, Up) => table[p2i(A, A)],
        (Up, Down) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (Up, Left) => table[p2i(A, Down)] + table[p2i(Down, Left)] + table[p2i(Left, A)],
        (Up, Right) => u64::min(
            table[p2i(A, Right)] + table[p2i(Right, Down)] + table[p2i(Down, A)],
            table[p2i(A, Down)] + table[p2i(Down, Right)] + table[p2i(Right, A)],
        ),
        (Up, A) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (Down, Up) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (Down, Down) => table[p2i(A, A)],
        (Down, Left) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (Down, Right) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (Down, A) => u64::min(
            table[p2i(A, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)],
            table[p2i(A, Right)] + table[p2i(Right, Up)] + table[p2i(Up, A)],
        ),
        (Left, Up) => table[p2i(A, Right)] + table[p2i(Right, Up)] + table[p2i(Up, A)],
        (Left, Down) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (Left, Left) => table[p2i(A, A)],
        (Left, Right) => table[p2i(A, Right)] + table[p2i(Right, Right)] + table[p2i(Right, A)],
        (Left, A) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, Up)]
                + table[p2i(Up, A)]
        }
        (Right, Up) => u64::min(
            table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)],
            table[p2i(A, Left)] + table[p2i(Left, Up)] + table[p2i(Up, A)],
        ),
        (Right, Down) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (Right, Left) => table[p2i(A, Left)] + table[p2i(Left, Left)] + table[p2i(Left, A)],
        (Right, Right) => table[p2i(A, A)],
        (Right, A) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (A, Up) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (A, Down) => u64::min(
            table[p2i(A, Down)] + table[p2i(Down, Left)] + table[p2i(Left, A)],
            table[p2i(A, Left)] + table[p2i(Left, Down)] + table[p2i(Down, A)],
        ),
        (A, Left) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, Left)]
                + table[p2i(Left, A)]
        }
        (A, Right) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (A, A) => table[p2i(A, A)],
    }
}

pub fn generate_multirobot_table(robots: u8) -> [u64; 25] {
    let mut curr_table = [0u64; 25];
    for (i, v) in curr_table.iter_mut().enumerate() {
        *v = robot1(i2d(i / 5), i2d(i % 5)) as u64;
    }
    for _ in 1..robots {
        let mut next_table = [0u64; 25];
        for (i, v) in next_table.iter_mut().enumerate() {
            *v = robot_with_table(i2d(i / 5), i2d(i % 5), curr_table);
        }
        curr_table = next_table;
    }
    curr_table
}

const fn numpad_robot_with_table(start: Numpad, end: Numpad, table: [u64; 25]) -> u64 {
    use DirKeypad::*;
    use Numpad as N;
    match (start, end) {
        (N::Zero, N::Zero) => table[p2i(A, A)],
        (N::Zero, N::One) => table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)],
        (N::Zero, N::Two) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::Zero, N::Three) => table[p2i(A, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)],
        (N::Zero, N::Four) => {
            table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)]
        }
        (N::Zero, N::Five) => table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, A)],
        (N::Zero, N::Six) => {
            table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)]
        }
        (N::Zero, N::Seven) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Left)]
                + table[p2i(Left, A)]
        }
        (N::Zero, N::Eight) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, A)]
        }
        (N::Zero, N::Nine) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Right)]
                + table[p2i(Up, A)]
        }
        (N::Zero, N::A) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (N::One, N::Zero) => table[p2i(A, Right)] + table[p2i(Right, Down)] + table[p2i(Down, A)],
        (N::One, N::One) => table[p2i(A, A)],
        (N::One, N::Two) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (N::One, N::Three) => {
            table[p2i(A, Right)] + table[p2i(Right, Right)] + table[p2i(Right, A)]
        }
        (N::One, N::Four) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::One, N::Five) => table[p2i(A, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)],
        (N::One, N::Six) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, A)]
        }
        (N::One, N::Seven) => table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, A)],
        (N::One, N::Eight) => {
            table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)]
        }
        (N::One, N::Nine) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, A)]
        }
        (N::One, N::A) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, A)]
        }
        (N::Two, N::Zero) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Two, N::One) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (N::Two, N::Two) => table[p2i(A, A)],
        (N::Two, N::Three) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (N::Two, N::Four) => table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)],
        (N::Two, N::Five) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::Two, N::Six) => table[p2i(A, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)],
        (N::Two, N::Seven) => {
            table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)]
        }
        (N::Two, N::Eight) => table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, A)],
        (N::Two, N::Nine) => {
            table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)]
        }
        (N::Two, N::A) => table[p2i(A, Right)] + table[p2i(Right, Down)] + table[p2i(Down, A)],
        (N::Three, N::Zero) => table[p2i(A, Down)] + table[p2i(Down, Left)] + table[p2i(Left, A)],
        (N::Three, N::One) => table[p2i(A, Left)] + table[p2i(Left, Left)] + table[p2i(Left, A)],
        (N::Three, N::Two) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (N::Three, N::Three) => table[p2i(A, A)],
        (N::Three, N::Four) => {
            table[p2i(A, Left)] + table[p2i(Left, Left)] + table[p2i(Left, Up)] + table[p2i(Up, A)]
            // table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, Left)] + table[p2i(Left, A)]
        }
        (N::Three, N::Five) => table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)],
        (N::Three, N::Six) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::Three, N::Seven) => {
            table[p2i(A, Left)]
                + table[p2i(Left, Left)]
                + table[p2i(Left, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, A)]
        }
        (N::Three, N::Eight) => {
            table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)]
        }
        (N::Three, N::Nine) => table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, A)],
        (N::Three, N::A) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Four, N::Zero) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Four, N::One) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Four, N::Two) => table[p2i(A, Right)] + table[p2i(Right, Down)] + table[p2i(Down, A)],
        (N::Four, N::Three) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, A)]
        }
        (N::Four, N::Four) => table[p2i(A, A)],
        (N::Four, N::Five) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (N::Four, N::Six) => table[p2i(A, Right)] + table[p2i(Right, Right)] + table[p2i(Right, A)],
        (N::Four, N::Seven) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::Four, N::Eight) => table[p2i(A, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)],
        (N::Four, N::Nine) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, A)]
        }
        (N::Four, N::A) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Five, N::Zero) => table[p2i(A, Down)] + table[p2i(Down, Down)] + table[p2i(Down, A)],
        (N::Five, N::One) => table[p2i(A, Down)] + table[p2i(Down, Left)] + table[p2i(Left, A)],
        (N::Five, N::Two) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Five, N::Three) => table[p2i(A, Right)] + table[p2i(Right, Down)] + table[p2i(Down, A)],
        (N::Five, N::Four) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (N::Five, N::Five) => table[p2i(A, A)],
        (N::Five, N::Six) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (N::Five, N::Seven) => table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)],
        (N::Five, N::Eight) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::Five, N::Nine) => table[p2i(A, Up)] + table[p2i(Up, Right)] + table[p2i(Right, A)],
        (N::Five, N::A) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Six, N::Zero) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, A)]
        }
        (N::Six, N::One) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, Left)]
                + table[p2i(Left, A)]
        }
        (N::Six, N::Two) => table[p2i(A, Down)] + table[p2i(Down, Left)] + table[p2i(Left, A)],
        (N::Six, N::Three) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Six, N::Four) => table[p2i(A, Left)] + table[p2i(Left, Left)] + table[p2i(Left, A)],
        (N::Six, N::Five) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (N::Six, N::Six) => table[p2i(A, A)],
        (N::Six, N::Seven) => {
            table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, Left)] + table[p2i(Left, A)]
        }
        (N::Six, N::Eight) => table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)],
        (N::Six, N::Nine) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::Six, N::A) => table[p2i(A, Down)] + table[p2i(Down, Down)] + table[p2i(Down, A)],
        (N::Seven, N::Zero) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Seven, N::One) => table[p2i(A, Down)] + table[p2i(Down, Down)] + table[p2i(Down, A)],
        (N::Seven, N::Two) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Seven, N::Three) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Seven, N::Four) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Seven, N::Five) => {
            table[p2i(A, Right)] + table[p2i(Right, Down)] + table[p2i(Right, A)]
        }
        (N::Seven, N::Six) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, A)]
        }
        (N::Seven, N::Seven) => table[p2i(A, A)],
        (N::Seven, N::Eight) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (N::Seven, N::Nine) => {
            table[p2i(A, Right)] + table[p2i(Right, Right)] + table[p2i(Right, A)]
        }
        (N::Seven, N::A) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Eight, N::Zero) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Eight, N::One) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, A)]
        }
        (N::Eight, N::Two) => table[p2i(A, Down)] + table[p2i(Down, Down)] + table[p2i(Down, A)],
        (N::Eight, N::Three) => {
            table[p2i(A, Right)]
                + table[p2i(Right, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::Eight, N::Four) => table[p2i(A, Down)] + table[p2i(Down, Left)] + table[p2i(Down, A)],
        (N::Eight, N::Five) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Eight, N::Six) => table[p2i(A, Right)] + table[p2i(Right, Down)] + table[p2i(Down, A)],
        (N::Eight, N::Seven) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (N::Eight, N::Eight) => table[p2i(A, A)],
        (N::Eight, N::Nine) => table[p2i(A, Right)] + table[p2i(Right, A)],
        (N::Eight, N::A) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Right)]
                + table[p2i(Right, A)]
        }
        (N::Nine, N::Zero) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, A)]
        }
        (N::Nine, N::One) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, Left)]
                + table[p2i(Left, A)]
        }
        (N::Nine, N::Two) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, A)]
        }
        (N::Nine, N::Three) => table[p2i(A, Down)] + table[p2i(Down, Down)] + table[p2i(Down, A)],
        (N::Nine, N::Four) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Left)]
                + table[p2i(Left, Left)]
                + table[p2i(Left, A)]
        }
        (N::Nine, N::Five) => table[p2i(A, Down)] + table[p2i(Down, Left)] + table[p2i(Left, A)],
        (N::Nine, N::Six) => table[p2i(A, Down)] + table[p2i(Down, A)],
        (N::Nine, N::Seven) => table[p2i(A, Left)] + table[p2i(Left, Left)] + table[p2i(Left, A)],
        (N::Nine, N::Eight) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (N::Nine, N::Nine) => table[p2i(A, A)],
        (N::Nine, N::A) => {
            table[p2i(A, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, Down)]
                + table[p2i(Down, A)]
        }
        (N::A, N::Zero) => table[p2i(A, Left)] + table[p2i(Left, A)],
        (N::A, N::One) => {
            table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, Left)] + table[p2i(Left, A)]
        }
        (N::A, N::Two) => table[p2i(A, Up)] + table[p2i(Up, Left)] + table[p2i(Left, A)],
        (N::A, N::Three) => table[p2i(A, Up)] + table[p2i(Up, A)],
        (N::A, N::Four) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Left)]
                + table[p2i(Left, Left)]
                + table[p2i(Left, A)]
        }
        (N::A, N::Five) => {
            table[p2i(A, Left)] + table[p2i(Left, Up)] + table[p2i(Up, Up)] + table[p2i(Up, A)]
        }
        (N::A, N::Six) => table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, A)],
        (N::A, N::Seven) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Left)]
                + table[p2i(Left, Left)]
                + table[p2i(Left, A)]
        }
        (N::A, N::Eight) => {
            table[p2i(A, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Up)]
                + table[p2i(Up, Left)]
                + table[p2i(Left, A)]
        }
        (N::A, N::Nine) => {
            table[p2i(A, Up)] + table[p2i(Up, Up)] + table[p2i(Up, Up)] + table[p2i(Up, A)]
        }
        (N::A, N::A) => table[p2i(A, A)],
    }
}

const fn robot3(start: Numpad, end: Numpad) -> u8 {
    use DirKeypad::*;
    use Numpad as N;
    match (start, end) {
        (N::Zero, N::Zero) => robot2(A, A),
        (N::Zero, N::One) => robot2(A, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Zero, N::Two) => robot2(A, Up) + robot2(Up, A),
        (N::Zero, N::Three) => robot2(A, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::Zero, N::Four) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Zero, N::Five) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, A),
        (N::Zero, N::Six) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::Zero, N::Seven) => {
            robot2(A, Up)
                + robot2(Up, Up)
                + robot2(Up, Up)
                + robot2(Up, Up)
                + robot2(Up, Left)
                + robot2(Left, A)
        }
        (N::Zero, N::Eight) => {
            robot2(A, Up) + robot2(Up, Up) + robot2(Up, Up) + robot2(Up, Up) + robot2(Up, A)
        }
        (N::Zero, N::Nine) => {
            robot2(A, Up)
                + robot2(Up, Up)
                + robot2(Up, Up)
                + robot2(Up, Up)
                + robot2(Up, Right)
                + robot2(Up, A)
        }
        (N::Zero, N::A) => robot2(A, Right) + robot2(Right, A),
        (N::One, N::Zero) => robot2(A, Right) + robot2(Right, Down) + robot2(Down, A),
        (N::One, N::One) => robot2(A, A),
        (N::One, N::Two) => robot2(A, Right) + robot2(Right, A),
        (N::One, N::Three) => robot2(A, Right) + robot2(Right, Right) + robot2(Right, A),
        (N::One, N::Four) => robot2(A, Up) + robot2(Up, A),
        (N::One, N::Five) => robot2(A, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::One, N::Six) => {
            robot2(A, Up) + robot2(Up, Right) + robot2(Right, Right) + robot2(Right, A)
        }
        (N::One, N::Seven) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, A),
        (N::One, N::Eight) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::One, N::Nine) => {
            robot2(A, Up)
                + robot2(Up, Up)
                + robot2(Up, Right)
                + robot2(Right, Right)
                + robot2(Right, A)
        }
        (N::One, N::A) => {
            robot2(A, Right) + robot2(Right, Right) + robot2(Right, Down) + robot2(Down, A)
        }
        (N::Two, N::Zero) => robot2(A, Down) + robot2(Down, A),
        (N::Two, N::One) => robot2(A, Left) + robot2(Left, A),
        (N::Two, N::Two) => robot2(A, A),
        (N::Two, N::Three) => robot2(A, Right) + robot2(Right, A),
        (N::Two, N::Four) => robot2(A, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Two, N::Five) => robot2(A, Up) + robot2(Up, A),
        (N::Two, N::Six) => robot2(A, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::Two, N::Seven) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Two, N::Eight) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, A),
        (N::Two, N::Nine) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::Two, N::A) => robot2(A, Right) + robot2(Right, Down) + robot2(Down, A),
        (N::Three, N::Zero) => robot2(A, Down) + robot2(Down, Left) + robot2(Left, A),
        (N::Three, N::One) => robot2(A, Left) + robot2(Left, Left) + robot2(Left, A),
        (N::Three, N::Two) => robot2(A, Left) + robot2(Left, A),
        (N::Three, N::Three) => robot2(A, A),
        (N::Three, N::Four) => {
            robot2(A, Left) + robot2(Left, Left) + robot2(Left, Up) + robot2(Up, A)
            // robot2(A, Up) + robot2(Up, Left) + robot2(Left, Left) + robot2(Left, A)
        }
        (N::Three, N::Five) => robot2(A, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Three, N::Six) => robot2(A, Up) + robot2(Up, A),
        (N::Three, N::Seven) => {
            robot2(A, Left) + robot2(Left, Left) + robot2(Left, Up) + robot2(Up, Up) + robot2(Up, A)
        }
        (N::Three, N::Eight) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Three, N::Nine) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, A),
        (N::Three, N::A) => robot2(A, Down) + robot2(Down, A),
        (N::Four, N::Zero) => {
            robot2(A, Right) + robot2(Right, Down) + robot2(Down, Down) + robot2(Down, A)
        }
        (N::Four, N::One) => robot2(A, Down) + robot2(Down, A),
        (N::Four, N::Two) => robot2(A, Right) + robot2(Right, Down) + robot2(Down, A),
        (N::Four, N::Three) => {
            robot2(A, Right) + robot2(Right, Right) + robot2(Right, Down) + robot2(Down, A)
        }
        (N::Four, N::Four) => robot2(A, A),
        (N::Four, N::Five) => robot2(A, Right) + robot2(Right, A),
        (N::Four, N::Six) => robot2(A, Right) + robot2(Right, Right) + robot2(Right, A),
        (N::Four, N::Seven) => robot2(A, Up) + robot2(Up, A),
        (N::Four, N::Eight) => robot2(A, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::Four, N::Nine) => {
            robot2(A, Up) + robot2(Up, Right) + robot2(Right, Right) + robot2(Right, A)
        }
        (N::Four, N::A) => {
            robot2(A, Right)
                + robot2(Right, Right)
                + robot2(Right, Down)
                + robot2(Down, Down)
                + robot2(Down, A)
        }
        (N::Five, N::Zero) => robot2(A, Down) + robot2(Down, Down) + robot2(Down, A),
        (N::Five, N::One) => robot2(A, Down) + robot2(Down, Left) + robot2(Left, A),
        (N::Five, N::Two) => robot2(A, Down) + robot2(Down, A),
        (N::Five, N::Three) => robot2(A, Right) + robot2(Right, Down) + robot2(Down, A),
        (N::Five, N::Four) => robot2(A, Left) + robot2(Left, A),
        (N::Five, N::Five) => robot2(A, A),
        (N::Five, N::Six) => robot2(A, Right) + robot2(Right, A),
        (N::Five, N::Seven) => robot2(A, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Five, N::Eight) => robot2(A, Up) + robot2(Up, A),
        (N::Five, N::Nine) => robot2(A, Up) + robot2(Up, Right) + robot2(Right, A),
        (N::Five, N::A) => {
            robot2(A, Right) + robot2(Right, Down) + robot2(Down, Down) + robot2(Down, A)
        }
        (N::Six, N::Zero) => {
            robot2(A, Down) + robot2(Down, Down) + robot2(Down, Left) + robot2(Left, A)
        }
        (N::Six, N::One) => {
            robot2(A, Down) + robot2(Down, Left) + robot2(Left, Left) + robot2(Left, A)
        }
        (N::Six, N::Two) => robot2(A, Down) + robot2(Down, Left) + robot2(Left, A),
        (N::Six, N::Three) => robot2(A, Down) + robot2(Down, A),
        (N::Six, N::Four) => robot2(A, Left) + robot2(Left, Left) + robot2(Left, A),
        (N::Six, N::Five) => robot2(A, Left) + robot2(Left, A),
        (N::Six, N::Six) => robot2(A, A),
        (N::Six, N::Seven) => {
            robot2(A, Up) + robot2(Up, Left) + robot2(Left, Left) + robot2(Left, A)
        }
        (N::Six, N::Eight) => robot2(A, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::Six, N::Nine) => robot2(A, Up) + robot2(Up, A),
        (N::Six, N::A) => robot2(A, Down) + robot2(Down, Down) + robot2(Down, A),
        (N::Seven, N::Zero) => {
            robot2(A, Right)
                + robot2(Right, Down)
                + robot2(Down, Down)
                + robot2(Down, Down)
                + robot2(Down, A)
        }
        (N::Seven, N::One) => robot2(A, Down) + robot2(Down, Down) + robot2(Down, A),
        (N::Seven, N::Two) => {
            robot2(A, Right) + robot2(Right, Down) + robot2(Down, Down) + robot2(Down, A)
        }
        (N::Seven, N::Three) => {
            robot2(A, Right)
                + robot2(Right, Right)
                + robot2(Right, Down)
                + robot2(Down, Down)
                + robot2(Down, A)
        }
        (N::Seven, N::Four) => robot2(A, Down) + robot2(Down, A),
        (N::Seven, N::Five) => robot2(A, Right) + robot2(Right, Down) + robot2(Right, A),
        (N::Seven, N::Six) => {
            robot2(A, Right) + robot2(Right, Right) + robot2(Right, Down) + robot2(Down, A)
        }
        (N::Seven, N::Seven) => robot2(A, A),
        (N::Seven, N::Eight) => robot2(A, Right) + robot2(Right, A),
        (N::Seven, N::Nine) => robot2(A, Right) + robot2(Right, Right) + robot2(Right, A),
        (N::Seven, N::A) => {
            robot2(A, Right)
                + robot2(Right, Right)
                + robot2(Right, Down)
                + robot2(Down, Down)
                + robot2(Down, Down)
                + robot2(Down, A)
        }
        (N::Eight, N::Zero) => {
            robot2(A, Down) + robot2(Down, Down) + robot2(Down, Down) + robot2(Down, A)
        }
        (N::Eight, N::One) => {
            robot2(A, Down) + robot2(Down, Down) + robot2(Down, Left) + robot2(Left, A)
        }
        (N::Eight, N::Two) => robot2(A, Down) + robot2(Down, Down) + robot2(Down, A),
        (N::Eight, N::Three) => {
            robot2(A, Right) + robot2(Right, Down) + robot2(Down, Down) + robot2(Down, A)
        }
        (N::Eight, N::Four) => robot2(A, Down) + robot2(Down, Left) + robot2(Down, A),
        (N::Eight, N::Five) => robot2(A, Down) + robot2(Down, A),
        (N::Eight, N::Six) => robot2(A, Right) + robot2(Right, Down) + robot2(Down, A),
        (N::Eight, N::Seven) => robot2(A, Left) + robot2(Left, A),
        (N::Eight, N::Eight) => robot2(A, A),
        (N::Eight, N::Nine) => robot2(A, Right) + robot2(Right, A),
        (N::Eight, N::A) => {
            robot2(A, Down)
                + robot2(Down, Down)
                + robot2(Down, Down)
                + robot2(Down, Right)
                + robot2(Right, A)
        }
        (N::Nine, N::Zero) => {
            robot2(A, Down)
                + robot2(Down, Down)
                + robot2(Down, Down)
                + robot2(Down, Left)
                + robot2(Left, A)
        }
        (N::Nine, N::One) => {
            robot2(A, Down)
                + robot2(Down, Down)
                + robot2(Down, Left)
                + robot2(Left, Left)
                + robot2(Left, A)
        }
        (N::Nine, N::Two) => {
            robot2(A, Down) + robot2(Down, Down) + robot2(Down, Left) + robot2(Left, A)
        }
        (N::Nine, N::Three) => robot2(A, Down) + robot2(Down, Down) + robot2(Down, A),
        (N::Nine, N::Four) => {
            robot2(A, Down) + robot2(Down, Left) + robot2(Left, Left) + robot2(Left, A)
        }
        (N::Nine, N::Five) => robot2(A, Down) + robot2(Down, Left) + robot2(Left, A),
        (N::Nine, N::Six) => robot2(A, Down) + robot2(Down, A),
        (N::Nine, N::Seven) => robot2(A, Left) + robot2(Left, Left) + robot2(Left, A),
        (N::Nine, N::Eight) => robot2(A, Left) + robot2(Left, A),
        (N::Nine, N::Nine) => robot2(A, A),
        (N::Nine, N::A) => {
            robot2(A, Down) + robot2(Down, Down) + robot2(Down, Down) + robot2(Down, A)
        }
        (N::A, N::Zero) => robot2(A, Left) + robot2(Left, A),
        (N::A, N::One) => robot2(A, Up) + robot2(Up, Left) + robot2(Left, Left) + robot2(Left, A),
        (N::A, N::Two) => robot2(A, Up) + robot2(Up, Left) + robot2(Left, A),
        (N::A, N::Three) => robot2(A, Up) + robot2(Up, A),
        (N::A, N::Four) => {
            robot2(A, Up) + robot2(Up, Up) + robot2(Up, Left) + robot2(Left, Left) + robot2(Left, A)
        }
        (N::A, N::Five) => robot2(A, Left) + robot2(Left, Up) + robot2(Up, Up) + robot2(Up, A),
        (N::A, N::Six) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, A),
        (N::A, N::Seven) => {
            robot2(A, Up)
                + robot2(Up, Up)
                + robot2(Up, Up)
                + robot2(Up, Left)
                + robot2(Left, Left)
                + robot2(Left, A)
        }
        (N::A, N::Eight) => {
            robot2(A, Up) + robot2(Up, Up) + robot2(Up, Up) + robot2(Up, Left) + robot2(Left, A)
        }
        (N::A, N::Nine) => robot2(A, Up) + robot2(Up, Up) + robot2(Up, Up) + robot2(Up, A),
        (N::A, N::A) => robot2(A, A),
    }
}

pub const fn robot2(start: DirKeypad, end: DirKeypad) -> u8 {
    use DirKeypad::*;
    match (start, end) {
        (Up, Up) => robot1(A, A),
        (Up, Down) => robot1(A, Down) + robot1(Down, A),
        (Up, Left) => robot1(A, Down) + robot1(Down, Left) + robot1(Left, A),
        (Up, Right) => robot1(A, Right) + robot1(Right, Down) + robot1(Down, A),
        (Up, A) => robot1(A, Right) + robot1(Right, A),
        (Down, Up) => robot1(A, Up) + robot1(Up, A),
        (Down, Down) => robot1(A, A),
        (Down, Left) => robot1(A, Left) + robot1(Left, A),
        (Down, Right) => robot1(A, Right) + robot1(Right, A),
        (Down, A) => robot1(A, Up) + robot1(Up, Right) + robot1(Right, A),
        (Left, Up) => robot1(A, Right) + robot1(Right, Up) + robot1(Up, A),
        (Left, Down) => robot1(A, Right) + robot1(Right, A),
        (Left, Left) => robot1(A, A),
        (Left, Right) => robot1(A, Right) + robot1(Right, Right) + robot1(Right, A),
        (Left, A) => robot1(A, Right) + robot1(Right, Right) + robot1(Right, Up) + robot1(Up, A),
        (Right, Up) => robot1(A, Up) + robot1(Up, Left) + robot1(Left, A),
        (Right, Down) => robot1(A, Left) + robot1(Left, A),
        (Right, Left) => robot1(A, Left) + robot1(Left, Left) + robot1(Left, A),
        (Right, Right) => robot1(A, A),
        (Right, A) => robot1(A, Up) + robot1(Up, A),
        (A, Up) => robot1(A, Left) + robot1(Left, A),
        (A, Down) => robot1(A, Down) + robot1(Down, Left) + robot1(Left, A),
        (A, Left) => robot1(A, Down) + robot1(Down, Left) + robot1(Left, Left) + robot1(Left, A),
        (A, Right) => robot1(A, Down) + robot1(Down, A),
        (A, A) => robot1(A, A),
    }
}

pub const fn robot1(start: DirKeypad, end: DirKeypad) -> u8 {
    use DirKeypad::*;
    match (start, end) {
        (Up, Up) => 1,
        (Up, Down) => 2,
        (Up, Left) => 3,
        (Up, Right) => 3,
        (Up, A) => 2,
        (Down, Up) => 2,
        (Down, Down) => 1,
        (Down, Left) => 2,
        (Down, Right) => 2,
        (Down, A) => 3,
        (Left, Up) => 3,
        (Left, Down) => 2,
        (Left, Left) => 1,
        (Left, Right) => 3,
        (Left, A) => 4,
        (Right, Up) => 3,
        (Right, Down) => 2,
        (Right, Left) => 3,
        (Right, Right) => 1,
        (Right, A) => 2,
        (A, Up) => 2,
        (A, Down) => 3,
        (A, Left) => 4,
        (A, Right) => 2,
        (A, A) => 1,
    }
}
