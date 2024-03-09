use pyinrs::List;
use rstest::{fixture, rstest};

struct Fixture {
    empty: List<i32>,
    one: List<i32>,
    some: List<i32>,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        empty: List::new(),
        one: List::from(&[1]),
        some: List::from(&[1, 2, 3, 4, 5]),
    }
}

#[rstest]
fn basics(setup: Fixture) {
    assert_eq!(setup.empty.size(), 0);
    assert_eq!(setup.empty.is_empty(), true);

    assert_eq!(setup.one.size(), 1);
    assert_eq!(setup.one.is_empty(), false);

    assert_eq!(setup.some.size(), 5);
    assert_eq!(setup.some.is_empty(), false);
}

#[rstest]
fn compare(setup: Fixture) {
    assert!(List::new() == setup.empty);
    assert!(List::from(&[1, 2, 3, 4, 5]) == setup.some);

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
fn access(mut setup: Fixture) {
    // forward
    for i in 0..setup.some.size() {
        assert_eq!(setup.some[i], (i + 1).try_into().unwrap());
    }

    // assignment
    setup.some[0] = 0;
    assert_eq!(setup.some[0], 0);
}

#[rstest]
fn iterator(setup: Fixture) {
    let mut i = 1;
    for e in setup.some {
        assert_eq!(e, i);
        i += 1;
    }
}

#[rstest]
fn examination(setup: Fixture) {
    // find
    assert_eq!(setup.some.find(&0), None);
    assert_eq!(setup.some.find(&1), Some(&1));

    // contains
    assert_eq!(setup.some.contains(&0), false);
    assert_eq!(setup.some.contains(&1), true);

    // count
    assert_eq!(setup.some.count(&0), 0);
    assert_eq!(setup.some.count(&1), 1);
}

#[rstest]
fn uniquify() {
    let mut list = List::from(&[1, 2, 2, 3, 3, 3]);
    list.uniquify();
    assert_eq!(list, List::from(&[1, 2, 3]));

    let mut list = List::from(&[1, 2, 3, 1, 2, 3, 1, 2, 3]);
    list.uniquify();
    assert_eq!(list, List::from(&[1, 2, 3]));

    let mut many = List::new();
    for _ in 0..10000 {
        many.push(0);
    }
    many.uniquify();
    assert_eq!(many, List::from(&[0]));
}

#[rstest]
fn append(mut setup: Fixture) {
    setup.empty += setup.empty.clone();
    assert_eq!(setup.empty, List::new());

    setup.one += setup.one.clone();
    assert_eq!(setup.one, List::from(&[1, 1]));

    setup.one += setup.one.clone();
    assert_eq!(setup.one, List::from(&[1, 1, 1, 1]));
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "[]");
    assert_eq!(format!("{}", setup.one), "[1]");
    assert_eq!(format!("{}", setup.some), "[1, 2, 3, 4, 5]");
}
