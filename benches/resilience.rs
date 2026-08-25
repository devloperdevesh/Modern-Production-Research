use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpr_performance_core::resilience::policy::ResiliencePolicy;

fn resilience_policy_healthy_request(c: &mut Criterion) {
    let policy = ResiliencePolicy::new(3, 100, 1000);

    c.bench_function("resilience_policy_healthy_request", |b| {
        b.iter(|| {
            black_box(policy.allow_request());
        });
    });
}

fn resilience_policy_failure_tracking(c: &mut Criterion) {
    c.bench_function("resilience_policy_failure_tracking", |b| {
        b.iter(|| {
            let policy = ResiliencePolicy::new(3, 100, 1000);

            policy.record_failure();
            black_box(policy.allow_request());
        });
    });
}

criterion_group!(
    resilience_benches,
    resilience_policy_healthy_request,
    resilience_policy_failure_tracking
);

criterion_main!(resilience_benches);
