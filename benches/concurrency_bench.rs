use criterion::{criterion_group, criterion_main, Criterion};
use mpr_performance_core::rate_limit::token_bucket::TokenBucket;

fn bench_atomic_token_acquisition(c: &mut Criterion) {
    let bucket = TokenBucket::new(10_000, 500);

    c.bench_function("atomic_cas_acquire", |b| {
        b.iter(|| bucket.acquire(1));
    });
}

criterion_group!(benches, bench_atomic_token_acquisition);
criterion_main!(benches);
