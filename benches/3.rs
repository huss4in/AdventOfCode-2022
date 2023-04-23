use criterion::{black_box, criterion_group, criterion_main, Criterion};

use AdventOfCode::{Challenge, Day3 as Day};

fn criterion_benchmark(c: &mut Criterion) {
    let input = Day::input();

    c.bench_function(&format!("{}::part1", Day::name()), |b| {
        b.iter(|| Day::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::part2", Day::name()), |b| {
        b.iter(|| Day::part2(black_box(&input)))
    });

    c.bench_function(&format!("{}::common", Day::name()), |b| {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        b.iter(|| Day::common(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
