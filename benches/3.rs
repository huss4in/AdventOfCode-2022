use criterion::{black_box, criterion_group, criterion_main, Criterion};

use AdventOfCode::{Challenge, Day3 as Day};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(&format!("{}::part1", Day::name()), |b| {
        b.iter(|| Day::part1(black_box(Day::input())))
    });

    c.bench_function(&format!("{}::part2", Day::name()), |b| {
        b.iter(|| Day::part2(black_box(Day::input())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
