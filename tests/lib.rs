#[macro_use]
extern crate quickcheck;
extern crate byteorder;
extern crate byte;

use byteorder::*;
use byte::*;
use byte::ctx::str::*;
use byte::ctx::bytes::*;

#[test]
fn test_str_read() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";
    assert_eq!(TryRead::try_read(bytes, StrCtx::Delimiter(NULL)).unwrap(),
               ("hello, world!", 14));
    assert!(bytes
                .read_with::<&str>(&mut 0, StrCtx::Delimiter(RET))
                .is_err());

    let mut offset = 0;
    assert_eq!(bytes
                   .read_with::<&str>(&mut offset, StrCtx::Delimiter(NULL))
                   .unwrap(),
               "hello, world!");
    assert_eq!(offset, 14);

    let bytes: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    assert_eq!(TryRead::try_read(bytes, StrCtx::Length(15)).unwrap(),
               ("abcdefghijklmno", 15));
    assert_eq!(TryRead::try_read(bytes, StrCtx::Length(26)).unwrap(),
               ("abcdefghijklmnopqrstuvwxyz", 26));

    assert!(bytes
                .read_with::<&str>(&mut 0, StrCtx::Length(26))
                .is_ok());
    assert!(bytes
                .read_with::<&str>(&mut 0, StrCtx::Length(27))
                .is_err());
    assert!(bytes
                .read_with::<&str>(&mut 27, StrCtx::Length(0))
                .is_err());
    assert!(bytes
                .read_with::<&str>(&mut 26, StrCtx::Length(1))
                .is_err());
}

#[test]
fn test_str_delimitor_until() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";

    assert_eq!(TryRead::try_read(bytes, StrCtx::DelimiterUntil(NULL, 20)).unwrap(),
               ("hello, world!", 14));
    assert_eq!(TryRead::try_read(bytes, StrCtx::DelimiterUntil(NULL, 14)).unwrap(),
               ("hello, world!", 14));
    assert_eq!(TryRead::try_read(bytes, StrCtx::DelimiterUntil(NULL, 10)).unwrap(),
               ("hello, wor", 10));

    let bytes: &[u8] = b"hello, world!";
    assert!(bytes
                .read_with::<&str>(&mut 0, StrCtx::DelimiterUntil(NULL, 20))
                .is_err());
    assert!(bytes
                .read_with::<&str>(&mut 0, StrCtx::Delimiter(NULL))
                .is_err());
}

#[test]
fn test_str_write() {
    let mut bytes = [0; 20];
    let mut offset = 0;
    bytes.write(&mut offset, "hello world!").unwrap();
    assert_eq!(offset, 12);
    assert_eq!(&bytes[..offset], b"hello world!" as &[u8]);

    let mut bytes = &mut [0; 10];
    assert!(bytes.write(&mut 0, "hello world!").is_err());
}

#[test]
fn test_bytes() {
    let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef];
    assert_eq!(TryRead::try_read(&bytes, ByteCtx::Length(4)).unwrap(),
               (&bytes[..], 4));

    assert!(bytes
                .read_with::<&[u8]>(&mut 5, ByteCtx::Length(0))
                .is_err());

    let mut write = [0; 5];
    assert_eq!(TryWrite::try_write(bytes, &mut write, ()).unwrap(), 4);
    assert_eq!(&write[..4], bytes);

    assert!([0u8; 3].write(&mut 0, bytes).is_err());
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
fn test_bytes_pattern() {
    let bytes: &[u8] = b"abcde\0fghijk";

    assert_eq!(TryRead::try_read(bytes, ByteCtx::Pattern(b"abc")).unwrap(),
               (&b"abc"[..], 3));

    assert_eq!(TryRead::try_read(bytes, ByteCtx::UntilPattern(b"fg")).unwrap(),
               (&b"abcde\0fg"[..], 8));

    assert_eq!(TryRead::try_read(bytes, ByteCtx::UntilPattern(b"jk")).unwrap(),
               (&b"abcde\0fghijk"[..], 12));

    assert!(bytes
                .read_with::<&[u8]>(&mut 0, ByteCtx::Pattern(b"bcd"))
                .is_err());
    assert!(bytes
                .read_with::<&[u8]>(&mut 0, ByteCtx::UntilPattern(b"xyz"))
                .is_err());
    assert!(bytes
                .read_with::<&[u8]>(&mut 0, ByteCtx::UntilPattern(b"jkl"))
                .is_err());
}

#[test]
fn test_iter() {
    let bytes: &[u8] = b"hello\0world\0dead\0beef\0more";
    let mut offset = 0;
    {
        let mut iter = bytes.read_iter(&mut offset, StrCtx::Delimiter(NULL));
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
    }
}

test_num!(test_u16, u16, read_u16, write_u16);
test_num!(test_u32, u32, read_u32, write_u32);
test_num!(test_u64, u64, read_u64, write_u64);
test_num!(test_i16, i16, read_i16, write_i16);
test_num!(test_i32, i32, read_i32, write_i32);
test_num!(test_i64, i64, read_i64, write_i64);
test_num!(test_f32, f32, read_f32, write_f32);
test_num!(test_f64, f64, read_f64, write_f64);