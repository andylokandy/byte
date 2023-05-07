#[macro_use]
extern crate quickcheck;
extern crate byte;
extern crate byteorder;

use byte::ctx::*;
use byte::*;
use byteorder::*;

#[test]
fn test_str() {
    let bytes: &[u8] = b"abcd\0efg";

    let mut offset = 0;
    assert_eq!(
        bytes
            .read_with::<&str>(&mut offset, Str::Delimiter(NULL))
            .unwrap(),
        "abcd"
    );
    assert_eq!(offset, 5);

    let bytes: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    assert_eq!(
        TryRead::try_read(bytes, Str::Len(15)).unwrap(),
        ("abcdefghijklmno", 15)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::Len(26)).unwrap(),
        ("abcdefghijklmnopqrstuvwxyz", 26)
    );

    assert!(bytes.read_with::<&str>(&mut 0, Str::Len(27)).is_err());
    assert!(bytes.read_with::<&str>(&mut 27, Str::Len(0)).is_err());
    assert!(bytes.read_with::<&str>(&mut 26, Str::Len(1)).is_err());
}

#[test]
fn test_str_delimitor() {
    let bytes: &[u8] = b"";
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 0)).unwrap(),
        ("", 0)
    );

    let bytes: &[u8] = b"abcdefg";
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 6)).unwrap(),
        ("abcdef", 6)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 7)).unwrap(),
        ("abcdefg", 7)
    );

    let bytes: &[u8] = b"\0abcdefg";
    assert_eq!(
        TryRead::try_read(bytes, Str::Delimiter(NULL)).unwrap(),
        ("", 1)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 0)).unwrap(),
        ("", 0)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 1)).unwrap(),
        ("", 1)
    );

    let bytes: &[u8] = b"abcd\0efg";
    assert_eq!(
        TryRead::try_read(bytes, Str::Delimiter(NULL)).unwrap(),
        ("abcd", 5)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 4)).unwrap(),
        ("abcd", 4)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 5)).unwrap(),
        ("abcd", 5)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 6)).unwrap(),
        ("abcd", 5)
    );

    let bytes: &[u8] = b"abcdefg\0";
    assert_eq!(
        TryRead::try_read(bytes, Str::Delimiter(NULL)).unwrap(),
        ("abcdefg", 8)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 8)).unwrap(),
        ("abcdefg", 8)
    );
    assert_eq!(
        TryRead::try_read(bytes, Str::DelimiterUntil(NULL, 20)).unwrap(),
        ("abcdefg", 8)
    );

    let bytes: &[u8] = b"";
    assert!(bytes
        .read_with::<&str>(&mut 0, Str::Delimiter(NULL))
        .is_err());
    assert!(bytes
        .read_with::<&str>(&mut 0, Str::DelimiterUntil(NULL, 1))
        .is_err());

    let bytes: &[u8] = b"abcdefg";
    assert!(bytes
        .read_with::<&str>(&mut 0, Str::DelimiterUntil(NULL, 8))
        .is_err());
    assert!(bytes
        .read_with::<&str>(&mut 0, Str::Delimiter(NULL))
        .is_err());
}

#[test]
fn test_str_write() {
    let mut bytes = [0; 20];
    let mut offset = 0;
    bytes.write(&mut offset, "hello world!").unwrap();
    assert_eq!(offset, 12);
    assert_eq!(&bytes[..offset], b"hello world!" as &[u8]);

    let bytes = &mut [0; 10];
    assert!(bytes.write(&mut 0, "hello world!").is_err());
}

#[test]
fn test_bytes() {
    let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef];
    assert_eq!(
        TryRead::try_read(&bytes, Bytes::Len(4)).unwrap(),
        (&bytes[..], 4)
    );

    assert!(bytes.read_with::<&[u8]>(&mut 5, Bytes::Len(0)).is_err());

    let mut write = [0; 5];
    assert_eq!(TryWrite::try_write(bytes, &mut write, ()).unwrap(), 4);
    assert_eq!(&write[..4], bytes);

    assert!([0u8; 3].write(&mut 0, bytes).is_err());
}

#[test]
fn test_bytes_pattern() {
    let bytes: &[u8] = b"abcdefghijk";

    assert_eq!(
        TryRead::try_read(bytes, Bytes::Pattern(b"abc")).unwrap(),
        (&b"abc"[..], 3)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::Pattern(b"cde")).unwrap(),
        (&b"abcde"[..], 5)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::Pattern(b"jk")).unwrap(),
        (&b"abcdefghijk"[..], 11)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::PatternUntil(b"abc", 3)).unwrap(),
        (&b"abc"[..], 3)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::PatternUntil(b"abc", 4)).unwrap(),
        (&b"abc"[..], 3)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::PatternUntil(b"cde", 3)).unwrap(),
        (&b"abc"[..], 3)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::PatternUntil(b"cde", 4)).unwrap(),
        (&b"abcd"[..], 4)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::PatternUntil(b"cde", 5)).unwrap(),
        (&b"abcde"[..], 5)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::PatternUntil(b"cde", 6)).unwrap(),
        (&b"abcde"[..], 5)
    );
    assert_eq!(
        TryRead::try_read(bytes, Bytes::PatternUntil(b"xyz", 5)).unwrap(),
        (&b"abcde"[..], 5)
    );
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::Pattern(b"xyz"))
        .is_err());
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::Pattern(b""))
        .is_err());
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::PatternUntil(b"", 3))
        .is_err());
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::PatternUntil(b"abcd", 3))
        .is_err());
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::PatternUntil(b"xyz", 20))
        .is_err());

    let bytes: &[u8] = b"";
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::Pattern(b"xyz"))
        .is_err());
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::PatternUntil(b"abc", 3))
        .is_err());
    assert!(bytes
        .read_with::<&[u8]>(&mut 0, Bytes::PatternUntil(b"abc", 4))
        .is_err());
}

#[test]
fn test_bool() {
    let bytes = [0x00, 0x01, 0x80, 0xff];
    assert_eq!(bytes.read::<bool>(&mut 0).unwrap(), false);
    assert_eq!(bytes.read::<bool>(&mut 1).unwrap(), true);
    assert_eq!(bytes.read::<bool>(&mut 2).unwrap(), true);
    assert_eq!(bytes.read::<bool>(&mut 3).unwrap(), true);

    let mut bytes = [0u8; 2];
    bytes.write(&mut 0, false).unwrap();
    bytes.write(&mut 1, true).unwrap();
    assert!(bytes[0] == 0);
    assert!(bytes[1] != 0);
}

#[test]
fn test_iter() {
    let bytes: &[u8] = b"hello\0world\0dead\0beef\0more";
    let mut offset = 0;
    {
        let mut iter = bytes.read_iter(&mut offset, Str::Delimiter(NULL));
        assert_eq!(iter.next(), Some("hello"));
        assert_eq!(iter.next(), Some("world"));
        assert_eq!(iter.next(), Some("dead"));
        assert_eq!(iter.next(), Some("beef"));
        assert_eq!(iter.next(), None);
    }
    assert_eq!(offset, 22);
}

macro_rules! test_num {
    ($test_name: tt, $ty: ty, $byteorder_read_fn: tt, $byteorder_write_fn: tt) => {
        quickcheck! {
            fn $test_name (num: $ty) -> () {
                let mut bytes = [0u8; 8];
                bytes.write_with(&mut 0, num, LE).unwrap();
                let result = LittleEndian::$byteorder_read_fn(&bytes);
                assert_eq!(result, num);

                let mut bytes = [0u8; 8];
                LittleEndian::$byteorder_write_fn(&mut bytes, num);
                let result: $ty = bytes.read_with(&mut 0, LE).unwrap();
                assert_eq!(result, num);

                let mut bytes = [0u8; 8];
                bytes.write_with(&mut 0, num, BE).unwrap();
                let result = BigEndian::$byteorder_read_fn(&bytes);
                assert_eq!(result, num);

                let mut bytes = [0u8; 8];
                BigEndian::$byteorder_write_fn(&mut bytes, num);
                let result: $ty = bytes.read_with(&mut 0, BE).unwrap();
                assert_eq!(result, num);
            }
        }
    };
}

test_num!(test_u16, u16, read_u16, write_u16);
test_num!(test_u32, u32, read_u32, write_u32);
test_num!(test_u64, u64, read_u64, write_u64);
test_num!(test_i16, i16, read_i16, write_i16);
test_num!(test_i32, i32, read_i32, write_i32);
test_num!(test_i64, i64, read_i64, write_i64);
test_num!(test_f32, f32, read_f32, write_f32);
test_num!(test_f64, f64, read_f64, write_f64);

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

#[test]
fn test_api() {
    let bytes = [0, 5, b"H"[0], b"E"[0], b"L"[0], b"L"[0], b"O"[0], 0];

    let header: Header = bytes.read_with(&mut 0, BE).unwrap();
    assert_eq!(header.name, "HELLO");
    assert_eq!(header.enabled, false);

    let mut write = [0u8; 8];
    write.write_with(&mut 0, header, BE).unwrap();
    assert_eq!(write, bytes);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Empty;

impl<'a> TryRead<'a, ()> for Empty {
    fn try_read(bytes: &'a [u8], _ctx: ()) -> Result<(Self, usize)> {
        Ok((Self, 0))
    }
}

impl TryWrite<()> for Empty {
    fn try_write(self, bytes: &mut [u8], _ctx: ()) -> Result<usize> {
        Ok(0)
    }
}

#[test]
fn test_empty() {
    let empty_bytes: [u8; 0] = [];
    let mut offset = 0;
    let empty: Empty = empty_bytes.read(&mut offset).unwrap();
    assert_eq!(empty, Empty);
    assert_eq!(offset, 0);

    let zero_bytes = [0; 8];
    let mut offset = 0;
    let empty: Empty = zero_bytes.read(&mut offset).unwrap();
    assert_eq!(empty, Empty);
    assert_eq!(offset, 0);

    let mut write_empty_bytes: [u8; 0] = [];
    let mut offset = 0;
    write_empty_bytes.write(&mut offset, Empty).unwrap();
    assert_eq!(write_empty_bytes, empty_bytes);
    assert_eq!(offset, 0);

    let mut write_zero_bytes = [0u8; 4];
    let mut offset = 0;
    write_zero_bytes.write(&mut offset, Empty).unwrap();
    assert_eq!(write_zero_bytes, [0u8; 4]);
    assert_eq!(offset, 0);
}
