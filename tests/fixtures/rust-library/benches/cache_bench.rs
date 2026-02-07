use criterion::{criterion_group, criterion_main, Criterion};
use fastcache::Cache;
use std::time::Duration;

fn bench_insert(c: &mut Criterion) {
    let cache = Cache::new(10000, Duration::from_secs(60));
    c.bench_function("insert", |b| {
        b.iter(|| cache.insert("key", "value"))
    });
}

fn bench_get(c: &mut Criterion) {
    let cache = Cache::new(10000, Duration::from_secs(60));
    cache.insert("key", "value");
    c.bench_function("get", |b| {
        b.iter(|| cache.get(&"key"))
    });
}

criterion_group!(benches, bench_insert, bench_get);
criterion_main!(benches);
