use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpr_performance_core::concurrency::semaphore::Semaphore;

fn benchmark_semaphore_acquire_release(c: &mut Criterion) {
    let semaphore = Semaphore::new(1);

    c.bench_function("semaphore_acquire_release", |b| {
        b.iter(|| {
            black_box(semaphore.try_acquire());
            semaphore.release();
        });
    });
}

fn benchmark_semaphore_available_permits(c: &mut Criterion) {
    let semaphore = Semaphore::new(1024);

    c.bench_function("semaphore_available_permits", |b| {
        b.iter(|| {
            black_box(semaphore.available_permits());
        });
    });
}

criterion_group!(
    benches,
    benchmark_semaphore_acquire_release,
    benchmark_semaphore_available_permits
);

criterion_main!(benches);
