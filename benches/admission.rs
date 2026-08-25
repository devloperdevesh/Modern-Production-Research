use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpr_performance_core::backpressure::QueueGuard;
use mpr_performance_core::load_shedding::LoadShedder;

fn benchmark_queue_guard(c: &mut Criterion) {
    let guard = QueueGuard::new(1024);

    c.bench_function("queue_guard_evaluate", |b| {
        b.iter(|| {
            black_box(guard.evaluate(512));
        });
    });
}

fn benchmark_queue_capacity(c: &mut Criterion) {
    let guard = QueueGuard::new(1024);

    c.bench_function("queue_guard_remaining_capacity", |b| {
        b.iter(|| {
            black_box(guard.remaining_capacity(512));
        });
    });
}

fn benchmark_load_shedder(c: &mut Criterion) {
    let shedder = LoadShedder::new(1024);

    c.bench_function("load_shedder_evaluate", |b| {
        b.iter(|| {
            black_box(shedder.evaluate(512));
        });
    });
}

criterion_group!(
    benches,
    benchmark_queue_guard,
    benchmark_queue_capacity,
    benchmark_load_shedder
);

criterion_main!(benches);
