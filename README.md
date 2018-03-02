# `Byte`

[![build status](https://travis-ci.org/andylokandy/byte.svg?branch=master)](https://travis-ci.org/andylokandy/byte)
[![crates.io](https://img.shields.io/crates/v/byte.svg)](https://crates.io/crates/byte)
[![docs.rs](https://docs.rs/byte/badge.svg)](https://docs.rs/byte)

A low-level, zero-copy, panic-free, binary serializer and deserializer

This crate is inspired by [**m4b/scroll**](https://github.com/m4b/scroll)

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

`Byte` is mainly used to encode and decode binary data with standard or protocol,
such as network TCP packages and hardware communication packages.
It's similar to crate `nom` but more ligthweight and specialized for operating binary in low-level and hardware programing.

`Byte` delivers two core traits `TryRead` and `TryWrite`.
Types implement these traits can be serialize into or deserialize from byte slices.
Byte slices `[u8]` derives methods `read()` and `write()` to serialize, deserialize and handle offset.

Small and general is kept in mind in this library.
For example, `Byte` can take byte slice from [**MMap**](https://crates.io/crates/mmap) to read binary file,
or take heap-allocated byte buffer from [**Bytes**](https://github.com/carllerche/bytes).


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