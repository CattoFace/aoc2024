use std::array::from_fn;

use aoc_runner_derive::aoc;
use bitvec::prelude::*;
use fxhash::FxHashSet;
use itertools::Itertools;
use tinyvec::ArrayVec;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    part1_first(&input[..input.len() - 1])
}

pub fn part2(input: &str) -> String {
    let input = input.as_bytes();
    part2_brute_force(&input[..input.len() - 1])
}

fn brute_force_clique_adj(
    neighbours: [ArrayVec<[u32; 15]>; 26 * 26],
    neighbours_adj: BitArr!(for 26*26*26*26),
) -> Vec<u32> {
    for (c1, n1) in neighbours.iter().enumerate() {
        // looking specifically for cliques of size 13, vertices with less than 12 neighbours are
        // irrelevant
        if n1.len() < 12 {
            continue;
        }
        // iterate over all combinations of 12 neighbours
        let potential_cliques = n1
            .iter()
            .filter(|&&n| neighbours[n as usize].len() >= 12)
            .copied()
            .combinations(12);
        for mut pc in potential_cliques {
            // check if the current combination forms a clique
            if pc
                .iter()
                .tuple_combinations()
                .all(|(&a, &b)| neighbours_adj[(a * 26 * 26 + b) as usize])
            {
                pc.push(c1 as u32);
                return pc;
            }
        }
    }
    unreachable!()
}

fn brute_force_clique(neighbours: [FxHashSet<usize>; 26 * 26]) -> Vec<usize> {
    for (c1, n1) in neighbours.iter().enumerate() {
        // looking specifically for cliques of size 13, vertices with less than 12 neighbours are
        // irrelevant
        if n1.len() < 12 {
            continue;
        }
        // iterate over all combinations of 12 neighbours
        let potential_cliques = n1
            .iter()
            .filter(|n| neighbours[**n].len() >= 12)
            .copied()
            .combinations(12);
        for mut pc in potential_cliques {
            // check if the current combination forms a clique
            if pc
                .iter()
                .tuple_combinations()
                .all(|(a, b)| neighbours[*a].contains(b))
            {
                pc.push(c1);
                return pc;
            }
        }
    }
    unreachable!()
}

fn bron_kerbosch(
    included: &mut Vec<usize>,
    mut potential: FxHashSet<usize>,
    neighbours: &[FxHashSet<usize>; 26 * 26],
) -> Vec<usize> {
    if potential.is_empty() {
        return included.clone();
    }
    let mut max_set: Vec<usize> = Default::default();
    let curr_potential = potential.clone();
    for &vertex in curr_potential.iter() {
        included.push(vertex);
        let s = bron_kerbosch(
            included,
            potential
                .intersection(&neighbours[vertex])
                .copied()
                .collect(),
            neighbours,
        );
        included.pop();
        if s.len() > max_set.len() {
            max_set = s;
        }
        potential.remove(&vertex);
    }
    max_set
}

#[aoc(day23, part2, adj)]
pub fn part2_brute_adj(input: &[u8]) -> String {
    let mut neighbours: [ArrayVec<[u32; 15]>; 26 * 26] = from_fn(|_| Default::default());
    let mut neighbours_adj = bitarr![0usize;26*26*26*26];
    input.chunks(6).for_each(|line: &[u8]| {
        let comp1 = (line[0] - b'a') as u32 * 26 + (line[1] - b'a') as u32;
        let comp2 = (line[3] - b'a') as u32 * 26 + (line[4] - b'a') as u32;
        let adj1 = comp1 * 26 * 26 + comp2;
        let adj2 = comp2 * 26 * 26 + comp1;
        neighbours[comp1 as usize].push(comp2);
        neighbours[comp2 as usize].push(comp1);
        neighbours_adj.set(adj1 as usize, true);
        neighbours_adj.set(adj2 as usize, true);
    });
    let max_set = brute_force_clique_adj(neighbours, neighbours_adj);
    max_set
        .iter()
        .sorted()
        .map(|&comp| {
            String::from_utf8([(comp / 26) as u8 + b'a', (comp % 26) as u8 + b'a'].to_vec())
                .unwrap()
        })
        .join(",")
}

#[aoc(day23, part2, brute_force)]
pub fn part2_brute_force(input: &[u8]) -> String {
    let mut neighbours: [FxHashSet<usize>; 26 * 26] = from_fn(|_| Default::default());
    input.chunks(6).for_each(|line: &[u8]| {
        let comp1 = (line[0] - b'a') as usize * 26 + (line[1] - b'a') as usize;
        let comp2 = (line[3] - b'a') as usize * 26 + (line[4] - b'a') as usize;
        neighbours[comp1].insert(comp2);
        neighbours[comp2].insert(comp1);
    });
    let max_set = brute_force_clique(neighbours);
    max_set
        .iter()
        .sorted()
        .map(|&comp| {
            String::from_utf8([(comp / 26) as u8 + b'a', (comp % 26) as u8 + b'a'].to_vec())
                .unwrap()
        })
        .join(",")
}

#[aoc(day23, part2)]
pub fn part2_first(input: &[u8]) -> String {
    let mut neighbours: [FxHashSet<usize>; 26 * 26] = from_fn(|_| Default::default());
    input.chunks(6).for_each(|line: &[u8]| {
        let comp1 = (line[0] - b'a') as usize * 26 + (line[1] - b'a') as usize;
        let comp2 = (line[3] - b'a') as usize * 26 + (line[4] - b'a') as usize;
        neighbours[comp1].insert(comp2);
        neighbours[comp2].insert(comp1);
    });
    let potential = neighbours
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if n.is_empty() { None } else { Some(i) })
        .collect();
    let max_set = bron_kerbosch(&mut Default::default(), potential, &neighbours);
    max_set
        .iter()
        .sorted()
        .map(|&comp| {
            String::from_utf8([(comp / 26) as u8 + b'a', (comp % 26) as u8 + b'a'].to_vec())
                .unwrap()
        })
        .join(",")
}

#[aoc(day23, part1, hashset)]
pub fn part1_hashset(input: &[u8]) -> usize {
    let mut neighbours: [FxHashSet<usize>; 26 * 26] = from_fn(|_| Default::default());
    input.chunks(6).for_each(|line: &[u8]| {
        // rotate letters to make t the smallest, for de-duplication later
        let c11 = if line[0] < b't' {
            line[0] + 26 - b't'
        } else {
            line[0] - b't'
        };
        let c12 = if line[1] < b't' {
            line[1] + 26 - b't'
        } else {
            line[1] - b't'
        };
        let c21 = if line[3] < b't' {
            line[3] + 26 - b't'
        } else {
            line[3] - b't'
        };
        let c22 = if line[4] < b't' {
            line[4] + 26 - b't'
        } else {
            line[4] - b't'
        };
        let comp1 = c11 as usize * 26 + c12 as usize;
        let comp2 = c21 as usize * 26 + c22 as usize;
        // as a de-duplication policy, connections will be increasing IDs
        if comp1 <= comp2 {
            neighbours[comp1].insert(comp2);
        } else {
            neighbours[comp2].insert(comp1);
        }
    });
    // only check computers that start with t
    let mut count = 0usize;
    for neighs in &neighbours[..26] {
        for &comp2 in neighs {
            count += neighs.intersection(&neighbours[comp2]).count();
        }
    }
    count
}

#[aoc(day23, part1)]
pub fn part1_first(input: &[u8]) -> u32 {
    let mut count = 0u32;
    let mut neighbours: [ArrayVec<[usize; 15]>; 26 * 26] = from_fn(|_| Default::default());
    input.chunks(6).for_each(|line: &[u8]| {
        // rotate letters to make t the smallest, for de-duplication later
        let c11 = if line[0] < b't' {
            line[0] + 26 - b't'
        } else {
            line[0] - b't'
        };
        let c12 = if line[1] < b't' {
            line[1] + 26 - b't'
        } else {
            line[1] - b't'
        };
        let c21 = if line[3] < b't' {
            line[3] + 26 - b't'
        } else {
            line[3] - b't'
        };
        let c22 = if line[4] < b't' {
            line[4] + 26 - b't'
        } else {
            line[4] - b't'
        };
        let comp1 = c11 as usize * 26 + c12 as usize;
        let comp2 = c21 as usize * 26 + c22 as usize;
        // as a de-duplication policy, connections will be increasing IDs
        if comp1 <= comp2 {
            neighbours[comp1].push(comp2);
        } else {
            neighbours[comp2].push(comp1);
        }
    });
    // only check computers that start with t
    for (comp1, neighs) in neighbours[..26].iter().enumerate() {
        for &comp2 in neighs {
            let neighs2 = &neighbours[comp2];
            for &comp3 in neighs2 {
                if neighbours[comp1].contains(&comp3) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::day23::{part1_first, part2_first};

    #[test]
    fn sample_part2() {
        assert_eq!(
            part2_first(
                b"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
            ),
            "co,de,ka,ta"
        );
    }

    #[test]
    fn sample_part1() {
        assert_eq!(
            part1_first(
                b"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
            ),
            7
        );
    }
}
