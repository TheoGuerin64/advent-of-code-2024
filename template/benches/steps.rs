use std::fs::read_to_string;

use criterion::{criterion_group, criterion_main, Criterion};

use advent_of_code_2024::{step1, step2};

fn bench_steps(c: &mut Criterion) {
    let input = read_to_string("input.txt").expect("Input file not found");

    let mut group = c.benchmark_group("steps");
    group.bench_function("step1", |b| {
        b.iter(|| step1(&input));
    });
    group.bench_function("step2", |b| {
        b.iter(|| step2(&input));
    });
    group.finish();
}

criterion_group!(benches, bench_steps);
criterion_main!(benches);
