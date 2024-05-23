use pyinrs::Complex;
use rstest::{fixture, rstest};

struct Fixture {
    zero: Complex,
    positive: Complex,
    negative: Complex,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        zero: Complex::new(),
        positive: Complex::from((1., 2.)),
        negative: Complex::from((-1., 2.)),
    }
}

#[rstest]
fn compare(setup: Fixture) {
    assert!(setup.zero == setup.zero);
    assert!(setup.positive == setup.positive);

    assert!(setup.zero != setup.positive);
    assert!(setup.positive != setup.negative);
}

#[rstest]
fn copy(mut setup: Fixture) {
    setup.positive = setup.negative;
    assert_eq!(setup.positive, Complex::from((-1., 2.)));
    assert_eq!(setup.negative, Complex::from((-1., 2.)));
}

#[rstest]
fn examination(setup: Fixture) {
    assert_eq!(setup.zero.real(), 0.);
    assert_eq!(setup.positive.real(), 1.);
    assert_eq!(setup.negative.real(), -1.);

    assert_eq!(setup.zero.imag(), 0.);
    assert_eq!(setup.positive.imag(), 2.);
    assert_eq!(setup.negative.imag(), 2.);

    assert_eq!(setup.zero.abs(), 0.);
    assert_eq!(setup.positive.abs(), 2.23606797749979);
    assert_eq!(setup.negative.abs(), 2.23606797749979);

    assert_eq!(setup.zero.arg(), 0.);
    assert_eq!(setup.positive.arg(), 1.1071487177940904);
    assert_eq!(setup.negative.arg(), 2.0344439357957027);
}

#[rstest]
fn unary(setup: Fixture) {
    assert_eq!(-setup.zero, Complex::new());
    assert_eq!(-setup.positive, Complex::from((-1., -2.)));
    assert_eq!(-setup.negative, Complex::from((1., -2.)));

    assert_eq!(setup.zero.conjugate(), Complex::new());
    assert_eq!(setup.positive.conjugate(), Complex::from((1., -2.)));
    assert_eq!(setup.negative.conjugate(), Complex::from((-1., -2.)));
}

#[rstest]
fn add(setup: Fixture) {
    assert_eq!(setup.positive + setup.positive, Complex::from((2., 4.)));
    assert_eq!(setup.positive + setup.zero, Complex::from((1., 2.)));
    assert_eq!(setup.positive + setup.negative, Complex::from((0., 4.)));

    assert_eq!(setup.negative + setup.positive, Complex::from((0., 4.)));
    assert_eq!(setup.negative + setup.zero, Complex::from((-1., 2.)));
    assert_eq!(setup.negative + setup.negative, Complex::from((-2., 4.)));

    assert_eq!(setup.zero + setup.positive, Complex::from((1., 2.)));
    assert_eq!(setup.zero + setup.zero, Complex::from(0.));
    assert_eq!(setup.zero + setup.negative, Complex::from((-1., 2.)));
}

#[rstest]
fn sub(setup: Fixture) {
    assert_eq!(setup.positive - setup.positive, Complex::from(0.));
    assert_eq!(setup.positive - setup.zero, Complex::from((1., 2.)));
    assert_eq!(setup.positive - setup.negative, Complex::from(2.));

    assert_eq!(setup.negative - setup.positive, Complex::from(-2.));
    assert_eq!(setup.negative - setup.zero, Complex::from((-1., 2.)));
    assert_eq!(setup.negative - setup.negative, Complex::from(0.));

    assert_eq!(setup.zero - setup.positive, Complex::from((-1., -2.)));
    assert_eq!(setup.zero - setup.zero, Complex::from(0.));
    assert_eq!(setup.zero - setup.negative, Complex::from((1., -2.)));
}

#[rstest]
fn mul(setup: Fixture) {
    assert_eq!(setup.positive * setup.positive, Complex::from((-3., 4.)));
    assert_eq!(setup.positive * setup.zero, Complex::from(0.));
    assert_eq!(setup.positive * setup.negative, Complex::from(-5.));

    assert_eq!(setup.negative * setup.positive, Complex::from(-5.));
    assert_eq!(setup.negative * setup.zero, Complex::from(0.));
    assert_eq!(setup.negative * setup.negative, Complex::from((-3., -4.)));

    assert_eq!(setup.zero * setup.positive, Complex::from(0.));
    assert_eq!(setup.zero * setup.zero, Complex::from(0.));
    assert_eq!(setup.zero * setup.negative, Complex::from(0.));
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
            assert_eq!(setup.positive / setup.positive, Complex::from(1.));
            // 2
            assert_eq!(setup.positive / setup.negative, Complex::from((0.6, -0.8)));

            assert_eq!(setup.negative / setup.positive, Complex::from((0.6, 0.8)));
            // 3
            assert_eq!(setup.negative / setup.negative, Complex::from(1.));

            assert_eq!(setup.zero / setup.positive, Complex::from(0.));
            // 4
            assert_eq!(setup.zero / setup.negative, Complex::from(0.));

            Complex::new() // for compatible types
        }
        2 => setup.positive / setup.zero,
        3 => setup.negative / setup.zero,
        4 => setup.zero / setup.zero,
        _ => unreachable!(),
    };
}

#[rstest]
fn pow(setup: Fixture) {
    assert_eq!(Complex::pow(&setup.positive, &setup.zero), Complex::from(1.));
    assert_eq!(
        Complex::pow(&setup.positive, &setup.positive),
        Complex::from((-0.22251715680177267, 0.10070913113607541))
    );
    assert_eq!(
        Complex::pow(&setup.positive, &setup.negative),
        Complex::from((0.04281551979798478, 0.023517649351954585))
    );

    assert_eq!(Complex::pow(&setup.negative, &setup.zero), Complex::from(1.));
    assert_eq!(
        Complex::pow(&setup.negative, &setup.positive),
        Complex::from((-0.0335067906880002, -0.018404563532749985))
    );
    assert_eq!(
        Complex::pow(&setup.negative, &setup.negative),
        Complex::from((0.006965545047800022, -0.0031525388861500334))
    );

    assert_eq!(Complex::pow(&setup.zero, &setup.zero), Complex::from(1.));
}

#[rstest]
#[should_panic(expected = "Error: Math domain error.")]
fn bad_pow(setup: Fixture) {
    let _ = Complex::pow(&setup.zero, &setup.positive);
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.zero), "(0+0j)");
    assert_eq!(format!("{}", setup.positive), "(1+2j)");
    assert_eq!(format!("{}", setup.negative), "(-1+2j)");

    assert_eq!(format!("{}", Complex::from((1., -2.))), "(1-2j)");
}

#[rstest]
fn parse() {
    assert_eq!(Complex::from((1., -2.)), "+1-2j".parse().unwrap());
    assert_eq!(Complex::from(233.33), "233.33".parse().unwrap());
    assert_eq!(Complex::from((-1234., -4321.)), "-1234-4321j".parse().unwrap());
    assert_eq!(Complex::from((0., 3.)), "   3j   ".parse().unwrap());

    assert!("z1+2j".parse::<Complex>().is_err());
    assert!("1z+2j".parse::<Complex>().is_err());
    assert!("1+z2j".parse::<Complex>().is_err());
    assert!("1+2zj".parse::<Complex>().is_err());
    assert!("123jj".parse::<Complex>().is_err());
}
