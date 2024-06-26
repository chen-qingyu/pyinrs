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
        one: List::from([1]),
        some: List::from([1, 2, 3, 4, 5]),
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
    assert!(List::new() == setup.empty);
    assert!(List::from([1, 2, 3, 4, 5]) == setup.some);

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
fn iterator(setup: Fixture) {
    let mut i = 0;
    for e in setup.some.clone() {
        i += 1;
        assert_eq!(e, i);
    }
    assert_eq!(i, 5);

    for &e in setup.some.iter().rev() {
        assert_eq!(e, i);
        i -= 1;
    }
    assert_eq!(i, 0);

    let mapped: List<i32> = setup.some.iter().map(|x| x * 2).collect();
    assert_eq!(mapped, List::from([2, 4, 6, 8, 10]));

    let filtered: List<i32> = setup.some.clone().into_iter().filter(|x| x & 1 == 1).collect();
    assert_eq!(filtered, List::from([1, 3, 5]));

    assert_eq!(setup.empty.into_iter().rev().collect::<List<i32>>(), List::new());
    assert_eq!(setup.one.into_iter().rev().collect::<List<i32>>(), List::from([1]));
    assert_eq!(setup.some.into_iter().rev().collect::<List<i32>>(), List::from([5, 4, 3, 2, 1]));
}

#[rstest]
fn access(mut setup: Fixture) {
    // forward
    for i in 0..setup.some.len() {
        assert_eq!(setup.some[i], i + 1);
    }

    // backward
    for i in -1..-setup.some.len() {
        assert_eq!(setup.some[i], i + 6);
    }

    // assignment
    setup.some[0] = 0;
    assert_eq!(setup.some, List::from([0, 2, 3, 4, 5]));

    setup.some[-1] = 999;
    assert_eq!(setup.some, List::from([0, 2, 3, 4, 999]));
}

#[rstest]
#[should_panic(expected = "Error: Index out of range: 5 not in -5..5.")]
fn bad_access(setup: Fixture) {
    setup.some[5];
}

#[rstest]
fn examination(setup: Fixture) {
    // find
    assert_eq!(setup.some.find(&0), None);
    assert_eq!(setup.some.find(&1), Some(&1));

    // contains
    assert!(!setup.some.contains(&0));
    assert!(setup.some.contains(&1));

    // count
    assert_eq!(setup.some.count(&0), 0);
    assert_eq!(setup.some.count(&1), 1);
}

#[rstest]
fn insert(mut setup: Fixture) {
    setup.empty.insert(0, 233);
    assert_eq!(setup.empty, List::from([233]));
    setup.empty.insert(0, 1);
    assert_eq!(setup.empty, List::from([1, 233]));
    setup.empty.insert(2, 999);
    assert_eq!(setup.empty, List::from([1, 233, 999]));
    setup.empty.insert(1, 5);
    assert_eq!(setup.empty, List::from([1, 5, 233, 999]));
    setup.empty.insert(-1, -1);
    assert_eq!(setup.empty, List::from([1, 5, 233, -1, 999]));
}

#[rstest]
#[should_panic(expected = "Error: Index out of range: 999 not in -5..6.")]
fn bad_insert(mut setup: Fixture) {
    setup.some.insert(999, 0);
}

/* The test was successful! But the testing time is too long, and comment it out. */
// #[rstest]
// #[should_panic(expected = "Error: The container has reached the maximum size: 2147483647.")]
// fn full_insert(mut setup: Fixture) {
//     for _ in 0..i32::MAX {
//         setup.empty.insert(setup.empty.size(), 0);
//     }
//     assert_eq!(setup.empty.size(), i32::MAX);
//     setup.empty.insert(setup.empty.size(), 0);
// }

#[rstest]
fn remove(mut setup: Fixture) {
    assert_eq!(setup.some.remove(-2), 4);
    assert_eq!(setup.some.remove(1), 2);
    assert_eq!(setup.some.remove(0), 1);
    assert_eq!(setup.some.remove(0), 3);
    assert_eq!(setup.some.remove(0), 5);
}

#[rstest]
#[should_panic(expected = "Error: Index out of range: 999 not in -5..5.")]
fn bad_remove(mut setup: Fixture) {
    setup.some.remove(999);
}

#[rstest]
#[should_panic(expected = "Error: The container is empty.")]
fn empty_remove(mut setup: Fixture) {
    setup.empty.remove(0);
}

#[rstest]
fn uniquify() {
    assert_eq!(List::from([1, 2, 2, 3, 3, 3]).uniquify(), List::from([1, 2, 3]));
    assert_eq!(List::from([1, 2, 3, 1, 2, 3, 1, 2, 3]).uniquify(), List::from([1, 2, 3]));
    assert_eq!(List::from([0].repeat(10000)).uniquify(), List::from([0]));
}

#[rstest]
fn append(mut setup: Fixture) {
    setup.empty += setup.empty.clone();
    assert_eq!(setup.empty, List::new());

    setup.one += setup.one.clone();
    assert_eq!(setup.one, List::from([1, 1]));

    setup.one += setup.one.clone();
    assert_eq!(setup.one, List::from([1, 1, 1, 1]));
}

#[rstest]
fn clear(mut setup: Fixture) {
    setup.some.clear();
    assert_eq!(setup.some, setup.empty);
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "[]");
    assert_eq!(format!("{}", setup.one), "[1]");
    assert_eq!(format!("{}", setup.some), "[1, 2, 3, 4, 5]");
}
