# enumly
Provides a procedural macro that exposes a compile-time static list of all variants of an enum.

## Usage
```rust
use enumly::Enumly;

#[derive(Enumly, Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

assert_eq!(Color::COUNT, 3);
assert_eq!(Color::VARIANTS, &[Color::Red, Color::Green, Color::Blue]);
```

## What does not work
Non-unit variants are rejected at compile time:
```rust
use enumly::Enumly;

#[derive(Enumly)]
enum Bad {
    Tuple(u8),
    Struct { value: u8 },
}
```
