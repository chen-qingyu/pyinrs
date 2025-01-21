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

Int from                time:   [55.735 ns 55.917 ns 56.123 ns]
                        change: [+2.6766% +3.3196% +3.9220%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

Int +                   time:   [71.664 ns 72.004 ns 72.396 ns]
                        change: [-0.5599% +0.2542% +1.0177%] (p = 0.54 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Int -                   time:   [88.808 ns 89.144 ns 89.548 ns]
                        change: [-6.3946% -4.7916% -3.0982%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

Int *                   time:   [87.277 ns 87.578 ns 87.918 ns]
                        change: [+3.2316% +6.1930% +8.3912%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

Int /                   time:   [311.56 ns 312.57 ns 313.73 ns]
                        change: [+13.764% +15.119% +16.757%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high severe

Int %                   time:   [306.92 ns 307.89 ns 308.92 ns]
                        change: [+9.4470% +10.620% +11.611%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

Int gcd                 time:   [5.1418 µs 5.1770 µs 5.2167 µs]
                        change: [-42.007% -41.286% -40.582%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
*/
