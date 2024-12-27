# PyInRs

_A Rust type library that is as easy to use as Python built-in types._

## 1. Attribute

- Name: PyInRs
- Goal: Provide a Rust type library that is as easy to use as Python built-in types
- Module: List, Set, Dict, Int, Str, Complex, Deque, Fraction

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

There are a total of 8 classes, refer to commonly used classes in Python:

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

Some simple examples:

```rust
use pyinrs::*;

// List support negative index
List::from([1, 2, 3, 4, 5])[-1]; // 5
// List uniquify
List::from([1, 2, 3, 1, 2, 3, 1, 2, 3]).uniquify(); // [1, 2, 3]

// test whether a Set is proper subset of another Set
Set::from([5, 1]) < Set::from([1, 2, 3, 4, 5]); // true
// intersection of Sets, support intersection, union, difference, and symmetric difference
Set::from([1, 2, 3, 4, 5]) & Set::from([1, 3, 5, 7, 9]); // {1, 3, 5}

// Dict access
Dict::from([("one", 1), ("two", 2), ("three", 3)])[&"one"]; // 1
// Dict get values as a Set
Dict::from([("one", 1), ("two", 2), ("three", 3)]).values().collect::<Set<&i32>>(); // {1, 2, 3}

// Int basic operation, support +, -, *, /, % and compare
Int::from("18446744073709551617") + Int::from("18446744073709551617"); // 36893488147419103234
// Int increment, after my optimization, much faster than `+= 1`
Int::from("99999999999999").inc(); // 100000000000000
// Int modular power, very fast
Int::pow_mod(&"1024".into(), &"1024".into(), &"100".into()); // 76
// Int factorial
Int::from("5").factorial().factorial(); // 66895029134491270575881180540903725867527463...
// get random Int of specified number of digits
Int::random(1024); // 23795759214348387514699522496327832510939573336290225099601421311...
// calculate the next prime that greater than self
Int::from("7").next_prime(); // 11
// calculate the tetration
Int::hyperoperation(&"4".into(), &"3".into(), &"3".into()); // 7625597484987

// Str split
Str::from("one, two, three").split(", "); // ["one", "two", "three"]
// Str join
Str::from(".").join(["192", "168", "0", "1"].into()); // "192.168.0.1"

// Complex addition
Complex::from((1., 2.)) + Complex::from((1., 3.)); // (2+5j)
// Complex power
Complex::pow(&Complex::from((1., 2.)), &Complex::from((-1., 2.))); // (0.04281551979798478+0.023517649351954585j)

// Deque element reference
Deque::from([1, 2, 3, 4, 5]).front(); // 1
// Deque rotate to right (or left), very vivid!
Deque::from([1, 2, 3, 4, 5]) >> 1; // <5, 1, 2, 3, 4>

// Fraction addition
Fraction::from((1, 2)) + Fraction::from((1, 3)); // 5/6
// Fraction modulo
Fraction::from((1, 2)) % Fraction::from((1, 3)); // 1/6
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
print!("{dict}"); // {"first": [123, 456], "second": [789], "third": [12345678987654321, 5]}
dict.keys().cloned().collect::<Set<Str>>(); // {"first", "second", "third"}
dict[&"third".into()][-1].factorial(); // 120

// 2. All container types are iterable:
for (k, v) in Dict::from([(1, 1), (2, 4), (3, 9)]) {
    assert_eq!(k * k, v);
}

// 3. All immutable types are hashable:
use std::collections::HashSet;
let _set1: HashSet<Int> = HashSet::from(["1".into(), "2".into(), "3".into(), "18446744073709551617".into()]);
let _set2: HashSet<Str> = HashSet::from(["hello".into(), "pyinrs".into()]);
let _set3: HashSet<Fraction> = HashSet::from([(1, 2).into(), (3, 4).into()]);

// 4. Using pyinrs::Fraction in mymatrix to display accurate matrix.
use mymatrix::Matrix;

let a = Matrix::from([[1, 2], [3, 4]]);
let b = Matrix::zeros(2, 2);
let c = Matrix::ones(2, 2);
let d = Matrix::identity(2);

print!("{}", ((a + b) * (c + d)).inv().unwrap());
/*
[
-11/6   5/6
  5/3  -2/3
]
*/
```

If you want to use a similar library in C++, please see: [PyInCpp](https://github.com/chen-qingyu/pyincpp).
