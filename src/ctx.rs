//! Context for primitives

mod bool;
mod bytes;
mod num;
mod str;

pub use self::num::*;
pub use self::str::*;

/// Context that provides a fixed length for a slice.
///
/// # Example
///
/// ```
/// use byte::*;
/// use byte::ctx::*;
///
/// let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef, 0x00, 0xff];
///
/// let sub: &[u8] = bytes.read_with(&mut 0, Len(2)).unwrap();
/// assert_eq!(sub, &[0xde, 0xad]);
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Len(pub usize);

/// Context that provides a pattern that determines where the slice ends.
///
/// # Example
///
/// ```
/// use byte::*;
/// use byte::ctx::*;
///
/// let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef, 0x00, 0xff];
///
/// static PATTERN: &'static [u8; 2] = &[0x00, 0xff];
///
/// let sub: &[u8] = bytes.read_with(&mut 0, Pattern(PATTERN)).unwrap();
/// assert_eq!(sub, &[0xde, 0xad, 0xbe, 0xef, 0x00, 0xff]);
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Pattern(pub &'static [u8]);

/// Context that provides a pattern that determines where the slice ends and a maximum length.
///
/// # Example
///
/// ```
/// use byte::*;
/// use byte::ctx::*;
///
/// let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef, 0x00, 0xff];
///
/// static PATTERN: &'static [u8; 2] = &[0x00, 0xff];
///
/// let sub: &[u8] = bytes.read_with(&mut 0, PatternUntil(PATTERN, 4)).unwrap();
/// assert_eq!(sub, &[0xde, 0xad, 0xbe, 0xef]);
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PatternUntil(pub &'static [u8], pub usize);

/// Context that provides a delimiter that determines where the slice ends.
///
/// # Example
///
/// ```
/// use byte::*;
/// use byte::ctx::*;
///
/// let bytes: &[u8] = b"hello, world!\0";

/// let str: &str = bytes.read_with(&mut 0, Delimiter(b"!"[0])).unwrap();
/// assert_eq!(str, "hello, world");
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Delimiter(pub u8);

/// Context that provides a delimiter that determines where the slice ends and a maximum length.
///
/// # Example
///
/// ```
/// use byte::*;
/// use byte::ctx::*;
///
/// let bytes: &[u8] = b"hello, world!\0";

/// let str: &str = bytes.read_with(&mut 0, DelimiterUntil(NULL, 5)).unwrap();
/// assert_eq!(str, "hello");
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct DelimiterUntil(pub u8, pub usize);
