use aoc9::aoc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("my_benchmark", |b| b.iter(|| aoc()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
