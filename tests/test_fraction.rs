use pyinrs::Fraction;
use rstest::{fixture, rstest};

struct Fixture {
    zero: Fraction,
    positive: Fraction,
    negative: Fraction,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        zero: Fraction::new(),
        positive: Fraction::from((1, 2)),
        negative: Fraction::from((-1, 2)),
    }
}

#[rstest]
#[should_panic(expected = "Error: Divide by zero.")]
fn basics() {
    let _ = Fraction::from((1, 0));
}

#[rstest]
fn compare(setup: Fixture) {
    assert!(setup.zero == setup.zero);
    assert!(Fraction::from((9, 6)) == Fraction::from((3, 2)));

    assert!(setup.zero != setup.positive);
    assert!(setup.positive != setup.negative);

    assert!(setup.zero > setup.negative);
    assert!(Fraction::from((1, 2)) > Fraction::from((1, 3)));

    assert!(setup.zero < setup.positive);
    assert!(Fraction::from((1, 4)) < Fraction::from((1, 3)));

    assert!(setup.zero >= setup.zero);
    assert!(setup.positive >= setup.negative);

    assert!(setup.zero <= setup.zero);
    assert!(setup.negative <= setup.positive);
}

#[rstest]
fn copy(mut setup: Fixture) {
    setup.positive = setup.negative;
    assert_eq!(setup.positive, Fraction::from((-1, 2)));
    assert_eq!(setup.negative, Fraction::from((-1, 2)));
}

#[rstest]
fn examination(setup: Fixture) {
    assert_eq!(setup.zero.numerator(), 0);
    assert_eq!(setup.positive.numerator(), 1);
    assert_eq!(setup.negative.numerator(), -1);

    assert_eq!(setup.zero.denominator(), 1);
    assert_eq!(setup.positive.denominator(), 2);
    assert_eq!(setup.negative.denominator(), 2);
}

#[rstest]
fn unary(setup: Fixture) {
    assert_eq!(-setup.zero, Fraction::from(0));
    assert_eq!(-setup.positive, Fraction::from((-1, 2)));
    assert_eq!(-setup.negative, Fraction::from((1, 2)));

    assert_eq!(setup.zero.abs(), Fraction::from(0));
    assert_eq!(setup.positive.abs(), Fraction::from((1, 2)));
    assert_eq!(setup.negative.abs(), Fraction::from((1, 2)));
}

#[rstest]
fn add(setup: Fixture) {
    assert_eq!(setup.positive + setup.positive, Fraction::from(1));
    assert_eq!(setup.positive + setup.zero, Fraction::from((1, 2)));
    assert_eq!(setup.positive + setup.negative, Fraction::from(0));

    assert_eq!(setup.negative + setup.positive, Fraction::from(0));
    assert_eq!(setup.negative + setup.zero, Fraction::from((-1, 2)));
    assert_eq!(setup.negative + setup.negative, Fraction::from(-1));

    assert_eq!(setup.zero + setup.positive, Fraction::from((1, 2)));
    assert_eq!(setup.zero + setup.zero, Fraction::from(0));
    assert_eq!(setup.zero + setup.negative, Fraction::from((-1, 2)));
}

#[rstest]
fn sub(setup: Fixture) {
    assert_eq!(setup.positive - setup.positive, Fraction::from(0));
    assert_eq!(setup.positive - setup.zero, Fraction::from((1, 2)));
    assert_eq!(setup.positive - setup.negative, Fraction::from(1));

    assert_eq!(setup.negative - setup.positive, Fraction::from(-1));
    assert_eq!(setup.negative - setup.zero, Fraction::from((-1, 2)));
    assert_eq!(setup.negative - setup.negative, Fraction::from(0));

    assert_eq!(setup.zero - setup.positive, Fraction::from((-1, 2)));
    assert_eq!(setup.zero - setup.zero, Fraction::from(0));
    assert_eq!(setup.zero - setup.negative, Fraction::from((1, 2)));
}

#[rstest]
fn mul(setup: Fixture) {
    assert_eq!(setup.positive * setup.positive, Fraction::from((1, 4)));
    assert_eq!(setup.positive * setup.zero, Fraction::from(0));
    assert_eq!(setup.positive * setup.negative, Fraction::from((-1, 4)));

    assert_eq!(setup.negative * setup.positive, Fraction::from((-1, 4)));
    assert_eq!(setup.negative * setup.zero, Fraction::from(0));
    assert_eq!(setup.negative * setup.negative, Fraction::from((1, 4)));

    assert_eq!(setup.zero * setup.positive, Fraction::from(0));
    assert_eq!(setup.zero * setup.zero, Fraction::from(0));
    assert_eq!(setup.zero * setup.negative, Fraction::from(0));
}

#[rstest]
#[case::no_panic(1)]
#[should_panic(expected = "Error: Divide by zero.")]
#[case::panic_with_message(2)]
#[should_panic(expected = "Error: Divide by zero.")]
#[case::panic_with_message(3)]
#[should_panic(expected = "Error: Divide by zero.")]
#[case::panic_with_message(4)]
fn div(setup: Fixture, #[case] case: i32) {
    match case {
        1 => {
            assert_eq!(setup.positive / setup.positive, Fraction::from(1));
            // 2
            assert_eq!(setup.positive / setup.negative, Fraction::from(-1));

            assert_eq!(setup.negative / setup.positive, Fraction::from(-1));
            // 3
            assert_eq!(setup.negative / setup.negative, Fraction::from(1));

            assert_eq!(setup.zero / setup.positive, Fraction::from(0));
            // 4
            assert_eq!(setup.zero / setup.negative, Fraction::from(0));

            Fraction::new() // for compatible types
        }
        2 => setup.positive / setup.zero,
        3 => setup.negative / setup.zero,
        4 => setup.zero / setup.zero,
        _ => unreachable!(),
    };
}

#[rstest]
#[case::no_panic(1)]
#[should_panic(expected = "Error: Divide by zero.")]
#[case::panic_with_message(2)]
#[should_panic(expected = "Error: Divide by zero.")]
#[case::panic_with_message(3)]
#[should_panic(expected = "Error: Divide by zero.")]
#[case::panic_with_message(4)]
fn rem(setup: Fixture, #[case] case: i32) {
    match case {
        1 => {
            assert_eq!(setup.positive % setup.positive, Fraction::from(0));
            // 2
            assert_eq!(setup.positive % setup.negative, Fraction::from(0));

            assert_eq!(setup.negative % setup.positive, Fraction::from(0));
            // 3
            assert_eq!(setup.negative % setup.negative, Fraction::from(0));

            assert_eq!(setup.zero % setup.positive, Fraction::from(0));
            // 4
            assert_eq!(setup.zero % setup.negative, Fraction::from(0));

            Fraction::new() // for compatible types
        }
        2 => setup.positive % setup.zero,
        3 => setup.negative % setup.zero,
        4 => setup.zero % setup.zero,
        _ => unreachable!(),
    };
}

#[rstest]
fn from_integer() {
    let zero = Fraction::from(0);
    assert_eq!(zero.numerator(), 0);
    assert_eq!(zero.denominator(), 1);

    let one = Fraction::from(1);
    assert_eq!(one, Fraction::from(1i8));
    assert_eq!(one, Fraction::from(1i16));
    assert_eq!(one, Fraction::from(1i32));
    assert_eq!(one, Fraction::from(1i64));
    assert_eq!(one, Fraction::from(1i128));
    assert_eq!(one, Fraction::from(1u8));
    assert_eq!(one, Fraction::from(1u16));
    assert_eq!(one, Fraction::from(1u32));
    assert_eq!(one, Fraction::from(1u64));

    let small_pos = Fraction::from(1);
    assert!(small_pos.numerator() == 1);
    assert!(small_pos.denominator() == 1);

    let small_neg = Fraction::from(-1);
    assert_eq!(small_neg.numerator(), -1);
    assert_eq!(small_neg.denominator(), 1);

    let big_pos = Fraction::from(i128::MAX);
    assert_eq!(big_pos.numerator(), i128::MAX);
    assert_eq!(big_pos.denominator(), 1);

    let big_neg = Fraction::from(i128::MIN);
    assert_eq!(big_neg.numerator(), i128::MIN);
    assert_eq!(big_neg.denominator(), 1);

    let reduced = Fraction::from((12, -4));
    assert_eq!(reduced, Fraction::from((-3, 1)));

    let unity = Fraction::from((42, 42));
    assert_eq!(unity, Fraction::from(1));
}

#[rstest]
fn from_float() {
    assert_eq!(Fraction::from(0.0), Fraction::from((0, 1)));
    assert_eq!(Fraction::from(1.1), Fraction::from((11, 10)));
    assert_eq!(Fraction::from(0.75), Fraction::from((3, 4)));
    assert_eq!(Fraction::from(-1.2), Fraction::from((-6, 5)));
    assert_eq!(Fraction::from(std::f64::consts::PI), Fraction::from((3141592653589793i128, 1000000000000000)));
}

#[rstest]
fn to_float() {
    let zero = Fraction::from((0, 2));
    let half = Fraction::from((1, 2));
    let thirds = Fraction::from((2, 3));
    let neg_half = Fraction::from((1, -2));

    assert_eq!(f64::from(zero), 0.0);
    assert_eq!(f64::from(half), 0.5);
    assert_eq!(f64::from(thirds), 2.0 / 3.0);
    assert_eq!(f64::from(neg_half), -0.5);

    assert_eq!(f32::from(zero), 0.0);
    assert_eq!(f32::from(half), 0.5);
    assert_eq!(f32::from(thirds), 2.0 / 3.0);
    assert_eq!(f32::from(neg_half), -0.5);
}

#[rstest]
fn gcd_lcm() {
    // gcd()
    assert_eq!(Fraction::gcd(Fraction::from(0), Fraction::from(0)), Fraction::from(0));
    assert_eq!(Fraction::gcd(Fraction::from(0), Fraction::from(1)), Fraction::from(1));
    assert_eq!(Fraction::gcd(Fraction::from(1), Fraction::from(0)), Fraction::from(1));
    assert_eq!(Fraction::gcd(Fraction::from(1), Fraction::from(1)), Fraction::from(1));

    assert_eq!(Fraction::gcd(Fraction::from((1, 2)), Fraction::from((3, 4))), Fraction::from((1, 4)));
    assert_eq!(Fraction::gcd(Fraction::from((3, 4)), Fraction::from((1, 6))), Fraction::from((1, 12)));
    assert_eq!(Fraction::gcd(Fraction::from((233, 2333)), Fraction::from((7, 77))), Fraction::from((1, 25663)));
    assert_eq!(Fraction::gcd(Fraction::from((-1, 2)), Fraction::from((-3, 4))), Fraction::from((1, 4)));

    // lcm()
    assert_eq!(Fraction::lcm(Fraction::from(0), Fraction::from(0)), Fraction::from(0));
    assert_eq!(Fraction::lcm(Fraction::from(0), Fraction::from(1)), Fraction::from(0));
    assert_eq!(Fraction::lcm(Fraction::from(1), Fraction::from(0)), Fraction::from(0));
    assert_eq!(Fraction::lcm(Fraction::from(1), Fraction::from(1)), Fraction::from(1));

    assert_eq!(Fraction::lcm(Fraction::from((1, 2)), Fraction::from((3, 4))), Fraction::from((3, 2)));
    assert_eq!(Fraction::lcm(Fraction::from((3, 4)), Fraction::from((1, 6))), Fraction::from((3, 2)));
    assert_eq!(Fraction::lcm(Fraction::from((233, 2333)), Fraction::from((7, 77))), Fraction::from(233));
    assert_eq!(Fraction::lcm(Fraction::from((-1, 2)), Fraction::from((-3, 4))), Fraction::from((3, 2)));
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.zero), "0");
    assert_eq!(format!("{}", setup.positive), "1/2");
    assert_eq!(format!("{}", setup.negative), "-1/2");
}

#[rstest]
fn parse(setup: Fixture) {
    assert_eq!(setup.zero, "0".parse().unwrap());
    assert_eq!(setup.positive, "1/2".parse().unwrap());
    assert_eq!(setup.negative, "-1/2".parse().unwrap());

    assert_eq!(setup.zero, "  000\n\n".parse().unwrap());
    assert_eq!(setup.positive, "  -2/-4\t\n".parse().unwrap());
    assert_eq!(setup.negative, "\t3/-6\n\n".parse().unwrap());

    assert!("z1/2".parse::<Fraction>().is_err());
    assert!("1z/2".parse::<Fraction>().is_err());
    assert!("1/z2".parse::<Fraction>().is_err());
    assert!("1/2z".parse::<Fraction>().is_err());
    assert!("1|2".parse::<Fraction>().is_err());
}
