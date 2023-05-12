use criterion::{black_box, criterion_group, criterion_main, Criterion};

use AdventOfCode::y2017::*;

const YEAR: usize = 2017;

fn criterion_benchmark_1(c: &mut Criterion) {
    let input = Day01::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day01::name()), |b| {
        b.iter(|| Day01::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day01::name()), |b| {
        b.iter(|| Day01::part2(black_box(&input)))
    });
}

fn criterion_benchmark_2(c: &mut Criterion) {
    let input = Day02::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day02::name()), |b| {
        b.iter(|| Day02::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day02::name()), |b| {
        b.iter(|| Day02::part2(black_box(&input)))
    });
}

fn criterion_benchmark_3(c: &mut Criterion) {
    let input = Day03::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day03::name()), |b| {
        b.iter(|| Day03::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day03::name()), |b| {
        b.iter(|| Day03::part2(black_box(&input)))
    });
}

fn criterion_benchmark_4(c: &mut Criterion) {
    let input = Day04::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day04::name()), |b| {
        b.iter(|| Day04::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day04::name()), |b| {
        b.iter(|| Day04::part2(black_box(&input)))
    });
}

fn criterion_benchmark_5(c: &mut Criterion) {
    let input = Day05::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day05::name()), |b| {
        b.iter(|| Day05::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day05::name()), |b| {
        b.iter(|| Day05::part2(black_box(&input)))
    });
}

fn criterion_benchmark_6(c: &mut Criterion) {
    let input = Day06::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day06::name()), |b| {
        b.iter(|| Day06::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day06::name()), |b| {
        b.iter(|| Day06::part2(black_box(&input)))
    });
}

fn criterion_benchmark_7(c: &mut Criterion) {
    let input = Day07::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day07::name()), |b| {
        b.iter(|| Day07::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day07::name()), |b| {
        b.iter(|| Day07::part2(black_box(&input)))
    });
}

fn criterion_benchmark_8(c: &mut Criterion) {
    let input = Day08::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day08::name()), |b| {
        b.iter(|| Day08::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day08::name()), |b| {
        b.iter(|| Day08::part2(black_box(&input)))
    });
}

fn criterion_benchmark_9(c: &mut Criterion) {
    let input = Day09::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day09::name()), |b| {
        b.iter(|| Day09::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day09::name()), |b| {
        b.iter(|| Day09::part2(black_box(&input)))
    });
}

fn criterion_benchmark_10(c: &mut Criterion) {
    let input = Day10::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day10::name()), |b| {
        b.iter(|| Day10::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day10::name()), |b| {
        b.iter(|| Day10::part2(black_box(&input)))
    });
}

fn criterion_benchmark_11(c: &mut Criterion) {
    let input = Day11::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day11::name()), |b| {
        b.iter(|| Day11::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day11::name()), |b| {
        b.iter(|| Day11::part2(black_box(&input)))
    });
}

fn criterion_benchmark_12(c: &mut Criterion) {
    let input = Day12::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day12::name()), |b| {
        b.iter(|| Day12::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day12::name()), |b| {
        b.iter(|| Day12::part2(black_box(&input)))
    });
}

fn criterion_benchmark_13(c: &mut Criterion) {
    let input = Day13::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day13::name()), |b| {
        b.iter(|| Day13::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day13::name()), |b| {
        b.iter(|| Day13::part2(black_box(&input)))
    });
}

fn criterion_benchmark_14(c: &mut Criterion) {
    let input = Day14::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day14::name()), |b| {
        b.iter(|| Day14::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day14::name()), |b| {
        b.iter(|| Day14::part2(black_box(&input)))
    });
}

fn criterion_benchmark_15(c: &mut Criterion) {
    let input = Day15::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day15::name()), |b| {
        b.iter(|| Day15::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day15::name()), |b| {
        b.iter(|| Day15::part2(black_box(&input)))
    });
}

fn criterion_benchmark_16(c: &mut Criterion) {
    let input = Day16::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day16::name()), |b| {
        b.iter(|| Day16::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day16::name()), |b| {
        b.iter(|| Day16::part2(black_box(&input)))
    });
}

fn criterion_benchmark_17(c: &mut Criterion) {
    let input = Day17::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day17::name()), |b| {
        b.iter(|| Day17::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day17::name()), |b| {
        b.iter(|| Day17::part2(black_box(&input)))
    });
}

fn criterion_benchmark_18(c: &mut Criterion) {
    let input = Day18::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day18::name()), |b| {
        b.iter(|| Day18::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day18::name()), |b| {
        b.iter(|| Day18::part2(black_box(&input)))
    });
}

fn criterion_benchmark_19(c: &mut Criterion) {
    let input = Day19::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day19::name()), |b| {
        b.iter(|| Day19::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day19::name()), |b| {
        b.iter(|| Day19::part2(black_box(&input)))
    });
}

fn criterion_benchmark_20(c: &mut Criterion) {
    let input = Day20::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day20::name()), |b| {
        b.iter(|| Day20::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day20::name()), |b| {
        b.iter(|| Day20::part2(black_box(&input)))
    });
}

fn criterion_benchmark_21(c: &mut Criterion) {
    let input = Day21::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day21::name()), |b| {
        b.iter(|| Day21::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day21::name()), |b| {
        b.iter(|| Day21::part2(black_box(&input)))
    });
}

fn criterion_benchmark_22(c: &mut Criterion) {
    let input = Day22::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day22::name()), |b| {
        b.iter(|| Day22::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day22::name()), |b| {
        b.iter(|| Day22::part2(black_box(&input)))
    });
}

fn criterion_benchmark_23(c: &mut Criterion) {
    let input = Day23::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day23::name()), |b| {
        b.iter(|| Day23::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day23::name()), |b| {
        b.iter(|| Day23::part2(black_box(&input)))
    });
}

fn criterion_benchmark_24(c: &mut Criterion) {
    let input = Day24::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day24::name()), |b| {
        b.iter(|| Day24::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day24::name()), |b| {
        b.iter(|| Day24::part2(black_box(&input)))
    });
}

fn criterion_benchmark_25(c: &mut Criterion) {
    let input = Day25::input();

    c.bench_function(&format!("{}::{}::part1", YEAR, Day25::name()), |b| {
        b.iter(|| Day25::part1(black_box(&input)))
    });

    c.bench_function(&format!("{}::{}::part2", YEAR, Day25::name()), |b| {
        b.iter(|| Day25::part2(black_box(&input)))
    });
}

criterion_group!(
    benches,
    criterion_benchmark_1,
    criterion_benchmark_2,
    criterion_benchmark_3,
    criterion_benchmark_4,
    criterion_benchmark_5,
    criterion_benchmark_6,
    criterion_benchmark_7,
    criterion_benchmark_8,
    criterion_benchmark_9,
    criterion_benchmark_10,
    criterion_benchmark_11,
    criterion_benchmark_12,
    criterion_benchmark_13,
    criterion_benchmark_14,
    criterion_benchmark_15,
    criterion_benchmark_16,
    criterion_benchmark_17,
    criterion_benchmark_18,
    criterion_benchmark_19,
    criterion_benchmark_20,
    criterion_benchmark_21,
    criterion_benchmark_22,
    criterion_benchmark_23,
    criterion_benchmark_24,
    criterion_benchmark_25,
);
criterion_main!(benches);
