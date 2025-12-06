use pyinrs::Decimal;
use rstest::{fixture, rstest};

struct Fixture {
    zero: Decimal,
    positive: Decimal,
    negative: Decimal,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        zero: Decimal::new(),
        positive: Decimal::from("0.3"),
        negative: Decimal::from("-0.3"),
    }
}

#[rstest]
fn unary(setup: Fixture) {
    assert_eq!(-setup.zero, Decimal::from("0"));
    assert_eq!(-setup.positive, Decimal::from("-0.3"));
    assert_eq!(-setup.negative, Decimal::from("0.3"));

    assert_eq!(setup.zero.abs(), Decimal::from("0"));
    assert_eq!(setup.positive.abs(), Decimal::from("0.3"));
    assert_eq!(setup.negative.abs(), Decimal::from("0.3"));
}

#[rstest]
fn add(setup: Fixture) {
    assert_eq!(setup.positive + setup.positive, Decimal::from("0.6"));
    assert_eq!(setup.positive + setup.zero, Decimal::from("0.3"));
    assert_eq!(setup.positive + setup.negative, Decimal::from("0"));

    assert_eq!(setup.negative + setup.positive, Decimal::from("0"));
    assert_eq!(setup.negative + setup.zero, Decimal::from("-0.3"));
    assert_eq!(setup.negative + setup.negative, Decimal::from("-0.6"));

    assert_eq!(setup.zero + setup.positive, Decimal::from("0.3"));
    assert_eq!(setup.zero + setup.zero, Decimal::from("0"));
    assert_eq!(setup.zero + setup.negative, Decimal::from("-0.3"));
}

#[rstest]
fn sub(setup: Fixture) {
    assert_eq!(setup.positive - setup.positive, Decimal::from("0"));
    assert_eq!(setup.positive - setup.zero, Decimal::from("0.3"));
    assert_eq!(setup.positive - setup.negative, Decimal::from("0.6"));

    assert_eq!(setup.negative - setup.positive, Decimal::from("-0.6"));
    assert_eq!(setup.negative - setup.zero, Decimal::from("-0.3"));
    assert_eq!(setup.negative - setup.negative, Decimal::from("0"));

    assert_eq!(setup.zero - setup.positive, Decimal::from("-0.3"));
    assert_eq!(setup.zero - setup.zero, Decimal::from("0"));
    assert_eq!(setup.zero - setup.negative, Decimal::from("0.3"));
}

#[rstest]
fn mul(setup: Fixture) {
    assert_eq!(setup.positive * setup.positive, Decimal::from("0.09"));
    assert_eq!(setup.positive * setup.zero, Decimal::from("0"));
    assert_eq!(setup.positive * setup.negative, Decimal::from("-0.09"));

    assert_eq!(setup.negative * setup.positive, Decimal::from("-0.09"));
    assert_eq!(setup.negative * setup.zero, Decimal::from("0"));
    assert_eq!(setup.negative * setup.negative, Decimal::from("0.09"));

    assert_eq!(setup.zero * setup.positive, Decimal::from("0"));
    assert_eq!(setup.zero * setup.zero, Decimal::from("0"));
    assert_eq!(setup.zero * setup.negative, Decimal::from("0"));
}

#[rstest]
fn div(setup: Fixture) {
    assert_eq!(setup.positive / setup.positive, Decimal::from("1"));
    assert_eq!(setup.positive / setup.negative, Decimal::from("-1"));
    assert_eq!(setup.negative / setup.positive, Decimal::from("-1"));
    assert_eq!(setup.negative / setup.negative, Decimal::from("1"));
    assert_eq!(setup.zero / setup.positive, Decimal::from("0"));
    assert_eq!(setup.zero / setup.negative, Decimal::from("0"));
}

#[rstest]
fn rem(setup: Fixture) {
    assert_eq!(setup.positive % setup.positive, Decimal::from("0"));
    assert_eq!(setup.positive % setup.negative, Decimal::from("0"));
    assert_eq!(setup.negative % setup.positive, Decimal::from("0"));
    assert_eq!(setup.negative % setup.negative, Decimal::from("0"));
    assert_eq!(setup.zero % setup.positive, Decimal::from("0"));
    assert_eq!(setup.zero % setup.negative, Decimal::from("0"));
}

#[rstest]
fn from_string() {
    assert_eq!(Decimal::from("0").as_fraction().to_string(), "0");
    assert_eq!(Decimal::from("1.1").as_fraction().to_string(), "11/10");
    assert_eq!(Decimal::from("1234.56789").as_fraction().to_string(), "123456789/100000");
    assert_eq!(Decimal::from("0.75").as_fraction().to_string(), "3/4");
    assert_eq!(Decimal::from("22.33").as_fraction().to_string(), "2233/100");
    assert_eq!(Decimal::from("-1.2").as_fraction().to_string(), "-6/5");

    assert_eq!(Decimal::from("0.5").as_fraction().to_string(), "1/2");
    assert_eq!(Decimal::from("0.~3").as_fraction().to_string(), "1/3"); // 0.333...
    assert_eq!(Decimal::from("0.0~3").as_fraction().to_string(), "1/30"); // 0.0333...
    assert_eq!(Decimal::from("0.8~3").as_fraction().to_string(), "5/6"); // 0.8333...
    assert_eq!(Decimal::from("0.~83").as_fraction().to_string(), "83/99"); // 0.838383...
    assert_eq!(Decimal::from("0.123").as_fraction().to_string(), "123/1000");
    assert_eq!(Decimal::from("0.~123").as_fraction().to_string(), "41/333"); // 0.123123123...
    assert_eq!(Decimal::from("123").as_fraction().to_string(), "123");
    assert_eq!(Decimal::from("0.1~123").as_fraction().to_string(), "187/1665"); // 0.1123123123...
    assert_eq!(Decimal::from("12.34~56").as_fraction().to_string(), "61111/4950"); // 12.34565656...
    assert_eq!(Decimal::from("0.24~9").as_fraction().to_string(), "1/4"); // 0.24999... = 0.25
    assert_eq!(Decimal::from("0.~375").as_fraction().to_string(), "125/333"); // 0.375375375...
    assert_eq!(Decimal::from("4.~518").as_fraction().to_string(), "122/27"); // 4.518518518...
    assert_eq!(Decimal::from("0.~9").as_fraction().to_string(), "1"); // 0.999... = 1

    assert_eq!(Decimal::from("-1").as_fraction().to_string(), "-1");
    assert_eq!(Decimal::from("-0.~1").as_fraction().to_string(), "-1/9"); // -0.111...
    assert_eq!(Decimal::from("-1.9").as_fraction().to_string(), "-19/10");
    assert_eq!(Decimal::from("-1.~9").as_fraction().to_string(), "-2"); // -1.999... = -2
    assert_eq!(Decimal::from("-1.1~9").as_fraction().to_string(), "-6/5"); // -1.1999... = -1.2

    assert_eq!(Decimal::from("11").as_fraction().to_string(), "11"); // 11
    assert_eq!(Decimal::from("11#10").as_fraction().to_string(), "11"); // 11(10) = 11(10)
    assert_eq!(Decimal::from("11#2").as_fraction().to_string(), "3"); // 11(2) = 3(10)
    assert_eq!(Decimal::from("11#16").as_fraction().to_string(), "17"); // 11(16) = 17(10)
    assert_eq!(Decimal::from("0.1#2").as_fraction().to_string(), "1/2"); // 0.1(2) = 0.5(10)
    assert_eq!(Decimal::from("0.0~0011#2").as_fraction().to_string(), "1/10"); // 0.0001100110011...(2) = 0.1(10)
    assert_eq!(Decimal::from("-0.0~0011#2").as_fraction().to_string(), "-1/10"); // -0.0001100110011...(2) = -0.1(10)
    assert_eq!(Decimal::from("0.~1#2").as_fraction().to_string(), "1"); // 0.111...(2) = 0.999...(10) = 1

    assert_eq!(
        Decimal::from(std::f64::consts::PI.to_string().as_str()).as_fraction(),
        pyinrs::Fraction::from((3141592653589793i128, 1000000000000000))
    );
    assert_eq!(
        Decimal::from(std::f64::consts::E.to_string().as_str()).as_fraction(),
        pyinrs::Fraction::from((2718281828459045i128, 1000000000000000))
    );
}

#[rstest]
fn to_string() {
    assert_eq!(Decimal::from("1.000").to_string(), "1");
    assert_eq!(Decimal::from("0.~3").to_string(), "0.333...");
    assert_eq!(Decimal::from("0.0~3").to_string(), "0.0333...");
    assert_eq!(Decimal::from("0.83~3").to_string(), "0.8333...");
    assert_eq!(Decimal::from("0.123").to_string(), "0.123");
    assert_eq!(Decimal::from("0.~123").to_string(), "0.123123123...");
    assert_eq!(Decimal::from("0.123~123").to_string(), "0.123123123...");
    assert_eq!(Decimal::from("-0.~3").to_string(), "-0.333...");
    assert_eq!(Decimal::from("-0.~1").to_string(), "-0.111...");

    assert_eq!(Decimal::from("-0.0~0011#2").to_string(), "-0.1"); // -0.0001100110011...(2) = -0.1(10)
    assert_eq!(Decimal::from("0.~1#2").to_string(), "1"); // 0.111...(2) = 0.999...(10) = 1
}

#[rstest]
fn parse(setup: Fixture) {
    assert_eq!(setup.zero, "0".parse().unwrap());
    assert_eq!(setup.positive, "0.3".parse().unwrap());
    assert_eq!(setup.negative, "-0.3".parse().unwrap());

    assert_eq!(setup.zero, "  000\n\n".parse().unwrap());
    assert_eq!(setup.positive, "  0.3\t\n".parse().unwrap());
    assert_eq!(setup.negative, "\t-0.3\n\n".parse().unwrap());

    assert!("z0.3".parse::<Decimal>().is_err());
    assert!("0z.3".parse::<Decimal>().is_err());
    assert!("0.z3".parse::<Decimal>().is_err());
    assert!("0.3z".parse::<Decimal>().is_err());
    assert!("0|3".parse::<Decimal>().is_err());
}
