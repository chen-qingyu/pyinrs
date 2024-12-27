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

Int from                time:   [118.00 ns 118.18 ns 118.37 ns]
                        change: [-3.1437% +0.9289% +4.5842%] (p = 0.66 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

Int +                   time:   [152.56 ns 152.94 ns 153.41 ns]
                        change: [-11.453% -10.533% -8.9234%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

Int -                   time:   [214.18 ns 214.64 ns 215.17 ns]
                        change: [-11.621% -11.167% -10.724%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

Int *                   time:   [156.24 ns 156.57 ns 156.95 ns]
                        change: [-38.594% -38.191% -37.776%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  8 (8.00%) high mild
  7 (7.00%) high severe

Int /                   time:   [635.94 ns 637.71 ns 640.14 ns]
                        change: [-11.953% -11.339% -10.710%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

Int %                   time:   [716.16 ns 730.71 ns 746.32 ns]
                        change: [-7.4528% -4.2331% -0.8918%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Int gcd                 time:   [20.971 µs 21.026 µs 21.089 µs]
                        change: [-26.840% -26.364% -25.859%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe
*/
