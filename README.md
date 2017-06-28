# `Byte`

[![build status](https://travis-ci.org/goandylok/byte.svg?branch=master)](https://travis-ci.org/goandylok/byte)
[![crates.io](https://img.shields.io/crates/v/byte.svg)](https://crates.io/crates/byte)
[![docs.rs](https://docs.rs/byte/badge.svg)](https://docs.rs/byte)

A low-level, zero-copy, panic-free, binary serializer and deserializer (parser and encoder)

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


## Overview

`Byte` is mainly used to encode and decode binary data with standard or protocol, 
such as network TCP packages and hardware communication packages. 
So it's similar to crate `nom` but more ligthweight and specialized for operating binary in low-level or hardware programing.

`Byte` delivers two core traits `TryRead` and `TryWrite`. 
Types implement these traits can be serialize into or deserialize from byte slices.
Byte slices `[u8]` derives methods `read()` and `write()` to serialize, deserialize and handle offset.

All functionality is kept minimum in order to work in more situations. 
For example, `Byte` can take byte slice from [**MMap**](https://crates.io/crates/mmap) to read binary file, 
or take heap-allocated byte buffer from [**Bytes**](https://github.com/carllerche/bytes). 


## Example

`Byte` consumes byte slice continuously. The first parameter of `read` is offset, 
instructing the position to begin, and it must be a mutable referece of usize, 
which will be increaed by the size operation consumed. 
Serializing types usually requires some context such as the endian for numbers, 
in such situations, `read_with` is used and we can pass context as the second parameter.

### Primitives

```rust
use byte::*;

let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef];
let offset = &mut 0;

let num = bytes.read_with::<u32>(offset, BE).unwrap();
assert_eq!(num, 0xdeadbeef);
assert_eq!(*offset, 4);
```

`Byte` supports language primitives by default. 
- `&str` (with context `Str`)
- `&[u8]` (with context `Byte`)
- `u8`, `i8`, `u64`, `f64` ... (with context `Endian`)
- `bool`
- ...

`&str` and `&[u8]` have references to the byte slice so there is no copy when `read` and it has the same lifetime as the byte slice.

```rust
use byte::*;
use byte::ctx::{Str, NULL};

let bytes: &[u8] = b"hello, world!\0more";
let str: &str = bytes.read_with(&mut 0, Str::Delimiter(NULL)).unwrap();
assert_eq!(str, "hello, world!");
```

## Define custom serializable type

In this example, we defined a custom type `Header`, which have a varibal-length name and a `bool` field. 
We implement `TryRead` and `TryWrite` to enable this type to be serialzed and deserialized. 

### Byte Representation
```
|       | Length of name (Big Endian) |                Name              | Enabled |
| ----- | --------------------------- | ---- | ---- | ---- | ---- | ---- | ------- |
| Byte  | 0            | 5            | 'H'  | 'E'  | 'L'  | 'L'  | 'O'  | 0       |
```

Note that the passed-in `bytes` is implicitly splitted by offset and should be read at head. 
And the type `Result` is an alias defind in `Byte` as `core::result::Result<(T, size), byte::Error>`, 
where the size is the number of bytes `read` or `write` consumed and it will be used to incread the offset.

```rust
use byte::*;
use byte::ctx::*;

struct Header<'a> {
    name: &'a str,
    enabled: bool,
}

impl<'a> TryRead<'a, Endian> for Header<'a> {
    fn try_read(bytes: &'a [u8], endian: Endian) -> Result<(Self, usize)> {
        let offset = &mut 0;

        let name_len = bytes.read_with::<u16>(offset, endian)? as usize;
        let header = Header {
            name: bytes.read_with::<&str>(offset, Str::Len(name_len))?,
            enabled: bytes.read(offset)?,
        };

        Ok((header, *offset))
    }
}

impl<'a> TryWrite<Endian> for Header<'a> {
    fn try_write(self, bytes: &mut [u8], endian: Endian) -> Result<usize> {
        let offset = &mut 0;

        bytes.write_with(offset, self.name.len() as u16, endian)?;
        bytes.write(offset, self.name)?;
        bytes.write(offset, self.enabled)?;

        Ok(*offset)
    }
}
```

### Usage

```rust
let bytes = [0, 5, b"H"[0], b"E"[0], b"L"[0], b"L"[0], b"O"[0], 0];

let header: Header = bytes.read_with(&mut 0, BE).unwrap();
assert_eq!(header.name, "HELLO");
assert_eq!(header.enabled, false);

let mut write = [0u8; 8];
write.write_with(&mut 0, header, BE).unwrap();
assert_eq!(write, bytes);
```


## Contribution

All kinds of contribution are welcomed.

- **Issus.** Feel free to open an issue when you find typos, bugs, or have any question.
- **Pull requests**. New collection, better implementation, more tests, more documents and typo fixes are all welcomed.


## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)