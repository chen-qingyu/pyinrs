# PyInRs

_A Rust type library that is as easy to use as Python built-in types._

### 1. Attribute

- Name: PyInRs.
- Language: Rust, requires version rustc >= `1.75.0`.
- Goal: Write a Rust type library that is as easy to use as Python built-in types.
- Module: List, Set, Map, Integer, String, Tuple, Deque, Fraction
- Style: Follow Rust's official recommended style.
- Test: Using [rstest](https://github.com/la10736/rstest) for unit testing and ensure that all tests passed.
- Security: There is no `unsafe` code block.
- Document: Using `cargo doc --open` to open documents.
- Run: Using `cargo run` to build and run all tests.

### 2. Feature

- Simple: Stay simple, stay young. While ensuring usability and robustness, try to be concise and easy to maintain and read.
- Friendly: Provides many convenient functions. For example, String class provides replace, split, find and other operations like Python's str, and List class and String class both support negative subscript like Python.
- Robust: A secure expansion mechanism to prevent overflow. There are corresponding checks for the addition, deletion, modification, and inspection of containers. Checking will have an impact on performance, but this library is not pursuing performance, but simplicity, usability, and robustness.
- Elegance: With my careful design, it can be used as conveniently as Python's built-in types. Very Pythonic.

### 3. Usage

To use it, add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
pyinrs = "0"
```

There are a total of 8 classes (in plan, 1 for now), refer to the 8 commonly used classes in Python:

| Type in PyInRs | Type in Python       |     |
| -------------- | -------------------- | --- |
| `List<T>`      | `list`               |     |
| `Set<T>`       | `set`                |     |
| `Map<K, V>`    | `dict`               |     |
| `Integer`      | `int`                |     |
| `String`       | `str`                |     |
| `Tuple<Ts...>` | `tuple`              |     |
| `Deque<T>`     | `collections.deque`  |     |
| `Fraction`     | `fractions.Fraction` | √   |

Some simple examples:

```rust
use pyinrs::*;

// Fraction addition
Fraction(1, 2) + Fraction(1, 3) // 5/6
// Fraction modulo
Fraction(1, 2) % Fraction(1, 3) // 1/6
```
