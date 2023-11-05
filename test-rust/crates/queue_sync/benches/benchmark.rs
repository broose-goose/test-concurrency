use criterion::{black_box, criterion_group, criterion_main, Criterion};
use queue_sync::std_mpsc;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("std_mpsc", |b| b.iter(|| std_mpsc(black_box(100_000))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);