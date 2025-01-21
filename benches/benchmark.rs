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
Result (2025.01.22, Windows 10, rustc v1.84.0, criterion v0.5):

Int from                time:   [53.963 ns 54.182 ns 54.461 ns]
                        change: [-2.4689% -1.4791% -0.5583%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

Int +                   time:   [71.268 ns 71.630 ns 72.060 ns]
                        change: [-0.8048% +0.2725% +1.3195%] (p = 0.62 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

Int -                   time:   [92.950 ns 93.519 ns 94.309 ns]
                        change: [+1.2044% +2.3581% +3.7778%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

Int *                   time:   [80.863 ns 81.244 ns 81.678 ns]
                        change: [-0.4182% +1.8167% +4.7052%] (p = 0.20 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

Int /                   time:   [272.39 ns 273.55 ns 274.98 ns]
                        change: [-0.9057% +0.1059% +1.0153%] (p = 0.83 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Int %                   time:   [273.72 ns 275.00 ns 276.52 ns]
                        change: [-0.2162% +0.9383% +2.1602%] (p = 0.13 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Int gcd                 time:   [8.7302 µs 8.7921 µs 8.8695 µs]
                        change: [+1.3157% +2.2769% +3.2556%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
*/
