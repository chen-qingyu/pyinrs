use pyinrs::Int;
use rstest::{fixture, rstest};

struct Fixture {
    zero: Int,
    positive: Int,
    negative: Int,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        zero: Int::new(),
        positive: Int::from("18446744073709551617"),  // 2^64+1
        negative: Int::from("-18446744073709551617"), // -(2^64+1)
    }
}

#[rstest]
fn basics() {
    let int1 = Int::new();
    assert_eq!(int1.digits(), 0);
    assert!(int1.is_zero());

    let int2 = Int::from("123456789000");
    assert_eq!(int2.digits(), 12);
    assert!(!int2.is_zero());

    let int3 = Int::from(123456789);
    assert_eq!(int3.digits(), 9);
    assert!(!int3.is_zero());

    let int4 = Int::default();
    assert_eq!(int4.digits(), 0);
    assert!(int4.is_zero());
}

#[rstest]
#[should_panic(expected = "Error: Wrong integer literal.")]
fn bad_from() {
    let _ = Int::from("hello");
}

#[rstest]
fn compare(setup: Fixture) {
    // operator==
    assert!(setup.zero == setup.zero);
    assert!(setup.positive == setup.positive);
    assert!(setup.negative == setup.negative);

    // operator!=
    assert!(setup.zero != setup.positive);
    assert!(setup.zero != setup.negative);

    // operator<
    assert!(setup.negative < setup.zero);
    assert!(setup.negative < setup.positive);

    // operator<=
    assert!(setup.negative <= setup.zero);
    assert!(setup.negative <= setup.positive);
    assert!(setup.negative <= setup.negative);

    // operator>
    assert!(setup.positive > setup.zero);
    assert!(setup.positive > setup.negative);

    // operator>=
    assert!(setup.positive >= setup.zero);
    assert!(setup.positive >= setup.negative);
    assert!(setup.positive >= setup.positive);
}

#[rstest]
fn examination(setup: Fixture) {
    // digits()
    assert_eq!(setup.zero.digits(), 0);
    assert_eq!(setup.positive.digits(), 20);
    assert_eq!(setup.negative.digits(), 20);

    // is_zero()
    assert!(setup.zero.is_zero());
    assert!(!setup.positive.is_zero());
    assert!(!setup.negative.is_zero());

    // is_positive()
    assert!(!setup.zero.is_positive());
    assert!(setup.positive.is_positive());
    assert!(!setup.negative.is_positive());

    // is_negative()
    assert!(!setup.zero.is_negative());
    assert!(!setup.positive.is_negative());
    assert!(setup.negative.is_negative());

    // is_even()
    assert!(setup.zero.is_even());
    assert!(!setup.positive.is_even());
    assert!(!setup.negative.is_even());

    // is_odd()
    assert!(!setup.zero.is_odd());
    assert!(setup.positive.is_odd());
    assert!(setup.negative.is_odd());
}

#[rstest]
fn is_prime() {
    assert!(!Int::from("-1").is_prime());
    assert!(!Int::from("0").is_prime());
    assert!(!Int::from("1").is_prime());
    assert!(Int::from("2").is_prime());
    assert!(Int::from("3").is_prime());
    assert!(!Int::from("4").is_prime());
    assert!(Int::from("5").is_prime());
    assert!(!Int::from("6").is_prime());
    assert!(Int::from("7").is_prime());
    assert!(!Int::from("8").is_prime());
    assert!(!Int::from("9").is_prime());
    assert!(!Int::from("10").is_prime());

    assert!(Int::from("2147483629").is_prime()); // maximum prime number that < i32::MAX
    assert!(Int::from("2147483647").is_prime()); // i32::MAX is a prime number
    assert!(Int::from("2147483659").is_prime()); // minimum prime number that > i32::MAX
}

#[rstest]
fn inc_dec() {
    // inc()
    assert_eq!(Int::from("-1").inc(), &Int::from("0"));
    assert_eq!(Int::from("0").inc(), &Int::from("1"));
    assert_eq!(Int::from("1").inc(), &Int::from("2"));
    assert_eq!(Int::from("99999999999999").inc(), &Int::from("100000000000000"));

    // dec()
    assert_eq!(Int::from("-1").dec(), &Int::from("-2"));
    assert_eq!(Int::from("0").dec(), &Int::from("-1"));
    assert_eq!(Int::from("1").dec(), &Int::from("0"));
    assert_eq!(Int::from("100000000000000").dec(), &Int::from("99999999999999"));
}

#[rstest]
fn add(setup: Fixture) {
    assert_eq!(&setup.positive + &setup.positive, Int::from("36893488147419103234"));
    assert_eq!(&setup.positive + &setup.zero, Int::from("18446744073709551617"));
    assert_eq!(&setup.positive + &setup.negative, Int::from("0"));

    assert_eq!(&setup.negative + &setup.positive, Int::from("0"));
    assert_eq!(&setup.negative + &setup.zero, Int::from("-18446744073709551617"));
    assert_eq!(&setup.negative + &setup.negative, Int::from("-36893488147419103234"));

    assert_eq!(&setup.zero + &setup.positive, Int::from("18446744073709551617"));
    assert_eq!(&setup.zero + &setup.zero, Int::from("0"));
    assert_eq!(&setup.zero + &setup.negative, Int::from("-18446744073709551617"));

    assert_eq!(Int::from(99999) + Int::from(1), 100000.into());
}

#[rstest]
fn sub(setup: Fixture) {
    assert_eq!(&setup.positive - &setup.positive, Int::from("0"));
    assert_eq!(&setup.positive - &setup.zero, Int::from("18446744073709551617"));
    assert_eq!(&setup.positive - &setup.negative, Int::from("36893488147419103234"));

    assert_eq!(&setup.negative - &setup.positive, Int::from("-36893488147419103234"));
    assert_eq!(&setup.negative - &setup.zero, Int::from("-18446744073709551617"));
    assert_eq!(&setup.negative - &setup.negative, Int::from("0"));

    assert_eq!(&setup.zero - &setup.positive, Int::from("-18446744073709551617"));
    assert_eq!(&setup.zero - &setup.zero, Int::from("0"));
    assert_eq!(&setup.zero - &setup.negative, Int::from("18446744073709551617"));

    assert_eq!(Int::from(100000) - Int::from(1), 99999.into());
}

#[rstest]
fn mul(setup: Fixture) {
    assert_eq!(&setup.positive * &setup.positive, Int::from("340282366920938463500268095579187314689"));
    assert_eq!(&setup.positive * &setup.zero, Int::from("0"));
    assert_eq!(&setup.positive * &setup.negative, Int::from("-340282366920938463500268095579187314689"));

    assert_eq!(&setup.negative * &setup.positive, Int::from("-340282366920938463500268095579187314689"));
    assert_eq!(&setup.negative * &setup.zero, Int::from("0"));
    assert_eq!(&setup.negative * &setup.negative, Int::from("340282366920938463500268095579187314689"));

    assert_eq!(&setup.zero * &setup.positive, Int::from("0"));
    assert_eq!(&setup.zero * &setup.zero, Int::from("0"));
    assert_eq!(&setup.zero * &setup.negative, Int::from("0"));

    assert_eq!(Int::from(100000) * Int::from(1), 100000.into());
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
            assert_eq!(&setup.positive / &setup.positive, Int::from(1));
            // 2
            assert_eq!(&setup.positive / &setup.negative, Int::from(-1));

            assert_eq!(&setup.negative / &setup.positive, Int::from(-1));
            // 3
            assert_eq!(&setup.negative / &setup.negative, Int::from(1));

            assert_eq!(&setup.zero / &setup.positive, Int::from(0));
            // 4
            assert_eq!(&setup.zero / &setup.negative, Int::from(0));

            assert_eq!(Int::from(100000) / Int::from(1), 100000.into());

            Int::new() // for compatible types
        }
        2 => &setup.positive / &setup.zero,
        3 => &setup.negative / &setup.zero,
        4 => &setup.zero / &setup.zero,
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
            assert_eq!(&setup.positive % &setup.positive, Int::from(0));
            // 2
            assert_eq!(&setup.positive % &setup.negative, Int::from(0));

            assert_eq!(&setup.negative % &setup.positive, Int::from(0));
            // 3
            assert_eq!(&setup.negative % &setup.negative, Int::from(0));

            assert_eq!(&setup.zero % &setup.positive, Int::from(0));
            // 4
            assert_eq!(&setup.zero % &setup.negative, Int::from(0));

            assert_eq!(Int::from(100000) % Int::from(1), 0.into());

            Int::new() // for compatible types
        }
        2 => &setup.positive % &setup.zero,
        3 => &setup.negative % &setup.zero,
        4 => &setup.zero % &setup.zero,
        _ => unreachable!(),
    };
}

#[rstest]
fn factorial() {
    // 0! == 1
    assert_eq!(Int::from("0").factorial(), "1".into());

    // 1! == 1
    assert_eq!(Int::from("1").factorial(), "1".into());

    // 2! == 2
    assert_eq!(Int::from("2").factorial(), "2".into());

    // 3! == 6
    assert_eq!(Int::from("3").factorial(), "6".into());

    // 100! == 93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000
    assert_eq!(
        Int::from("100").factorial(),
        "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000"
            .into()
    );

    // (5!)! == 6689502913449127057588118054090372586752746333138029810295671352301633557244962989366874165271984981308157637893214090552534408589408121859898481114389650005964960521256960000000000000000000000000000
    assert_eq!(Int::from("5").factorial().factorial(), "6689502913449127057588118054090372586752746333138029810295671352301633557244962989366874165271984981308157637893214090552534408589408121859898481114389650005964960521256960000000000000000000000000000".into());
}

#[rstest]
#[should_panic(expected = "Error: Negative integer have no factorial.")]
fn bad_factorial() {
    Int::from("-1").factorial();
}

#[rstest]
fn next_prime() {
    let mut number = Int::new(); // 0

    let primes: Vec<Int> = vec![
        2.into(),
        3.into(),
        5.into(),
        7.into(),
        11.into(),
        13.into(),
        17.into(),
        19.into(),
        23.into(),
        29.into(),
        31.into(),
        37.into(),
        41.into(),
        43.into(),
        47.into(),
        53.into(),
        59.into(),
        61.into(),
        67.into(),
        71.into(),
    ];

    for prime in primes {
        number = number.next_prime();
        assert_eq!(number, prime);
    }

    assert_eq!(Int::from(104728).next_prime(), 104729.into()); // the 10000th prime

    assert_eq!(Int::from("2147483628").next_prime(), "2147483629".into()); // maximum prime number that < i32::MAX
    assert_eq!(Int::from("2147483629").next_prime(), "2147483647".into()); // i32::MAX is a prime number
    assert_eq!(Int::from("2147483647").next_prime(), "2147483659".into()); // minimum prime number that > i32::MAX
}

#[rstest]
fn to_integer(setup: Fixture) {
    assert_eq!(setup.zero.to_integer::<i32>(), 0i32);
    assert_eq!(setup.zero.to_integer::<i8>(), 0i8);
    assert_eq!(setup.positive.to_integer::<i128>(), 18446744073709551617);
    assert_eq!(setup.negative.to_integer::<i128>(), -18446744073709551617);
}

#[rstest]
fn sqrt() {
    assert_eq!(Int::sqrt(&"0".into()), "0".into());
    assert_eq!(Int::sqrt(&"1".into()), "1".into());
    assert_eq!(Int::sqrt(&"2".into()), "1".into());
    assert_eq!(Int::sqrt(&"3".into()), "1".into());
    assert_eq!(Int::sqrt(&"4".into()), "2".into());
    assert_eq!(Int::sqrt(&"5".into()), "2".into());
    assert_eq!(Int::sqrt(&"9".into()), "3".into());
    assert_eq!(Int::sqrt(&"10".into()), "3".into());
    assert_eq!(Int::sqrt(&"16".into()), "4".into());
    assert_eq!(Int::sqrt(&"100".into()), "10".into());
    assert_eq!(Int::sqrt(&"9801".into()), "99".into());
}

#[rstest]
#[should_panic(expected = "Error: Cannot compute square root of a negative integer.")]
fn bad_sqrt() {
    Int::sqrt(&"-1".into());
}

#[rstest]
fn pow() {
    // special situations
    assert_eq!(Int::pow(&"-1".into(), &"-1".into()), Int::from("-1"));
    assert_eq!(Int::pow(&"-1".into(), &"0".into()), Int::from("1"));
    assert_eq!(Int::pow(&"-1".into(), &"1".into()), Int::from("-1"));
    // assert_eq!(Int::pow(&"0".into(), &"-1".into())); // Error: Math domain error.
    assert_eq!(Int::pow(&"0".into(), &"0".into()), Int::from("1"));
    assert_eq!(Int::pow(&"0".into(), &"1".into()), Int::from("0"));
    assert_eq!(Int::pow(&"1".into(), &"-1".into()), Int::from("1"));
    assert_eq!(Int::pow(&"1".into(), &"0".into()), Int::from("1"));
    assert_eq!(Int::pow(&"1".into(), &"1".into()), Int::from("1"));

    // 2^3 == 8
    assert_eq!(Int::pow(&"2".into(), &"3".into()), Int::from("8"));

    // 2^100 == 1267650600228229401496703205376
    assert_eq!(Int::pow(&"2".into(), &"100".into()), Int::from("1267650600228229401496703205376"));

    // (9^9)^9 == 196627050475552913618075908526912116283103450944214766927315415537966391196809
    assert_eq!(
        Int::pow(&Int::pow(&"9".into(), &"9".into()), &"9".into()),
        Int::from("196627050475552913618075908526912116283103450944214766927315415537966391196809")
    );

    // 1024^1024 % 100 == 76
    assert_eq!(Int::pow_mod(&"1024".into(), &"1024".into(), &"100".into()), Int::from("76"));

    // 9999^1001 % 100 == 99
    assert_eq!(Int::pow_mod(&"9999".into(), &"1001".into(), &"100".into()), Int::from("99"));
}

#[rstest]
fn log(setup: Fixture) {
    assert_eq!(Int::log(&1.into(), &2.into()), 0.into());
    assert_eq!(Int::log(&1.into(), &3.into()), 0.into());
    assert_eq!(Int::log(&1.into(), &4.into()), 0.into());

    assert_eq!(Int::log(&2.into(), &2.into()), 1.into());
    assert_eq!(Int::log(&4.into(), &2.into()), 2.into());
    assert_eq!(Int::log(&8.into(), &2.into()), 3.into());

    assert_eq!(Int::log(&10.into(), &10.into()), 1.into());
    assert_eq!(Int::log(&100.into(), &10.into()), 2.into());
    assert_eq!(Int::log(&1000.into(), &10.into()), 3.into());

    assert_eq!(Int::log(&123.into(), &10.into()), 2.into());
    assert_eq!(Int::log(&12345.into(), &10.into()), 4.into());
    assert_eq!(Int::log(&123456789.into(), &10.into()), 8.into());

    assert_eq!(Int::log(&setup.positive, &2.into()), 64.into()); // integer: 2^64+1
    assert_eq!(Int::log(&(&setup.positive * Int::from(2) - Int::from(3)), &2.into()), 64.into()); // integer: 2^65-1
    assert_eq!(Int::log(&(&setup.positive * Int::from(2) - Int::from(2)), &2.into()), 65.into()); // integer: 2^65
    assert_eq!(Int::log(&(&setup.positive * Int::from(2)), &2.into()), 65.into()); // integer: 2^65+2

    assert_eq!(Int::log(&"123456789000".into(), &233.into()), 4.into()); // 4.6851911360933745
}

#[rstest]
fn gcd_lcm() {
    // gcd()
    assert_eq!(Int::gcd(&"0".into(), &"1".into()), "1".into());
    assert_eq!(Int::gcd(&"6".into(), &"12".into()), "6".into());
    assert_eq!(Int::gcd(&"6".into(), &"11".into()), "1".into());
    assert_eq!(Int::gcd(&"12345".into(), &"54321".into()), "3".into());

    // lcm()
    assert_eq!(Int::lcm(&"0".into(), &"1".into()), "0".into());
    assert_eq!(Int::lcm(&"6".into(), &"12".into()), "12".into());
    assert_eq!(Int::lcm(&"6".into(), &"11".into()), "66".into());
    assert_eq!(Int::lcm(&"12345".into(), &"54321".into()), "223530915".into());
}

#[rstest]
fn random() {
    assert_eq!(Int::random(0).digits(), 0);

    assert_eq!(Int::random(1).digits(), 1);
    assert_eq!(Int::random(2).digits(), 2);
    assert_eq!(Int::random(3).digits(), 3);

    assert_eq!(Int::random(4300).digits(), 4300);
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.zero), "0");
    assert_eq!(format!("{}", setup.positive), "18446744073709551617");
    assert_eq!(format!("{}", setup.negative), "-18446744073709551617");
}

#[rstest]
fn parse(setup: Fixture) {
    assert_eq!(setup.zero, "0".parse().unwrap());
    assert_eq!(setup.positive, "18446744073709551617".parse().unwrap());
    assert_eq!(setup.negative, "-18446744073709551617".parse().unwrap());

    assert_eq!(setup.zero, "\t0000  \n".parse().unwrap());
    assert_eq!(setup.positive, "  18446744073709551617 \n\n".parse().unwrap());
    assert_eq!(setup.negative, "  -18446744073709551617 \n\n".parse().unwrap());

    assert!("hello".parse::<Int>().is_err());
}
