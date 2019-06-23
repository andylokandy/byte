# `Byte`

[![build status](https://travis-ci.org/andylokandy/byte.svg?branch=master)](https://travis-ci.org/andylokandy/byte)
[![crates.io](https://img.shields.io/crates/v/byte.svg)](https://crates.io/crates/byte)
[![docs.rs](https://docs.rs/byte/badge.svg)](https://docs.rs/byte)

A low-level, zero-copy and panic-free serializer and deserializer for binary.

### [**Documentation**](https://docs.rs/byte)

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
byte = "0.2"
```

Next, add this to your crate root:

```rust
extern crate byte;
```

`Byte` is `no_std` library; it can directly be used in a `#![no_std]` situation or crate.


# Overview

`Byte` is designed to encode or decode binary data in a fast and low-level way.
A classical use case is I2C communication en/decoding.

`Byte` provides two core traits `TryRead` and `TryWrite`.
Types that implement these traits can be serialize into or deserialize from byte slices.

The library is meant to be simple, and it will always be.


# Example

```rust
use byte::*;

let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef];

let offset = &mut 0;
let num = bytes.read_with::<u32>(offset, BE).unwrap();
assert_eq!(num, 0xdeadbeef);
assert_eq!(*offset, 4);
```

```rust
use byte::*;
use byte::ctx::{Str, NULL};

let bytes: &[u8] = b"hello, world!\0dump";

let offset = &mut 0;
let str = bytes.read_with::<&str>(offset, Str::Delimiter(NULL)).unwrap();
assert_eq!(str, "hello, world!");
assert_eq!(*offset, 14);
```


## Contribution

All kinds of contribution are welcomed.

- **Issus.** Feel free to open an issue when you find typos, bugs, or have any question.
- **Pull requests**. New collection, better implementation, more tests, more documents and typo fixes are all welcomed.


## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)