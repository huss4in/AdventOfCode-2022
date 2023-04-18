use criterion::{black_box, criterion_group, criterion_main, Criterion};

use AdventOfCode::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day2::part1", |b| {
        b.iter(|| Day2::part1(black_box(Day2::input())))
    });
    c.bench_function("Day2::part2", |b| {
        b.iter(|| Day2::part2(black_box(Day2::input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
