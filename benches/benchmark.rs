use criterion::{criterion_group, criterion_main, Criterion};
use pyinrs::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let int1 = Int::from("987654321000");
    let int2 = Int::from("123456789000");

    c.bench_function("Int from", |b| b.iter(|| Int::from("987654321000")));
    c.bench_function("Int +", |b| b.iter(|| &int1 + &int2));
    c.bench_function("Int -", |b| b.iter(|| &int1 - &int2));
    c.bench_function("Int *", |b| b.iter(|| &int1 * &int2));
    c.bench_function("Int /", |b| b.iter(|| &int1 / &int2));
    // VecDeque -> Vec: ~30% faster

    c.bench_function("Int %", |b| b.iter(|| &int1 % &int2));
    // self - (self/rhs)*rhs -> flatten: ~10% faster

    c.bench_function("Int sqrt", |b| b.iter(|| Int::sqrt(&int1)));
    // cur_sqrt = integer/2 -> 10^(digits/2 - 1) in O(1): ~60% faster

    c.bench_function("Int gcd", |b| b.iter(|| Int::gcd(&int1, &int2)));
    // use in-place: ~20% faster for + - / %
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
