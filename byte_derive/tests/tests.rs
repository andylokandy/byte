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

#[test]
fn test_named_struct() {
    let data: Named = Named {
        id: 0x12345678,
        timestamp: 1234.5678,
        str: "hello",
    };
    let buf = &mut [0; 18];
    data.clone().try_write(buf, LE).unwrap();
    assert_eq!(Ok((data, 18)), Named::try_read(buf, LE));
}

#[derive(Debug, Clone, PartialEq, TryWrite, TryRead)]
struct NoLifetime {
    id: u32,
    timestamp: f64,
}

#[test]
fn test_no_lifetime_struct() {
    let data = NoLifetime {
        id: 0x12345678,
        timestamp: 1234.5678,
    };
    let buf = &mut [0; 12];
    data.clone().try_write(buf, LE).unwrap();
    assert_eq!(Ok((data, 12)), NoLifetime::try_read(buf, LE));
}

#[derive(Debug, Clone, PartialEq, TryWrite, TryRead)]
struct FieldDependent<'a> {
    len: usize,
    #[byte(read_ctx = Str::Len(len), write_ctx = NONE)]
    str: &'a str,
}

#[test]
fn test_len_dependent() {
    let data = FieldDependent {
        len: 2,
        str: "hello",
    };
    let buf = &mut [0; 13];
    data.clone().try_write(buf, LE).unwrap();
    assert_eq!(
        Ok((FieldDependent { len: 2, str: "he" }, 10)),
        FieldDependent::try_read(buf, LE)
    );
}

#[derive(Debug, Clone, PartialEq, TryRead, TryWrite)]
struct Tuple<'a>(
    u32,
    f64,
    #[byte(read_ctx = Str::Delimiter(0), write_ctx = NONE)] &'a str,
);

#[test]
fn test_tuple_struct() {
    let data: Tuple = Tuple(0x12345678, 1234.5678, "hello");
    let buf = &mut [0; 18];
    data.clone().try_write(buf, LE).unwrap();
    assert_eq!(Ok((data, 18)), Tuple::try_read(buf, LE));
}

#[derive(Debug, Clone, PartialEq, TryRead, TryWrite)]
struct Empty;

#[test]
fn test_empty_struct() {
    let data: Empty = Empty;
    let buf = &mut [0; 0];
    data.clone().try_write(buf, LE).unwrap();
    assert_eq!(Ok((data, 0)), Empty::try_read(buf, LE));
}
