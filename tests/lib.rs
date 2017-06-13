extern crate scroll;

use scroll::prelude::*;
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