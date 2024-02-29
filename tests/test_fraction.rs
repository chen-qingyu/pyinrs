use pyinrs::Fraction;
use rstest::{fixture, rstest};

#[rstest]
#[case::no_panic(1)]
#[case::no_panic(2)]
#[should_panic(expected = "Error: Zero denominator.")]
#[case::panic_with_message(3)]
fn basics(#[case] case: i32) {
    match case {
        1 => Fraction::new(),
        2 => Fraction::from(2, 3),
        3 => Fraction::from(1, 0),
        _ => unreachable!(),
    };
}

struct Fixture {
    zero: Fraction,
    positive: Fraction,
    negative: Fraction,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        zero: Fraction::new(),
        positive: Fraction::from(1, 2),
        negative: Fraction::from(-1, 2),
    }
}

#[rstest]
fn compare(setup: Fixture) {
    assert!(setup.zero == setup.zero);
    assert!(Fraction::from(9, 6) == Fraction::from(3, 2));

    assert!(setup.zero != setup.positive);
    assert!(setup.positive != setup.negative);

    assert!(setup.zero > setup.negative);
    assert!(Fraction::from(1, 2) > Fraction::from(1, 3));

    assert!(setup.zero < setup.positive);
    assert!(Fraction::from(1, 4) < Fraction::from(1, 3));

    assert!(setup.zero >= setup.zero);
    assert!(setup.zero >= setup.negative);

    assert!(setup.zero <= setup.zero);
    assert!(setup.zero <= setup.positive);
}

#[rstest]
fn copy(mut setup: Fixture) {
    setup.positive = setup.negative;
    assert_eq!(setup.positive, Fraction::from(-1, 2));
    assert_eq!(setup.negative, Fraction::from(-1, 2));
}

#[rstest]
fn to_decimal() {
    assert_eq!(Fraction::from(0, 2).to_decimal(), 0.0);
    assert_eq!(Fraction::from(1, 2).to_decimal(), 0.5);
    assert_eq!(Fraction::from(2, 3).to_decimal(), 2.0 / 3.0);
    assert_eq!(Fraction::from(1, -2).to_decimal(), -0.5);
}

#[rstest]
fn unary(setup: Fixture) {
    assert_eq!(-setup.positive, Fraction::from(-1, 2));
    assert_eq!(setup.positive.abs(), Fraction::from(1, 2));

    assert_eq!(-setup.negative, Fraction::from(1, 2));
    assert_eq!(setup.negative.abs(), Fraction::from(1, 2));
}

#[rstest]
fn add(setup: Fixture) {
    assert_eq!(setup.positive + setup.positive, Fraction::from(1, 1));
    assert_eq!(setup.positive + setup.zero, Fraction::from(1, 2));
    assert_eq!(setup.positive + setup.negative, Fraction::from(0, 1));

    assert_eq!(setup.negative + setup.positive, Fraction::from(0, 1));
    assert_eq!(setup.negative + setup.zero, Fraction::from(-1, 2));
    assert_eq!(setup.negative + setup.negative, Fraction::from(-1, 1));

    assert_eq!(setup.zero + setup.positive, Fraction::from(1, 2));
    assert_eq!(setup.zero + setup.zero, Fraction::from(0, 1));
    assert_eq!(setup.zero + setup.negative, Fraction::from(-1, 2));
}

#[rstest]
fn sub(setup: Fixture) {
    assert_eq!(setup.positive - setup.positive, Fraction::from(0, 1));
    assert_eq!(setup.positive - setup.zero, Fraction::from(1, 2));
    assert_eq!(setup.positive - setup.negative, Fraction::from(1, 1));

    assert_eq!(setup.negative - setup.positive, Fraction::from(-1, 1));
    assert_eq!(setup.negative - setup.zero, Fraction::from(-1, 2));
    assert_eq!(setup.negative - setup.negative, Fraction::from(0, 1));

    assert_eq!(setup.zero - setup.positive, Fraction::from(-1, 2));
    assert_eq!(setup.zero - setup.zero, Fraction::from(0, 1));
    assert_eq!(setup.zero - setup.negative, Fraction::from(1, 2));
}

#[rstest]
fn mul(setup: Fixture) {
    assert_eq!(setup.positive * setup.positive, Fraction::from(1, 4));
    assert_eq!(setup.positive * setup.zero, Fraction::from(0, 1));
    assert_eq!(setup.positive * setup.negative, Fraction::from(-1, 4));

    assert_eq!(setup.negative * setup.positive, Fraction::from(-1, 4));
    assert_eq!(setup.negative * setup.zero, Fraction::from(0, 1));
    assert_eq!(setup.negative * setup.negative, Fraction::from(1, 4));

    assert_eq!(setup.zero * setup.positive, Fraction::from(0, 1));
    assert_eq!(setup.zero * setup.zero, Fraction::from(0, 1));
    assert_eq!(setup.zero * setup.negative, Fraction::from(0, 1));
}

#[rstest]
#[case::no_panic(1)]
#[should_panic(expected = "Error: Zero denominator.")]
#[case::panic_with_message(2)]
#[should_panic(expected = "Error: Zero denominator.")]
#[case::panic_with_message(3)]
#[should_panic(expected = "Error: Zero denominator.")]
#[case::panic_with_message(4)]
fn div(setup: Fixture, #[case] case: i32) {
    match case {
        1 => {
            assert_eq!(setup.positive / setup.positive, Fraction::from(1, 1));
            // 2
            assert_eq!(setup.positive / setup.negative, Fraction::from(-1, 1));

            assert_eq!(setup.negative / setup.positive, Fraction::from(-1, 1));
            // 3
            assert_eq!(setup.negative / setup.negative, Fraction::from(1, 1));

            assert_eq!(setup.zero / setup.positive, Fraction::from(0, 1));
            // 4
            assert_eq!(setup.zero / setup.negative, Fraction::from(0, 1));

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
#[should_panic(expected = "Error: Zero denominator.")]
#[case::panic_with_message(2)]
#[should_panic(expected = "Error: Zero denominator.")]
#[case::panic_with_message(3)]
#[should_panic(expected = "Error: Zero denominator.")]
#[case::panic_with_message(4)]
fn rem(setup: Fixture, #[case] case: i32) {
    match case {
        1 => {
            assert_eq!(setup.positive % setup.positive, Fraction::from(0, 1));
            // 2
            assert_eq!(setup.positive % setup.negative, Fraction::from(0, 1));

            assert_eq!(setup.negative % setup.positive, Fraction::from(0, 1));
            // 3
            assert_eq!(setup.negative % setup.negative, Fraction::from(0, 1));

            assert_eq!(setup.zero % setup.positive, Fraction::from(0, 1));
            // 4
            assert_eq!(setup.zero % setup.negative, Fraction::from(0, 1));

            Fraction::new() // for compatible types
        }
        2 => setup.positive % setup.zero,
        3 => setup.negative % setup.zero,
        4 => setup.zero % setup.zero,
        _ => unreachable!(),
    };
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.zero), "0");
    assert_eq!(format!("{}", setup.positive), "1/2");
    assert_eq!(format!("{}", setup.negative), "-1/2");
}