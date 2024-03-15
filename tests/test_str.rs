use pyinrs::{List, Str};
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
        assert_eq!(setup.some[i], (i + 1).to_string());
    }

    // backward
    for i in -1..-setup.some.len() {
        assert_eq!(setup.some[i], (i + 6).to_string());
    }
}

#[rstest]
#[should_panic(expected = "Error: Index out of range: 5 not in -5..5.")]
fn bad_access(setup: Fixture) {
    setup.some[5].to_string();
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
    assert_eq!(
        Str::from("one, two, three").split(", "),
        List::from(["one", "two", "three"])
    );
    assert_eq!(Str::from("aaa").split("a"), List::from(["", "", "", ""]));
    assert_eq!(
        Str::from("192.168.0.1").split("."),
        List::from(["192", "168", "0", "1"])
    );
}

#[rstest]
fn join() {
    assert_eq!(Str::from(", ").join([""].into()), Str::from(""));
    assert_eq!(Str::from(", ").join(["a", "b"].into()), Str::from("a, b"));
    assert_eq!(
        Str::from(".").join(["192", "168", "0", "1"].into()),
        Str::from("192.168.0.1")
    );
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "\"\"");
    assert_eq!(format!("{}", setup.one), "\"1\"");
    assert_eq!(format!("{}", setup.some), "\"12345\"");
}
