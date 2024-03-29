# PyInRs

_A Rust type library that is as easy to use as Python built-in types._

### 1. Attribute

- Name: PyInRs.
- Language: Rust, requires version rustc >= `1.75.0`.
- Goal: Write a Rust type library that is as easy to use as Python built-in types.
- Module: List, Set, Dict, Int, Str, Deque, Fraction
- Style: Follow Rust's official recommended style.
- Test: Using [rstest](https://crates.io/crates/rstest) for unit testing and ensure that all tests passed.
- Security: There is no `unsafe` code block.
- Document: Using `cargo doc --open` to open documents.

### 2. Feature

- Simple: Stay simple, stay young. While ensuring friendly and robust, try to be concise and easy to maintain and read.
- Friendly: Provides many convenient functions. For example, `List` and `Str` both support negative subscript like Python.
- Robust: There are corresponding checks for the addition, deletion, modification, and inspection of containers.
- Elegance: With careful design, it can be used as conveniently as Python's built-in types. Very Pythonic.

### 3. Usage

To use it, add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
pyinrs = "1"
```

There are a total of 7 classes, refer to the 7 commonly used classes in Python:

| Type in PyInRs | Type in Python       |
| -------------- | -------------------- |
| `List<T>`      | `list`               |
| `Set<T>`       | `set`                |
| `Dict<K, V>`   | `dict`               |
| `Int`          | `int`                |
| `Str`          | `str`                |
| `Deque<T>`     | `collections.deque`  |
| `Fraction`     | `fractions.Fraction` |

Some simple examples:

```rust
use pyinrs::*;

// List index, supports negative subscript
List::from([1, 2, 3, 4, 5])[-1]; // 5
// List uniquify
List::from([1, 2, 3, 1, 2, 3, 1, 2, 3]).uniquify(); // [1, 2, 3]

// Adding elements to Set
Set::from([1, 2, 3, 4]).add(5); // {1, 2, 3, 4, 5}
// Intersection of Sets, supports intersection, union, difference, and symmetric difference
Set::from([1, 2, 3, 4, 5]) & Set::from([1, 3, 5, 7, 9]); // {1, 3, 5}

// Dict assign value for key
Dict::from([("one", 1), ("two", 2), ("three", 3)])[&"one"] = 11; // {one: 11, three: 3, two: 2}
// Dict get values
Dict::from([("one", 1), ("two", 2), ("three", 3)]).values(); // {1, 2, 3}

// Int modular power, very fast
Int::from("1024").pow(&"1024".into(), &"100".into()); // 76
// Int factorial
Int::from("5").factorial().factorial(); // 6689502913449127057588118054090372586752746333138...

// Str split
Str::from("one, two, three").split(", "); // ["one", "two", "three"]
// Str join
Str::from(".").join(["192", "168", "0", "1"].into()); // "192.168.0.1"

// Deque push back, supports both back and front push, pop, and element reference
Deque::from([1, 2, 3, 4]).push_back(5); // <1, 2, 3, 4, 5>
// Deque shifts to right (or left), very vivid!
Deque::from([1, 2, 3, 4, 5]) >> 1; // <5, 1, 2, 3, 4>

// Fraction addition
Fraction::from((1, 2)) + Fraction::from((1, 3)); // 5/6
// Fraction modulo
Fraction::from((1, 2)) % Fraction::from((1, 3)); // 1/6

// Arbitrarily nested multiple layers of types
let dict: Dict<Str, List<Int>> = [
    ("first".into(), ["123".into(), "456".into()].into()),
    ("second".into(), ["789".into()].into()),
    ("third".into(), ["12345678987654321".into(), "5".into()].into()),
].into(); // {"first": [123, 456], "second": [789], "third": [12345678987654321, 5]}
dict.len(); // 3;
dict.keys(); // {"first", "second", "third"}
dict[&"third".into()][-1].factorial(); // 120
```

If you want to use a similar library in C++, please see: [PyInCpp](https://github.com/chen-qingyu/pyincpp).
