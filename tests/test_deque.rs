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

    let mapped: Deque<i32> = setup.some.iter().map(|x| x * 2).collect();
    assert_eq!(mapped, Deque::from([2, 4, 6, 8, 10]));

    let filtered: Deque<i32> = setup.some.clone().into_iter().filter(|x| x & 1 == 1).collect();
    assert_eq!(filtered, Deque::from([1, 3, 5]));

    assert_eq!(setup.empty.into_iter().rev().collect::<Deque<i32>>(), Deque::new());
    assert_eq!(setup.one.into_iter().rev().collect::<Deque<i32>>(), Deque::from([1]));
    assert_eq!(setup.some.into_iter().rev().collect::<Deque<i32>>(), Deque::from([5, 4, 3, 2, 1]));
}

#[rstest]
fn access(mut setup: Fixture) {
    assert_eq!(setup.empty.back(), None);
    assert_eq!(setup.empty.front(), None);

    *setup.some.back_mut().unwrap() += 1;
    *setup.some.front_mut().unwrap() -= 1;
    assert_eq!(setup.some.back(), Some(&6));
    assert_eq!(setup.some.front(), Some(&0));

    setup.some[-1] += 1;
    setup.some[0] -= 1;
    assert_eq!(setup.some[-1], 7);
    assert_eq!(setup.some[0], -1);
}

#[rstest]
#[should_panic(expected = "Error: Index out of range: 0 not in 0..0.")]
fn bad_access(setup: Fixture) {
    setup.empty[0];
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
fn clear(mut setup: Fixture) {
    setup.some.clear();
    assert_eq!(setup.some, setup.empty);
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "<>");
    assert_eq!(format!("{}", setup.one), "<1>");
    assert_eq!(format!("{}", setup.some), "<1, 2, 3, 4, 5>");
}
