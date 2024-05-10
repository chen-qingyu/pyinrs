use criterion::{criterion_group, criterion_main, Criterion};
use pyinrs::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let int1 = Int::from("987654321000");
    let int2 = Int::from("123456789000");

    c.bench_function("Int from", |b| b.iter(|| Int::from("123456789000")));
    c.bench_function("Int +", |b| b.iter(|| &int1 + &int2));
    c.bench_function("Int -", |b| b.iter(|| &int1 - &int2));
    c.bench_function("Int *", |b| b.iter(|| &int1 * &int2));
    c.bench_function("Int /", |b| b.iter(|| &int1 / &int2));

    // VecDeque -> Vec: almost 30% faster
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
