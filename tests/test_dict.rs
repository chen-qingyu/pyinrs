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
fn access() {
    let mut dict = Dict::from([("one", 1), ("two", 2), ("three", 3)]);

    // get
    assert_eq!(dict.get(&"one", &0), &1);
    assert_eq!(dict.get(&"not exist", &0), &0);

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
fn keys_values_items(setup: Fixture) {
    assert_eq!(setup.some.keys(), pyinrs::Set::from([1, 2, 3]));
    assert_eq!(setup.some.values(), pyinrs::Set::from(["one", "two", "three"]));
    assert_eq!(setup.some.items(), pyinrs::Set::from([(1, "one"), (2, "two"), (3, "three")]));
}

#[rstest]
fn format(setup: Fixture) {
    assert_eq!(format!("{}", setup.empty), "{}");
    assert_eq!(format!("{}", setup.one), "{1: one}");
    assert_eq!(format!("{}", setup.some), "{1: one, 2: two, 3: three}");
}
