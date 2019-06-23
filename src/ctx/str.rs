use core::str;
use {check_len, Error, Result, TryRead, TryWrite};

/// Context for &str to determine where a &str ends.
///
/// Pattern will **not** be included in the result
///
/// Default to `NULL` delimiter.
///
/// # Example
///
/// ```
/// use byte::*;
/// use byte::ctx::*;
///
/// let bytes: &[u8] = b"hello, world!\0";
///
/// let str: &str = bytes.read(&mut 0).unwrap();
/// assert_eq!(str, "hello, world!");
///
/// let str: &str = bytes.read_with(&mut 0, Str::Len(5)).unwrap();
/// assert_eq!(str, "hello");
///
/// let str: &str = bytes.read_with(&mut 0, Str::Delimiter(b"!"[0])).unwrap();
/// assert_eq!(str, "hello, world");
///
/// let str: &str = bytes.read_with(&mut 0, Str::DelimiterUntil(NULL, 5)).unwrap();
/// assert_eq!(str, "hello");
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Str {
    /// Take fix-length bytes as str
    Len(usize),
    /// Take bytes until reaching a delimiter
    Delimiter(u8),
    /// Take bytes until either delimiter or length reached
    DelimiterUntil(u8, usize),
}

impl Default for Str {
    #[inline]
    fn default() -> Self {
        Str::Delimiter(NULL)
    }
}

/// Null string delimiter
pub const NULL: u8 = 0;
/// Space string delimiter
pub const SPACE: u8 = 0x20;
/// Return string delimiter
pub const RET: u8 = 0x0a;
/// Tab string delimiter
pub const TAB: u8 = 0x09;

impl<'a> TryRead<'a, Str> for &'a str {
    #[inline]
    fn try_read(bytes: &'a [u8], ctx: Str) -> Result<(Self, usize)> {
        let (bytes, size) = match ctx {
            Str::Len(len) => {
                let len = check_len(bytes, len)?;
                (&bytes[..len], len)
            }
            Str::Delimiter(delimiter) => {
                let position = bytes
                    .iter()
                    .position(|c| *c == delimiter)
                    .ok_or(Error::Incomplete)?;
                (&bytes[..position], position + 1)
            }
            Str::DelimiterUntil(delimiter, len) => {
                let position = bytes.iter().take(len).position(|c| *c == delimiter);
                match position {
                    Some(position) => (&bytes[..position], position + 1),
                    None => {
                        let len = check_len(bytes, len)?;
                        (&bytes[..len], len)
                    }
                }
            }
        };

        match str::from_utf8(bytes) {
            Ok(str) => Ok((str, size)),
            Err(_) => Err(Error::BadInput { err: "UTF8 Error" }),
        }
    }
}

impl<'a> TryWrite for &'a str {
    #[inline]
    fn try_write(self, bytes: &mut [u8], _ctx: ()) -> Result<usize> {
        let str_bytes = self.as_bytes();

        check_len(bytes, str_bytes.len())?;

        bytes[..str_bytes.len()].clone_from_slice(str_bytes);

        Ok(str_bytes.len())
    }
}
