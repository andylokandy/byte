extern crate scroll;

use scroll::*;
use scroll::ctx::str::StrCtx;

#[test]
fn test_pread_str() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";
    let s: &str = bytes.pread_with(0, StrCtx::Delimiter(0)).unwrap();
    assert_eq!(s, "hello, world!");

    let bytes: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let s: &str = bytes.pread_with(0, StrCtx::Length(15)).unwrap();
    assert_eq!(s, "abcdefghijklmno");

    assert!(bytes
                .pread_with::<&str, _>(0, StrCtx::Length(27))
                .is_err());

    assert!(bytes
                .pread_with::<&str, _>(27, StrCtx::Length(0))
                .is_err());

    assert!(bytes
                .pread_with::<&str, _>(26, StrCtx::Length(1))
                .is_err());
}

#[test]
fn test_gread_str() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";
    let mut offset = 0;
    let s: &str = bytes
        .gread_with(&mut offset, StrCtx::Delimiter(0))
        .unwrap();
    assert_eq!(s, "hello, world!");
    assert_eq!(offset, 13);
}

#[test]
fn test_pwrite_str() {
    let bytes: &mut [u8] = &mut [0; 20];
    bytes.pwrite(0, "hello world!").unwrap();
    assert_eq!(&bytes[..12], b"hello world!" as &[u8]);

    let bytes: &mut [u8] = &mut [0; 10];
    assert!(bytes.pwrite(0, "hello world!").is_err());
}

#[test]
fn test_gwrite_str() {
    let bytes: &mut [u8] = &mut [0; 20];
    let mut offset = 0;
    bytes.gwrite(&mut offset, "hello world!").unwrap();
    assert_eq!(offset, 12);
    assert_eq!(&bytes[..offset], b"hello world!" as &[u8]);
}

#[test]
fn test_bytes() {
    let bytes = [0xde, 0xad, 0xbe, 0xef];
    let (read, len): (&[u8], usize) = TryFromCtx::try_from_ctx(&bytes, 4).unwrap();
    assert_eq!(read, &[0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(len, 4);

    assert!(bytes.pread::<&[u8], _>(5).is_err());

    let mut bytes = [0; 5];
    let len = TryIntoCtx::try_into_ctx(read, &mut bytes, ()).unwrap();
    assert_eq!(bytes, [0xde, 0xad, 0xbe, 0xef, 0x00]);
    assert_eq!(len, 4);

    assert!([0u8; 3].pwrite(0, read).is_err());
}