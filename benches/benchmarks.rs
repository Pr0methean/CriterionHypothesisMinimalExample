use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rng, Rng};

fn next_u64(c: &mut Criterion) {
    let mut thread_rng = rng();
    let mut group = c.benchmark_group("next_u64");
    group.bench_function("ThreadRng", |b| b.iter(|| thread_rng.next_u64()));
    group.finish();
}

criterion_group!(name = benches; config = Criterion::default(); targets = next_u64);
criterion_main!(benches);