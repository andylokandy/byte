# byte_derive
Derive macros for the `byte` crate.

```rust
use byte::{
    ctx::{Str, NONE},
    TryRead, TryWrite, LE,
};

#[derive(Debug, Clone, PartialEq, TryWrite, TryRead)]
struct Named<'a> {
    id: u32,
    timestamp: f64,
    #[byte(read_ctx = Str::Delimiter(0), write_ctx = NONE)]
    str: &'a str,
}

fn main() {
    let data: Named = Named {
        id: 0x12345678,
        timestamp: 1234.5678,
        str: "hello",
    };
    let buf = &mut [0; 18];
    data.clone().try_write(buf, LE).unwrap();
    assert_eq!((data, 18), Named::try_read(buf, LE).unwrap());
}
```
