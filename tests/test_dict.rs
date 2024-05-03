use pyinrs::Dict;
use rstest::{fixture, rstest};

struct Fixture<'a> {
    empty: Dict<i32, &'a str>,
    one: Dict<i32, &'a str>,
    some: Dict<i32, &'a str>,
}

#[fixture]
fn setup() -> Fixture<'static> {
    Fixture {
        empty: Dict::new(),
        one: Dict::from([(1, "one")]),
        some: Dict::from([(1, "one"), (2, "two"), (3, "three")]),
    }
}

#[rstest]
fn basics(setup: Fixture) {
    assert_eq!(setup.empty.len(), 0);
    assert!(setup.empty.is_empty());

    assert_eq!(setup.one.len(), 1);
    assert!(!setup.one.is_empty());

    assert_eq!(setup.some.len(), 3);
    assert!(!setup.some.is_empty());
}

#[rstest]
fn compare(setup: Fixture) {
    // ==
    assert!(Dict::new() == setup.empty);
    assert!(Dict::from([(1, "one"), (2, "two"), (3, "three")]) == setup.some);

    // !=
    assert!(Dict::from([(1, "one")]) != setup.empty);
    assert!(Dict::from([(1, "one"), (2, "two"), (3, "three!")]) != setup.some);
}

#[rstest]
fn iterator(setup: Fixture) {
    for (k, v) in Dict::from([(1, 1), (2, 4), (3, 9)]) {
        assert_eq!(k * k, v);
    }

    let mapped: Dict<i32, &str> = setup.some.iter().map(|(&k, &v)| (k - 1, v)).collect();
    assert_eq!(mapped, Dict::from([(0, "one"), (1, "two"), (2, "three")]));

    let filtered: Dict<i32, &str> = setup.some.clone().into_iter().filter(|p| p.1.len() > 3).collect();
    assert_eq!(filtered, Dict::from([(3, "three")]));

    assert_eq!(setup.empty.into_iter().rev().collect::<Dict<i32, &str>>(), Dict::new());
    assert_eq!(setup.one.into_iter().rev().collect::<Dict<i32, &str>>(), Dict::from([(1, "one")]));
    assert_eq!(
        setup.some.into_iter().rev().collect::<Dict<i32, &str>>(),
        Dict::from([(1, "one"), (2, "two"), (3, "three")])
    );
}

#[rstest]
fn access() {
    let mut dict = Dict::from([("one", 1), ("two", 2), ("three", 3)]);

    // get
    assert_eq!(dict.get(&"one", &233), &1);
    assert_eq!(dict.get(&"not exist", &233), &233);

    // access
    assert_eq!(dict[&"one"], 1);
    assert_eq!(dict[&"two"], 2);
    assert_eq!(dict[&"three"], 3);

    // assignment
    dict[&"one"] = 1111;
    assert_eq!(dict[&"one"], 1111);
}

#[rstest]
#[should_panic(expected = "Error: Key is not found in the dict.")]
fn bad_access(setup: Fixture) {
    setup.some[&4];
}

#[rstest]
fn examination(setup: Fixture) {
    assert_eq!(setup.some.find(&1), Some((&1, &"one")));
    assert_eq!(setup.some.find(&0), None);

    assert_eq!(setup.some.contains(&1), true);
    assert_eq!(setup.some.contains(&0), false);
}

#[rstest]
fn keys_values_items(setup: Fixture) {
    assert_eq!(setup.some.keys(), pyinrs::Set::from([1, 2, 3]));
    assert_eq!(setup.some.values(), pyinrs::Set::from(["one", "two", "three"]));
    assert_eq!(setup.some.items(), pyinrs::Set::from([(1, "one"), (2, "two"), (3, "three")]));
}

#[rstest]
fn add(mut setup: Fixture) {
    assert_eq!(setup.empty.add(3, "three"), true);
    assert_eq!(setup.empty.add(1, "one"), true);
    assert_eq!(setup.empty.add(2, "two"), true);

    assert_eq!(setup.empty, Dict::from([(1, "one"), (2, "two"), (3, "three")]));

    assert_eq!(setup.empty.add(3, "three"), false);
    assert_eq!(setup.empty.add(1, "one"), false);
    assert_eq!(setup.empty.add(2, "two"), false);

    assert_eq!(setup.empty, Dict::from([(1, "one"), (2, "two"), (3, "three")]));
}

#[rstest]
fn remove(mut setup: Fixture) {
    assert_eq!(setup.some.remove(&3), true);
    assert_eq!(setup.some.remove(&1), true);
    assert_eq!(setup.some.remove(&2), true);

    assert_eq!(setup.some, Dict::new());

    assert_eq!(setup.some.remove(&2), false);
    assert_eq!(setup.some.remove(&1), false);
    assert_eq!(setup.some.remove(&3), false);

    assert_eq!(setup.some, Dict::new());
}

#[rstest]
fn pop(mut setup: Fixture) {
    assert_eq!(setup.some.pop(), Some((1, "one")));
    assert_eq!(setup.some.pop(), Some((2, "two")));
    assert_eq!(setup.some.pop(), Some((3, "three")));
    assert_eq!(setup.some.pop(), None);
}

#[rstest]
fn clear(mut setup: Fixture) {
    setup.some.clear();
    assert_eq!(setup.some, setup.empty);
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "{}");
    assert_eq!(format!("{}", setup.one), "{1: one}");
    assert_eq!(format!("{}", setup.some), "{1: one, 2: two, 3: three}");
}
