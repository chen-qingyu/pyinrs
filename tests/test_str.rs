use pyinrs::Str;
use rstest::{fixture, rstest};

struct Fixture {
    empty: Str,
    one: Str,
    some: Str,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        empty: Str::new(),
        one: Str::from("1"),
        some: Str::from("12345"),
    }
}

#[rstest]
fn basics(setup: Fixture) {
    assert_eq!(setup.empty.len(), 0);
    assert!(setup.empty.is_empty());

    assert_eq!(setup.one.len(), 1);
    assert!(!setup.one.is_empty());

    assert_eq!(setup.some.len(), 5);
    assert!(!setup.some.is_empty());
}

#[rstest]
fn compare(setup: Fixture) {
    assert!(Str::new() == setup.empty);
    assert!(Str::from("12345") == setup.some);

    assert!(setup.empty != setup.one);
    assert!(setup.one != setup.some);

    assert!(setup.some > setup.empty);
    assert!(setup.some > setup.one);

    assert!(setup.empty < setup.some);
    assert!(setup.one < setup.some);

    assert!(setup.empty >= setup.empty);
    assert!(setup.some >= setup.one);

    assert!(setup.empty <= setup.empty);
    assert!(setup.one <= setup.some);
}

#[rstest]
fn access(setup: Fixture) {
    // forward
    for i in 0..setup.some.len() {
        assert_eq!(setup.some[i], i as u8 + 1 + b'0');
    }

    // backward
    for i in (-setup.some.len()..=-1).rev() {
        assert_eq!(setup.some[i], (i as i8 + 6 + b'0' as i8) as u8);
    }

    assert_eq!(setup.one[0], b'1');
    assert_eq!(setup.one[-1], b'1');
    assert_eq!(setup.one.char_at(0), Some('1'));
    assert_eq!(setup.one.char_at(-1), Some('1'));
    assert_eq!(setup.one.char_at(1), None);

    let sparkle_heart = Str::from("💖");
    assert_eq!(sparkle_heart[0], 240);
    assert_eq!(sparkle_heart[1], 159);
    assert_eq!(sparkle_heart[2], 146);
    assert_eq!(sparkle_heart[3], 150);
    assert_eq!(sparkle_heart[-1], 150);
    assert_eq!(sparkle_heart[-2], 146);
    assert_eq!(sparkle_heart[-3], 159);
    assert_eq!(sparkle_heart[-4], 240);
    assert_eq!(sparkle_heart.char_at(0), Some('💖'));
    assert_eq!(sparkle_heart.char_at(-1), Some('💖'));
    assert_eq!(sparkle_heart.char_at(1), None);
}

#[rstest]
#[should_panic(expected = "Error: Index out of range: 1 not in -1..1.")]
fn bad_access(setup: Fixture) {
    setup.one[1];
}

#[rstest]
fn add(setup: Fixture) {
    assert_eq!(setup.some + setup.one, "123451".into());
    assert_eq!(Str::from("hello") + Str::from(" world"), "hello world".into());
}

#[rstest]
fn mul(setup: Fixture) {
    assert_eq!(setup.one.clone() * 0, "".into());
    assert_eq!(setup.one.clone() * 1, "1".into());
    assert_eq!(setup.one.clone() * 2, "11".into());
    assert_eq!(setup.one.clone() * 3, "111".into());
}

#[rstest]
fn count(setup: Fixture) {
    assert_eq!(setup.some.count('0'), 0);
    assert_eq!(setup.some.count('1'), 1);
    assert_eq!(Str::from("hello").count('l'), 2);
}

#[rstest]
fn to_decimal() {
    assert_eq!(Str::from("233.33").to_decimal().unwrap(), 233.33);
    assert_eq!(Str::from("123.456e-3").to_decimal().unwrap(), 0.123456);
    assert!(Str::from("hello").to_decimal().is_none());
}

#[rstest]
fn split() {
    assert_eq!(Str::from("one, two, three").split(", "), pyinrs::List::from(["one", "two", "three"]));
    assert_eq!(Str::from("aaa").split("a"), pyinrs::List::from(["", "", "", ""]));
    assert_eq!(Str::from("192.168.0.1").split("."), pyinrs::List::from(["192", "168", "0", "1"]));
}

#[rstest]
fn join() {
    assert_eq!(Str::from(", ").join([""].into()), Str::from(""));
    assert_eq!(Str::from(", ").join(["a", "b"].into()), Str::from("a, b"));
    assert_eq!(Str::from(".").join(["192", "168", "0", "1"].into()), Str::from("192.168.0.1"));
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "\"\"");
    assert_eq!(format!("{}", setup.one), "\"1\"");
    assert_eq!(format!("{}", setup.some), "\"12345\"");
}

#[rstest]
fn parse(setup: Fixture) {
    assert_eq!(setup.empty, "".parse().unwrap());
    assert_eq!(setup.one, "1".parse().unwrap());
    assert_eq!(setup.some, "12345".parse().unwrap());
}
