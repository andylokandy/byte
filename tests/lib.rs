extern crate scroll;

use scroll::*;
use scroll::ctx::str::*;

#[test]
fn test_pread_str() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";
    let s: &str = bytes.pread_with(0, StrCtx::Delimiter(NULL)).unwrap();
    assert_eq!(s, "hello, world!");

    let bytes: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let s: &str = bytes.pread_with(0, StrCtx::Length(15)).unwrap();
    assert_eq!(s, "abcdefghijklmno");

    assert!(bytes.pread_with::<&str>(0, StrCtx::Length(26)).is_ok());
    assert!(bytes.pread_with::<&str>(0, StrCtx::Length(27)).is_err());
    assert!(bytes.pread_with::<&str>(27, StrCtx::Length(0)).is_err());
    assert!(bytes.pread_with::<&str>(26, StrCtx::Length(1)).is_err());
}

#[test]
fn test_gread_str() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";
    let mut offset = 0;
    let s: &str = bytes
        .gread_with(&mut offset, StrCtx::Delimiter(NULL))
        .unwrap();
    assert_eq!(s, "hello, world!");
    assert_eq!(offset, 13);
}

#[test]
fn test_str_delimitor_until() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";

    let mut offset = 0;
    let s: &str = bytes
        .gread_with(&mut offset, StrCtx::DelimiterUntil(NULL, 20))
        .unwrap();
    assert_eq!(s, "hello, world!");
    assert_eq!(offset, 13);

    let mut offset = 0;
    let s: &str = bytes
        .gread_with(&mut offset, StrCtx::DelimiterUntil(NULL, 10))
        .unwrap();
    assert_eq!(s, "hello, wor");
    assert_eq!(offset, 10);

    let bytes: &[u8] = b"hello, world!";
    assert!(bytes
                .pread_with::<&str>(0, StrCtx::DelimiterUntil(NULL, 20))
                .is_err());
}

#[test]
fn test_pwrite_str() {
    let mut bytes = [0; 20];
    bytes.pwrite(0, "hello world!").unwrap();
    assert_eq!(&bytes[..12], b"hello world!" as &[u8]);

    let mut bytes = &mut [0; 10];
    assert!(bytes.pwrite(0, "hello world!").is_err());
}

#[test]
fn test_gwrite_str() {
    let mut bytes = [0; 20];
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

    assert!(bytes.pread::<&[u8]>(5).is_err());

    let mut write = [0; 5];
    let mut offset = 0;
    write.gwrite_with(&mut offset, read, ()).unwrap();
    // TryIntoCtx::try_into_ctx(read, &mut write[offset..], ());
    // assert_eq!(write, [0xde, 0xad, 0xbe, 0xef, 0x00]);
    // assert_eq!(offset, 4);

    assert!([0u8; 3].pwrite(0, read).is_err());
}

#[test]
fn test_bool() {
    let bytes = [0x00, 0x01, 0x80, 0xff];
    assert_eq!(bytes.pread::<bool>(0).unwrap(), false);
    assert_eq!(bytes.pread::<bool>(1).unwrap(), true);
    assert_eq!(bytes.pread::<bool>(2).unwrap(), true);
    assert_eq!(bytes.pread::<bool>(3).unwrap(), true);

    let mut bytes = [0u8; 2];
    bytes.pwrite(0, false).unwrap();
    bytes.pwrite(1, true).unwrap();
    assert!(bytes[0] == 0);
    assert!(bytes[1] != 0);
}