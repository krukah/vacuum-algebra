# Vacuum

[![Rust](https://github.com/username/vacuum/actions/workflows/rust.yml/badge.svg)](https://github.com/username/vacuum/actions/workflows/rust.yml)
[![Code Coverage](https://codecov.io/gh/username/vacuum/branch/main/graph/badge.svg)](https://codecov.io/gh/username/vacuum)
[![Documentation](https://github.com/username/vacuum/actions/workflows/doc.yml/badge.svg)](https://username.github.io/vacuum/)

A Rust library for manipulating expressions composed of ladder operators.

## Usage

```rust
use vacuum::expression::Expression;

fn main() {
    let expression = Expression::from("010101");
    println!("Expression: {}", expression);
    println!("Evaluation: {}", expression.expectation());
}
```

## Features

- Create and manipulate expressions composed of ladder operators
- Calculate expectation values of expressions
- Split and concatenate expressions

## Testing

Run the tests with:

```bash
cargo test
```

## License

MIT 