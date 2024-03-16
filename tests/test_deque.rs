use pyinrs::Deque;
use rstest::{fixture, rstest};

struct Fixture {
    empty: Deque<i32>,
    one: Deque<i32>,
    some: Deque<i32>,
}

#[fixture]
fn setup() -> Fixture {
    Fixture {
        empty: Deque::new(),
        one: Deque::from([1]),
        some: Deque::from([1, 2, 3, 4, 5]),
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
    assert!(Deque::new() == setup.empty);
    assert!(Deque::from([1, 2, 3, 4, 5]) == setup.some);

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
    assert_eq!(setup.empty.into_iter().rev().collect::<Deque<i32>>(), Deque::new());
    assert_eq!(setup.one.into_iter().rev().collect::<Deque<i32>>(), Deque::from([1]));
    assert_eq!(setup.some.into_iter().rev().collect::<Deque<i32>>(), Deque::from([5, 4, 3, 2, 1]));
}

#[rstest]
fn peek(mut setup: Fixture) {
    assert_eq!(setup.some.back(), Some(&5));
    assert_eq!(setup.some.front(), Some(&1));

    *setup.some.back_mut().unwrap() += 1;
    *setup.some.front_mut().unwrap() -= 1;
    assert_eq!(setup.some.back(), Some(&6));
    assert_eq!(setup.some.front(), Some(&0));
}

#[rstest]
fn rotate(mut setup: Fixture) {
    setup.empty >>= 1;
    assert_eq!(setup.empty, Deque::new());
    setup.empty >>= 2;
    assert_eq!(setup.empty, Deque::new());
    setup.empty <<= 1;
    assert_eq!(setup.empty, Deque::new());
    setup.empty <<= 2;
    assert_eq!(setup.empty, Deque::new());

    setup.empty.push_back(1);

    setup.empty >>= 1;
    assert_eq!(setup.empty, setup.one);
    setup.empty >>= 2;
    assert_eq!(setup.empty, setup.one);
    setup.empty <<= 1;
    assert_eq!(setup.empty, setup.one);
    setup.empty <<= 2;
    assert_eq!(setup.empty, setup.one);

    setup.empty.push_back(2);
    setup.empty.push_back(3);
    setup.empty.push_back(4);
    setup.empty.push_back(5);

    setup.empty >>= 1;
    assert_eq!(setup.empty, Deque::from([5, 1, 2, 3, 4]));
    setup.empty >>= 2;
    assert_eq!(setup.empty, Deque::from([3, 4, 5, 1, 2]));
    setup.empty <<= 1;
    assert_eq!(setup.empty, Deque::from([4, 5, 1, 2, 3]));
    setup.empty <<= 2;
    assert_eq!(setup.empty, Deque::from([1, 2, 3, 4, 5]));

    setup.empty >>= 233;
    assert_eq!(setup.empty, Deque::from([3, 4, 5, 1, 2]));
    setup.empty <<= 233;
    assert_eq!(setup.empty, Deque::from([1, 2, 3, 4, 5]));
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "<>");
    assert_eq!(format!("{}", setup.one), "<1>");
    assert_eq!(format!("{}", setup.some), "<1, 2, 3, 4, 5>");
}
