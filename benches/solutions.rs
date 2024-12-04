use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

fn day_1_1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    let mut c = c.benchmark_group("day_1_1");
    c.bench_function("naive", |b| {
        b.iter(|| aoc2024::day1::part1_naive(black_box(input)))
    });
    c.bench_function("universal", |b| {
        b.iter(|| aoc2024::day1::part1_universal(black_box(input.as_bytes())))
    });
    c.bench_function("simd", |b| {
        b.iter(|| aoc2024::day1::part1(black_box(input)))
    });
}
fn day_1_2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    let mut c = c.benchmark_group("day_1_2");
    c.bench_function("naive", |b| {
        b.iter(|| aoc2024::day1::part2_naive(black_box(input)))
    });
    c.bench_function("universal", |b| {
        b.iter(|| aoc2024::day1::part2_universal(black_box(input.as_bytes())))
    });
    c.bench_function("simd", |b| {
        b.iter(|| aoc2024::day1::part2(black_box(input)))
    });
}

fn day_2_1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");
    let mut c = c.benchmark_group("day_2_1");
    c.bench_function("naive", |b| {
        b.iter(|| aoc2024::day2::part1_naive(black_box(input)))
    });
    c.bench_function("opt", |b| {
        b.iter(|| aoc2024::day2::part1_opt(black_box(input.as_bytes())))
    });
    // FIXME: This is fails somehow.
    //
    // c.bench_function("no_vec", |b| {
    //     b.iter(|| aoc2024::day2::part1_no_vec(black_box(input.as_bytes())))
    // });
}
fn day_2_2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");
    let mut c = c.benchmark_group("day_2_2");
    c.bench_function("naive", |b| {
        b.iter(|| aoc2024::day2::part2_naive(black_box(input)))
    });
    c.bench_function("opt", |b| {
        b.iter(|| aoc2024::day2::part2_opt(black_box(input.as_bytes())))
    });
    c.bench_function("single_pass", |b| {
        b.iter(|| aoc2024::day2::part2_single_pass(black_box(input.as_bytes())))
    });
}

fn day_3_1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day3.txt");
    let mut c = c.benchmark_group("day_3_1");
    c.bench_function("naive", |b| {
        b.iter(|| aoc2024::day3::part1_naive(black_box(input.as_bytes())))
    });
    c.bench_function("backwards", |b| {
        b.iter(|| aoc2024::day3::part1_backwards(black_box(input.as_bytes())))
    });
    c.bench_function("memchr", |b| {
        b.iter(|| aoc2024::day3::part1_memchr(black_box(input.as_bytes())))
    });
}
fn day_3_2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day3.txt");
    let mut c = c.benchmark_group("day_3_2");
    c.bench_function("backwards", |b| {
        b.iter(|| aoc2024::day3::part2_backwards(black_box(input.as_bytes())))
    });
    c.bench_function("memchr", |b| {
        b.iter(|| aoc2024::day3::part2_memchr(black_box(input.as_bytes())))
    });
}

fn day_4_1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day4.txt");
    let mut c = c.benchmark_group("day_4_1");
    // FIXME: There is a bug.
    //
    // c.bench_function("naive", |b| {
    //     b.iter(|| aoc2024::day4::part1_naive(black_box(input.as_bytes())))
    // });
    // c.bench_function("multithreaded", |b| {
    //     b.iter(|| aoc2024::day4::part1_mt(black_box(input.as_bytes())))
    // });
}
fn day_4_2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day4.txt");
    let mut c = c.benchmark_group("day_4_2");
    c.bench_function("naive", |b| {
        b.iter(|| aoc2024::day4::part2_naive(black_box(input.as_bytes())))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = day_1_1, day_1_2,
              day_2_1, day_2_2,
              day_3_1, day_3_2,
              day_4_1, day_4_2,
}
criterion_main!(benches);
