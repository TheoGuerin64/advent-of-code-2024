use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

use advent_of_code_2024::{parse_input_file, step1, step2};

fn bench_steps(c: &mut Criterion) {
    let data = parse_input_file(Path::new("input.txt")).unwrap();

    let mut group = c.benchmark_group("steps");
    group.bench_function("step1", |b| {
        b.iter(|| step1(&data));
    });
    group.bench_function("step2", |b| {
        b.iter(|| step2(&data));
    });
    group.finish();
}

criterion_group!(benches, bench_steps);
criterion_main!(benches);
