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
    assert!(Set::from([5, 4, 3, 2, 1, 2, 3, 4, 5]) == setup.some);

    // operator!=
    assert!(setup.one != setup.some);
    assert!(setup.empty != setup.one);

    // operator<
    assert!(Set::from([5, 1]) < setup.some);
    assert!(setup.empty < setup.one);

    // operator<=
    assert!(setup.some <= setup.some);
    assert!(setup.empty <= setup.one);

    // operator>
    assert!(Set::from([0, 1, 2, 3, 4, 5]) > setup.some);
    assert!(setup.one > setup.empty);

    // operator>=
    assert!(setup.some >= setup.some);
    assert!(setup.one >= setup.empty);

    // neither a subset nor a superset
    assert!(Set::from([0, 1]).partial_cmp(&Set::from([2, 3])).is_none());
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

    let mapped: Set<i32> = setup.some.iter().map(|x| x * 2).collect();
    assert_eq!(mapped, Set::from([2, 4, 6, 8, 10]));

    let filtered: Set<i32> = setup.some.clone().into_iter().filter(|x| x & 1 == 1).collect();
    assert_eq!(filtered, Set::from([1, 3, 5]));

    assert_eq!(setup.empty.into_iter().rev().collect::<Set<i32>>(), Set::new());
    assert_eq!(setup.one.into_iter().rev().collect::<Set<i32>>(), Set::from([1]));
    assert_eq!(setup.some.into_iter().rev().collect::<Set<i32>>(), Set::from([1, 2, 3, 4, 5]));
}

#[rstest]
fn examination(setup: Fixture) {
    assert_eq!(setup.some.find(&1), Some(&1));
    assert_eq!(setup.some.find(&0), None);

    assert_eq!(setup.some.contains(&1), true);
    assert_eq!(setup.some.contains(&0), false);

    assert_eq!(setup.some.min(), Some(&1));
    assert_eq!(setup.some.max(), Some(&5));
}

#[rstest]
fn add(mut setup: Fixture) {
    assert_eq!(setup.empty.add(3), true);
    assert_eq!(setup.empty.add(1), true);
    assert_eq!(setup.empty.add(2), true);
    assert_eq!(setup.empty.add(5), true);
    assert_eq!(setup.empty.add(4), true);

    assert_eq!(setup.empty, setup.some);

    assert_eq!(setup.empty.add(4), false);
    assert_eq!(setup.empty.add(5), false);
    assert_eq!(setup.empty.add(2), false);
    assert_eq!(setup.empty.add(1), false);
    assert_eq!(setup.empty.add(3), false);

    assert_eq!(setup.empty, setup.some);
}

#[rstest]
fn remove(mut setup: Fixture) {
    assert_eq!(setup.some.remove(&3), true);
    assert_eq!(setup.some.remove(&1), true);
    assert_eq!(setup.some.remove(&2), true);
    assert_eq!(setup.some.remove(&5), true);
    assert_eq!(setup.some.remove(&4), true);

    assert_eq!(setup.some, setup.empty);

    assert_eq!(setup.some.remove(&4), false);
    assert_eq!(setup.some.remove(&5), false);
    assert_eq!(setup.some.remove(&2), false);
    assert_eq!(setup.some.remove(&1), false);
    assert_eq!(setup.some.remove(&3), false);

    assert_eq!(setup.some, setup.empty);
}

#[rstest]
fn pop(mut setup: Fixture) {
    assert_eq!(setup.some.pop(), Some(1));
    assert_eq!(setup.some.pop(), Some(2));
    assert_eq!(setup.some.pop(), Some(3));
    assert_eq!(setup.some.pop(), Some(4));
    assert_eq!(setup.some.pop(), Some(5));
    assert_eq!(setup.some.pop(), None);
}

#[rstest]
fn ops() {
    let set1 = Set::from([1, 2, 3, 4, 5]);
    let set2 = Set::from([1, 3, 5, 7, 9]);

    assert_eq!(&set1 & &set2, Set::from([1, 3, 5]));
    assert_eq!(&set1 | &set2, Set::from([1, 2, 3, 4, 5, 7, 9]));
    assert_eq!(&set1 ^ &set2, Set::from([2, 4, 7, 9]));
    assert_eq!(&set1 - &set2, Set::from([2, 4]));

    assert_eq!(Set::<i32>::new() & Set::new(), Set::new());
    assert_eq!(Set::<i32>::new() | Set::new(), Set::new());
    assert_eq!(Set::<i32>::new() ^ Set::new(), Set::new());
    assert_eq!(Set::<i32>::new() - Set::new(), Set::new());
}

#[rstest]
fn clear(mut setup: Fixture) {
    setup.some.clear();
    assert_eq!(setup.some, setup.empty);
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "{}");
    assert_eq!(format!("{}", setup.one), "{1}");
    assert_eq!(format!("{}", setup.some), "{1, 2, 3, 4, 5}");
}
