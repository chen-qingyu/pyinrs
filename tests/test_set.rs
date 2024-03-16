use pyinrs::Set;
use rstest::{fixture, rstest};

struct Fixture {
    empty: Set<i32>,
    one: Set<i32>,
    some: Set<i32>,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        empty: Set::new(),
        one: Set::from([1]),
        some: Set::from([1, 2, 3, 4, 5]),
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
    // operator==
    assert!(Set::new() == setup.empty);
    assert!(Set::from([5, 4, 3, 2, 1]) == setup.some);

    // operator!=
    assert!(setup.one != setup.some);
    assert!(setup.empty != setup.one);

    // operator<
    assert!(Set::from([5, 4, 3, 2]) < setup.some);
    assert!(setup.empty < setup.one);

    // operator<=
    assert!(setup.some <= setup.some);
    assert!(setup.empty <= setup.one);

    // operator>
    assert!(Set::from([0, 1]) > setup.one);
    assert!(setup.one > setup.empty);

    // operator>=
    assert!(setup.some >= setup.some);
    assert!(setup.one >= setup.empty);
}

#[rstest]
fn iterator(setup: Fixture) {
    let mut i = 1;
    for &e in setup.some.iter() {
        assert_eq!(e, i);
        i += 1;
    }

    let mut i = 1;
    for e in setup.some {
        assert_eq!(e, i);
        i += 1;
    }
}

#[rstest]
fn ops() {
    let set1 = Set::from([1, 2, 3, 4, 5]);
    let set2 = Set::from([1, 3, 5, 7, 9]);

    assert_eq!(&set1 & &set2, Set::from([1, 3, 5]));
    assert_eq!(&set1 | &set2, Set::from([1, 2, 3, 4, 5, 7, 9]));
    assert_eq!(&set1 ^ &set2, Set::from([2, 4, 7, 9]));
    assert_eq!(&set1 - &set2, Set::from([2, 4]));
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "{}");
    assert_eq!(format!("{}", setup.one), "{1}");
    assert_eq!(format!("{}", setup.some), "{1, 2, 3, 4, 5}");
}
