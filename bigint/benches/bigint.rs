use bigint::bigint::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from usize", |b| {
        b.iter(|| BigInt::from_value(black_box(578437695752307201usize)))
    });

    c.bench_function("add usize", |b| {
        let bigint = BigInt::from_value(578437695752307201usize);
        b.iter(|| &bigint + &bigint)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
