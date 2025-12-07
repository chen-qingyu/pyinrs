# PyInRs

_A Rust type library that is as easy to use as Python built-in types._

## 1. Attribute

- Name: PyInRs (means **Py**thon **in** **R**u**s**t)
- Goal: Provide a Rust type library that is as easy to use as Python built-in types
- Module: List, Set, Dict, Int, Str, Complex, Deque, Fraction, Decimal

## 2. Feature

- Simple: Stay simple, stay young. While ensuring friendly and robust, try to be concise and easy to maintain and read
- Friendly: With my careful design, it can be used as conveniently as Python's built-in types. Very Pythonic
- Robust: There are corresponding checks for the insert, remove, modify, and access of containers
- Efficient: The performance of the parts with the same function as the standard library is almost the same
- Secure: Tested using [rstest](https://crates.io/crates/rstest) and no `unsafe` code block to ensure no security issues

## 3. Usage

To use it, add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
pyinrs = "1"
```

There are a total of 9 classes, refer to commonly used classes in Python:

| Type in PyInRs | Type in Python       |
| -------------- | -------------------- |
| `List<T>`      | `list`               |
| `Set<T>`       | `set`                |
| `Dict<K, V>`   | `dict`               |
| `Int`          | `int`                |
| `Str`          | `str`                |
| `Complex`      | `complex`            |
| `Deque<T>`     | `collections.deque`  |
| `Fraction`     | `fractions.Fraction` |
| `Decimal`      | `decimal.Decimal`    |

Some simple examples:

```rust
use pyinrs::*;

// List support negative index
assert_eq!(List::from([1, 2, 3, 4, 5])[-1], 5);
// List uniquify
assert_eq!(List::from([1, 2, 3, 1, 2, 3, 1, 2, 3]).uniquify(), List::from([1, 2, 3]));

// test whether a Set is proper subset of another Set
assert_eq!(Set::from([5, 1]) < Set::from([1, 2, 3, 4, 5]), true);
// intersection of Sets, support intersection, union, difference, and symmetric difference
assert_eq!(Set::from([1, 2, 3, 4, 5]) & Set::from([1, 3, 5, 7, 9]), Set::from([1, 3, 5]));

// Dict access
assert_eq!(Dict::from([("one", 1), ("two", 2), ("three", 3)])[&"one"], 1);
// Dict get values as a Set
assert_eq!(Dict::from([("one", 1), ("two", 2), ("three", 3)]).values().collect::<Set<&i32>>(), Set::from([&1, &2, &3]));

// Int basic operation, support +, -, *, /, % and compare
assert_eq!(Int::from("18446744073709551617") + Int::from("18446744073709551617"), Int::from("36893488147419103234"));
// Int increment, after my optimization, much faster than `+= 1`
assert_eq!(Int::from("99999999999999").inc(), &Int::from("100000000000000"));
// Int modular power, very fast
assert_eq!(Int::pow_mod(&"1024".into(), &"1024".into(), &"100".into()), Int::from("76"));
// Int factorial
assert_eq!(Int::from("5").factorial().factorial(), Int::from("6689502913449127057588118054090372586752746333138029810295671352301633557244962989366874165271984981308157637893214090552534408589408121859898481114389650005964960521256960000000000000000000000000000"));
// get random Int of specified number of digits
assert_eq!(Int::random(1024).digits(), 1024);
// calculate the next prime that greater than self
assert_eq!(Int::from("7").next_prime(), Int::from("11"));
// calculate the tetration
assert_eq!(Int::hyperoperation(&"4".into(), &"3".into(), &"3".into()), Int::from("7625597484987"));

// Str split
assert_eq!(Str::from("one, two, three").split(", "), List::from(["one", "two", "three"]));
// Str join
assert_eq!(Str::from(".").join(List::from(["192", "168", "0", "1"])), Str::from("192.168.0.1"));

// Complex addition
assert_eq!(Complex::from((1., 2.)) + Complex::from((1., 3.)), Complex::from((2., 5.)));
// Complex power
assert_eq!(Complex::pow(&Complex::from((1., 2.)), &Complex::from((-1., 2.))), Complex::from((0.04281551979798478, 0.023517649351954585)));

// Deque element reference
assert_eq!(Deque::from([1, 2, 3, 4, 5]).front(), Some(&1));
// Deque rotate to right (or left), very vivid!
assert_eq!(Deque::from([1, 2, 3, 4, 5]) >> 1, Deque::from([5, 1, 2, 3, 4]));

// Fraction addition
assert_eq!(Fraction::from("1/2") + Fraction::from("1/3"), Fraction::from("5/6"));
// Fraction modulo
assert_eq!(Fraction::from("1/2") % Fraction::from("1/3"), Fraction::from("1/6"));

// Decimal calculate exact result
assert_eq!(Decimal::from("0.1") + Decimal::from("0.2"), Decimal::from("0.3"));
// Decimal keeps repeating parts exactly
assert_eq!(Decimal::from("0.~3").as_fraction(), Fraction::from((1, 3)));
```

## 4. Advantage

The advantage of PyInRs is that it combines the high performance of Rust with the ease of use of Python, and can also be easily combined with other libraries, for example:

```rust
use pyinrs::*;

// 1. All types can be printed and easily combined:
let dict: Dict<Str, List<Int>> = [
    ("first".into(), ["123".into(), "456".into()].into()),
    ("second".into(), ["789".into()].into()),
    ("third".into(), ["12345678987654321".into(), "5".into()].into()),
].into();
assert_eq!(format!("{dict}"), "{\"first\": [123, 456], \"second\": [789], \"third\": [12345678987654321, 5]}");
assert_eq!(dict.keys().collect::<Set<&Str>>(), Set::from([&"first".into(), &"second".into(), &"third".into()]));
assert_eq!(dict[&"third".into()][-1].factorial(), Int::from(120));

// 2. All container types are iterable:
for (k, v) in Dict::from([(1, 1), (2, 4), (3, 9)]) {
    assert_eq!(k * k, v);
}

// 3. All immutable types are hashable:
use std::collections::HashSet;
let _set1: HashSet<Int> = HashSet::from(["1".into(), "2".into(), "3".into(), "18446744073709551617".into()]);
let _set2: HashSet<Str> = HashSet::from(["hello".into(), "pyinrs".into()]);
let _set3: HashSet<Fraction> = HashSet::from(["1/2".into(), "3/4".into()]);
let _set4: HashSet<Complex> = HashSet::from([(1., 2.).into(), (3., 4.).into()]);
let _set5: HashSet<Decimal> = HashSet::from(["0.5".into(), "0.~3".into()]);

// 4. Using pyinrs::Fraction in mymatrix to display accurate matrix.
use mymatrix::Matrix;

let a = Matrix::from([[1, 2], [3, 4]]);
let b = Matrix::zeros(2, 2);
let c = Matrix::ones(2, 2);
let d = Matrix::identity(2);

assert_eq!(format!("{}", ((a + b) * (c + d)).inv().unwrap()),
"[
-11/6   5/6
  5/3  -2/3
]"
);

// 5. Using pyinrs::Decimal to calculate infinite cyclic decimals.
assert_eq!(Decimal::from("0.~3") + Decimal::from("0.~6"), Decimal::from("1.0"));
assert_eq!(Decimal::from("0.~9"), Decimal::from("1.0"));
```

If you want to use a similar library in C++, please see: [PyInCpp](https://github.com/chen-qingyu/pyincpp).
