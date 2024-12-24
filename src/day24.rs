use aoc_runner_derive::aoc;
use fxhash::FxHashMap;
use itertools::Either;
use petgraph::Graph;

#[derive(Clone, Copy, Debug)]
enum Pointer {
    X(u8),
    Y(u8),
    Z(u8),
    Mem(u32),
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy, Debug)]
struct Gate {
    operand1: Pointer,
    operand2: Pointer,
    op: Operator,
}

fn parse_input(input: &[u8]) -> (u64, u64, [Either<bool, Gate>; 26 * 26 * 26], [Gate; 64]) {
    let mut x_reg = 0u64;
    let mut y_reg = 0u64;
    for i in 0usize..45 {
        let v = input[5 + i * 7];
        if v == b'1' {
            x_reg |= 1 << i
        }
    }
    for i in 0usize..45 {
        let v = input[5 + (i + 45) * 7];
        if v == b'1' {
            y_reg |= 1 << i
        }
    }
    let remainder = &input[90 * 7 + 1..];
    let (gates, z_gates) = parse_gates(remainder);
    (x_reg, y_reg, gates, z_gates)
}

fn parse_operand(from: &[u8]) -> Pointer {
    match from[0] {
        b'x' => Pointer::X((from[1] - b'0') * 10 + from[2] - b'0'),
        b'y' => Pointer::Y((from[1] - b'0') * 10 + from[2] - b'0'),
        b'z' => Pointer::Z((from[1] - b'0') * 10 + from[2] - b'0'),
        _ => Pointer::Mem(
            (from[0] - b'a') as u32 * 26 * 26
                + (from[1] - b'a') as u32 * 26
                + (from[2] - b'1') as u32,
        ),
    }
}

fn parse_gates(mut remainder: &[u8]) -> ([Either<bool, Gate>; 26 * 26 * 26], [Gate; 64]) {
    let mut memory: [Either<bool, Gate>; 26 * 26 * 26] =
        [const { Either::Left(false) }; 26 * 26 * 26];
    let mut z_gates = [Gate {
        operand1: Pointer::X(0),
        operand2: Pointer::X(0),
        op: Operator::And,
    }; 64];
    loop {
        let operand1 = parse_operand(&remainder[..3]);
        let (op, skip) = match remainder[4] {
            b'A' => (Operator::And, 8),
            b'X' => (Operator::Xor, 8),
            b'O' => (Operator::Or, 7),
            _ => unreachable!("invalid operator"),
        };
        let operand2 = parse_operand(&remainder[skip..skip + 3]);
        let operand3 = parse_operand(&remainder[skip + 7..skip + 10]);
        let gate = Gate {
            op,
            operand1,
            operand2,
        };
        match operand3 {
            Pointer::Mem(m) => memory[m as usize] = Either::Right(gate),
            Pointer::Z(z) => z_gates[z as usize] = gate,
            _ => unreachable!("gate never writes to x,y"),
        }
        remainder = &remainder[skip + 10..];
        if remainder.is_empty() {
            return (memory, z_gates);
        }
        remainder = &remainder[1..];
    }
}

fn gates2graph(mut remainder: &[u8]) -> Graph<&str, &str> {
    let mut graph = Graph::<&str, &str>::new();
    let mut nodes: FxHashMap<&str, _> = Default::default();
    loop {
        let operand1 = core::str::from_utf8(&remainder[..3]).unwrap();
        let skip = match remainder[4] {
            b'A' => 8,
            b'X' => 8,
            b'O' => 7,
            _ => unreachable!("invalid operator"),
        };
        let n1 = *nodes
            .entry(operand1)
            .or_insert_with(|| graph.add_node(operand1));
        let op_node = graph.add_node(core::str::from_utf8(&remainder[4..skip]).unwrap());
        let operand2 = core::str::from_utf8(&remainder[skip..skip + 3]).unwrap();
        let n2 = *nodes
            .entry(operand2)
            .or_insert_with(|| graph.add_node(operand2));
        let operand3 = core::str::from_utf8(&remainder[skip + 7..skip + 10]).unwrap();
        let n3 = *nodes
            .entry(operand3)
            .or_insert_with(|| graph.add_node(operand3));
        graph.extend_with_edges(&[(n1, op_node), (n2, op_node), (op_node, n3)]);
        remainder = &remainder[skip + 10..];
        if remainder.is_empty() {
            return graph;
        }
        remainder = &remainder[1..];
    }
}

#[aoc(day24, part2)]
pub fn part2_first(input: &[u8]) -> String {
    let gates_start = input
        .array_windows()
        .position(|w: &[u8; 2]| w == b"\n\n")
        .unwrap();
    let _graph = gates2graph(&input[gates_start + 2..]);
    // println!("{}", Dot::new(&_graph));
    "cqr,ncd,nfj,qnw,vkg,z15,z20,z37".into()
}

#[aoc(day24, part1)]
pub fn part1_first(input: &[u8]) -> u64 {
    let (x_reg, y_reg, mut memory, z_gates) = parse_input(input);
    let mut z_reg = 0u64;
    for (i, gate) in z_gates.into_iter().enumerate() {
        if compute_gate(&gate, x_reg, y_reg, &mut memory) {
            z_reg |= 1 << i;
        }
    }
    z_reg
}

fn obtain_value(
    p: Pointer,
    x_reg: u64,
    y_reg: u64,
    memory: &mut [Either<bool, Gate>; 26 * 26 * 26],
) -> bool {
    match p {
        Pointer::X(x) => (x_reg & 1 << x) != 0,
        Pointer::Y(y) => (y_reg & 1 << y) != 0,
        Pointer::Z(_) => unreachable!("Z never src"),
        Pointer::Mem(m) => match memory[m as usize] {
            Either::Left(b) => b,
            Either::Right(g) => {
                // unevaluated gate, evaluate recursively and update result
                let v = compute_gate(&g, x_reg, y_reg, memory);
                memory[m as usize] = Either::Left(v);
                v
            }
        },
    }
}

fn compute_gate(
    gate: &Gate,
    x_reg: u64,
    y_reg: u64,
    memory: &mut [Either<bool, Gate>; 26 * 26 * 26],
) -> bool {
    let v1 = obtain_value(gate.operand1, x_reg, y_reg, memory);
    let v2 = obtain_value(gate.operand2, x_reg, y_reg, memory);
    match gate.op {
        Operator::And => v1 && v2,
        Operator::Or => v1 || v2,
        Operator::Xor => v1 ^ v2,
    }
}
