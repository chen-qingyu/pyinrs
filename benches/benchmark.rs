use criterion::{criterion_group, criterion_main, Criterion};
use pyinrs::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let int1 = Int::from("987654321000000000000000000");
    let int2 = Int::from("123456789123456789123456789");

    c.bench_function("Int from", |b| b.iter(|| Int::from("123456789123456789123456789")));

    c.bench_function("Int +", |b| b.iter(|| &int1 + &int2));
    c.bench_function("Int -", |b| b.iter(|| &int1 - &int2));
    c.bench_function("Int *", |b| b.iter(|| &int1 * &int2));
    c.bench_function("Int /", |b| b.iter(|| &int1 / &int2));
    c.bench_function("Int %", |b| b.iter(|| &int1 % &int2));

    c.bench_function("Int gcd", |b| b.iter(|| Int::gcd(&int1, &int2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/*
Result (2024.12.27, Windows 10, rustc v1.80.0, criterion v0.5):

Int from                time:   [109.63 ns 110.17 ns 110.88 ns]
                        change: [+27.543% +32.898% +39.405%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 19 outliers among 100 measurements (19.00%)
  3 (3.00%) high mild
  16 (16.00%) high severe

Int +                   time:   [172.22 ns 172.46 ns 172.70 ns]
                        change: [-7.1230% -6.7329% -6.3369%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

Int -                   time:   [241.15 ns 241.98 ns 243.46 ns]
                        change: [-1.3173% -0.7494% -0.1750%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

Int *                   time:   [252.65 ns 252.94 ns 253.30 ns]
                        change: [-91.106% -91.063% -91.007%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe

Int /                   time:   [715.18 ns 718.97 ns 726.30 ns]
                        change: [-61.460% -61.194% -60.914%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe

Int %                   time:   [739.69 ns 751.35 ns 766.43 ns]
                        change: [-61.581% -60.479% -59.393%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

Int gcd                 time:   [28.489 µs 28.533 µs 28.583 µs]
                        change: [-31.391% -30.884% -30.395%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
*/
