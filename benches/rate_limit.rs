use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpr_performance_core::rate_limit::token_bucket::TokenBucket;

fn benchmark_token_bucket_acquire(c: &mut Criterion) {
    let bucket = TokenBucket::new(u64::MAX, 0);

    c.bench_function("token_bucket_acquire", |b| {
        b.iter(|| {
            black_box(bucket.acquire(1));
        });
    });
}

fn benchmark_token_bucket_available(c: &mut Criterion) {
    let bucket = TokenBucket::new(u64::MAX, 0);

    c.bench_function("token_bucket_available", |b| {
        b.iter(|| {
            black_box(bucket.available());
        });
    });
}

criterion_group!(
    benches,
    benchmark_token_bucket_acquire,
    benchmark_token_bucket_available
);

criterion_main!(benches);
