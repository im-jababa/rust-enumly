# enumly

[![Github](https://img.shields.io/badge/github-im--jababa%2Frust--enumly-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/im-jababa/rust-enumly)
[![Crates.io](https://img.shields.io/badge/crates.io-enumly-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/enumly)
[![Docs.rs](https://img.shields.io/badge/docs.rs-enumly-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/enumly)
[![Build](https://img.shields.io/github/actions/workflow/status/im-jababa/rust-enumly/rust.yml?branch=main&style=for-the-badge)](https://github.com/im-jababa/rust-enumly/actions?query=branch%3Amain)

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
