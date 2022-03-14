use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shanksbot_rs::shanks;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("shanks 60017", |b| b.iter(|| shanks(black_box(60017))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
