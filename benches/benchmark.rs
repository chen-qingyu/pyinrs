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

    c.bench_function("Int fac", |b| b.iter(|| Int::from("200").factorial()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/*
Result (2025.05.11, Windows 11, rustc v1.86.0, criterion v0.5):

Int from                time:   [57.716 ns 58.356 ns 59.093 ns]
                        change: [-1.6683% -0.3267% +1.1638%] (p = 0.65 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

Int +                   time:   [70.342 ns 70.484 ns 70.633 ns]
                        change: [-4.4015% -3.3011% -2.3843%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe

Int -                   time:   [89.547 ns 89.818 ns 90.092 ns]
                        change: [-6.6720% -5.5570% -4.4864%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe

Int *                   time:   [103.05 ns 103.32 ns 103.61 ns]
                        change: [-1.9676% -1.2194% -0.4863%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Int /                   time:   [360.29 ns 361.37 ns 362.43 ns]
                        change: [-5.5842% -3.9236% -2.6187%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

Int %                   time:   [363.12 ns 364.76 ns 366.79 ns]
                        change: [-2.3493% -1.6269% -0.9174%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

Int gcd                 time:   [5.6824 µs 5.7057 µs 5.7369 µs]
                        change: [-2.6096% -1.7693% -1.0215%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

Int fac                 time:   [18.696 µs 18.746 µs 18.801 µs]
                        change: [-40.968% -40.535% -40.081%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
*/
