use pyinrs::*;
use rstest::rstest;

#[rstest]
fn demo() {
    // List index, supports negative subscript
    assert_eq!(List::from([1, 2, 3, 4, 5])[-1], 5);
    // List uniquify
    assert_eq!(List::from([1, 2, 3, 1, 2, 3, 1, 2, 3]).uniquify(), [1, 2, 3].into());

    // Adding elements to Set
    assert_eq!(Set::from([1, 2, 3, 4]).add(5), &[1, 2, 3, 4, 5].into());
    // Intersection of Sets, supports intersection, union, difference, and symmetric difference
    assert_eq!(Set::from([1, 2, 3, 4, 5]) & Set::from([1, 3, 5, 7, 9]), [1, 3, 5].into());

    // Dict assign value for key
    let mut d = Dict::from([("one", 1), ("two", 2), ("three", 3)]);
    d[&"one"] = 11;
    assert_eq!(d, [("one", 11), ("two", 2), ("three", 3)].into());
    // Dict get values
    assert_eq!(Dict::from([("one", 1), ("two", 2), ("three", 3)]).values(), [1, 2, 3].into());

    // Int modular power, very fast
    assert_eq!(Int::from("1024").pow(&"1024".into(), &"100".into()), 76.into());
    // Int factorial
    assert_eq!(Int::from("5").factorial().factorial(), "6689502913449127057588118054090372586752746333138029810295671352301633557244962989366874165271984981308157637893214090552534408589408121859898481114389650005964960521256960000000000000000000000000000".into());

    // Str split
    assert_eq!(Str::from("one, two, three").split(", "), ["one", "two", "three"].into());
    // Str join
    assert_eq!(Str::from(".").join(["192", "168", "0", "1"].into()), "192.168.0.1".into());

    // Deque push back, supports both back and front push, pop, and element reference
    assert_eq!(Deque::from([1, 2, 3, 4]).push_back(5), &[1, 2, 3, 4, 5].into());
    // Deque shifts to right (or left), very vivid!
    assert_eq!(Deque::from([1, 2, 3, 4, 5]) >> 1, [5, 1, 2, 3, 4].into());

    // Fraction addition
    assert_eq!(Fraction::from((1, 2)) + Fraction::from((1, 3)), (5, 6).into());
    // Fraction modulo
    assert_eq!(Fraction::from((1, 2)) % Fraction::from((1, 3)), (1, 6).into());

    // Arbitrarily nested multiple layers of types
    let dict: Dict<Str, List<Int>> = [
        ("first".into(), ["123".into(), "456".into()].into()),
        ("second".into(), ["789".into()].into()),
        ("third".into(), ["12345678987654321".into(), "5".into()].into()),
    ]
    .into();
    assert_eq!(format!("{dict}"), "{\"first\": [123, 456], \"second\": [789], \"third\": [12345678987654321, 5]}");
    assert_eq!(dict.len(), 3);
    assert_eq!(dict.keys(), ["first".into(), "second".into(), "third".into()].into());
    assert_eq!(dict[&"third".into()][-1].factorial(), 120.into());
}
