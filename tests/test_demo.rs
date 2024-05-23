use pyinrs::*;
use rstest::rstest;

#[rstest]
fn usage() {
    // List support negative index
    assert_eq!(List::from([1, 2, 3, 4, 5])[-1], 5);
    // List uniquify
    assert_eq!(List::from([1, 2, 3, 1, 2, 3, 1, 2, 3]).uniquify(), [1, 2, 3].into());

    // test whether a Set is proper subset of another Set
    assert_eq!(Set::from([5, 1]) < Set::from([1, 2, 3, 4, 5]), true);
    // intersection of Sets, support intersection, union, difference, and symmetric difference
    assert_eq!(Set::from([1, 2, 3, 4, 5]) & Set::from([1, 3, 5, 7, 9]), [1, 3, 5].into());

    // Dict access
    assert_eq!(Dict::from([("one", 1), ("two", 2), ("three", 3)])[&"one"], 1);
    // Dict get values as a Set
    assert_eq!(
        Dict::from([("one", 1), ("two", 2), ("three", 3)]).values().collect::<Set<&i32>>(),
        [&1, &2, &3].into()
    );

    // Int basic operation, support +, -, *, /, % and compare
    assert_eq!(Int::from("18446744073709551617") + Int::from("18446744073709551617"), "36893488147419103234".into());
    // Int increment, after my optimization, much faster than `+= 1`
    assert_eq!(Int::from("99999999999999").inc(), &"100000000000000".into());
    // Int modular power, very fast
    assert_eq!(Int::pow_mod(&"1024".into(), &"1024".into(), &"100".into()), "76".into());
    // Int factorial
    assert_eq!(Int::from("5").factorial().factorial(), "6689502913449127057588118054090372586752746333138029810295671352301633557244962989366874165271984981308157637893214090552534408589408121859898481114389650005964960521256960000000000000000000000000000".into());
    // get random Int, using hardware device to generate true random integer if possible
    assert_eq!(Int::random(4300).digits(), 4300);
    // calculate the next prime that greater than self
    assert_eq!(Int::new().next_prime(), "2".into());

    // Str split
    assert_eq!(Str::from("one, two, three").split(", "), ["one", "two", "three"].into());
    // Str join
    assert_eq!(Str::from(".").join(["192", "168", "0", "1"].into()), "192.168.0.1".into());

    // Complex addition
    assert_eq!(Complex::from((1., 2.)) + Complex::from((1., 3.)), Complex::from((2., 5.)));
    // Complex power
    assert_eq!(
        Complex::pow(&Complex::from((1., 2.)), &Complex::from((-1., 2.))),
        Complex::from((0.04281551979798478, 0.023517649351954585))
    );

    // Deque element reference
    assert_eq!(Deque::from([1, 2, 3, 4, 5]).front(), Some(&1));
    // Deque rotate to right (or left), very vivid!
    assert_eq!(Deque::from([1, 2, 3, 4, 5]) >> 1, [5, 1, 2, 3, 4].into());

    // Fraction addition
    assert_eq!(Fraction::from((1, 2)) + Fraction::from((1, 3)), (5, 6).into());
    // Fraction modulo
    assert_eq!(Fraction::from((1, 2)) % Fraction::from((1, 3)), (1, 6).into());
}

#[rstest]
fn advantage() {
    // 1. All types can be printed and easily combined:
    let dict: Dict<Str, List<Int>> = [
        ("first".into(), ["123".into(), "456".into()].into()),
        ("second".into(), ["789".into()].into()),
        ("third".into(), ["12345678987654321".into(), "5".into()].into()),
    ]
    .into();
    assert_eq!(format!("{dict}"), "{\"first\": [123, 456], \"second\": [789], \"third\": [12345678987654321, 5]}");
    assert_eq!(dict.keys().cloned().collect::<Set<Str>>(), ["first".into(), "second".into(), "third".into()].into());
    assert_eq!(dict[&"third".into()][-1].factorial(), 120.into());

    // 2. All container types are iterable:
    for (k, v) in Dict::from([(1, 1), (2, 4), (3, 9)]) {
        assert_eq!(k * k, v);
    }

    // 3. All immutable types are hashable:
    use std::collections::HashSet;
    let _set1: HashSet<Int> = HashSet::from(["1".into(), "2".into(), "3".into(), "18446744073709551617".into()]);
    let _set2: HashSet<Str> = HashSet::from(["hello".into(), "pyincpp".into()]);
    let _set3: HashSet<Fraction> = HashSet::from([(1, 2).into(), (3, 4).into()]);

    // 4. Using pyinrs::Fraction in mymatrix to display accurate matrix.
    use mymatrix::Matrix;

    let a = Matrix::from([[1, 2], [3, 4]]);
    let b = Matrix::create(2, 2, 0.into());
    let c = Matrix::create(2, 2, 1.into());
    let d = Matrix::eye(2);

    assert_eq!(format!("{}", ((a + b) * (c + d)).inv().unwrap()), "[[\n-11/6 5/6;\n5/3 -2/3;\n]]");
}
