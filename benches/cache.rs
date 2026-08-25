use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpr_performance_core::cache::local_cache::LocalCache;

fn benchmark_cache_get(c: &mut Criterion) {
    let cache = LocalCache::new(1024);
    let key = String::from("hot-key");

    cache.insert(key.clone(), 42u64);

    c.bench_function("local_cache_get", |b| {
        b.iter(|| {
            black_box(cache.get(&key));
        });
    });
}

fn benchmark_cache_insert(c: &mut Criterion) {
    c.bench_function("local_cache_insert", |b| {
        b.iter(|| {
            let cache = LocalCache::new(1024);
            black_box(cache.insert(String::from("hot-key"), 42u64));
        });
    });
}

fn benchmark_cache_remove(c: &mut Criterion) {
    c.bench_function("local_cache_remove", |b| {
        b.iter(|| {
            let cache = LocalCache::new(1024);
            cache.insert(String::from("hot-key"), 42u64);
            black_box(cache.remove(&String::from("hot-key")));
        });
    });
}

fn benchmark_cache_len(c: &mut Criterion) {
    let cache = LocalCache::new(1024);
    cache.insert(String::from("hot-key"), 42u64);

    c.bench_function("local_cache_len", |b| {
        b.iter(|| {
            black_box(cache.len());
        });
    });
}

fn benchmark_cache_is_empty(c: &mut Criterion) {
    let cache: LocalCache<String, u64> = LocalCache::new(1024);

    c.bench_function("local_cache_is_empty", |b| {
        b.iter(|| {
            black_box(cache.is_empty());
        });
    });
}

criterion_group!(
    benches,
    benchmark_cache_get,
    benchmark_cache_insert,
    benchmark_cache_remove,
    benchmark_cache_len,
    benchmark_cache_is_empty
);

criterion_main!(benches);
