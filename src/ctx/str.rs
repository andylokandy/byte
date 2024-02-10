use crate::{check_len, Error, Result, TryRead, TryWrite};
use core::str;

use super::{Delimiter, DelimiterUntil, Len};

/// Null string delimiter
pub const NULL: u8 = 0;
/// Space string delimiter
pub const SPACE: u8 = 0x20;
/// Return string delimiter
pub const RET: u8 = 0x0a;
/// Tab string delimiter
pub const TAB: u8 = 0x09;

impl<'a> TryRead<'a, Len> for &'a str {
    #[inline]
    fn try_read(bytes: &'a [u8], Len(len): Len) -> Result<(Self, usize)> {
        let (bytes, size) = {
            let len = check_len(bytes, len)?;
            (&bytes[..len], len)
        };

        match str::from_utf8(bytes) {
            Ok(str) => Ok((str, size)),
            Err(_) => Err(Error::BadInput { err: "UTF8 Error" }),
        }
    }
}

impl<'a> TryRead<'a, Delimiter> for &'a str {
    #[inline]
    fn try_read(bytes: &'a [u8], Delimiter(delimiter): Delimiter) -> Result<(Self, usize)> {
        let (bytes, size) = {
            let position = bytes
                .iter()
                .position(|c| *c == delimiter)
                .ok_or(Error::Incomplete)?;
            (&bytes[..position], position + 1)
        };

        match str::from_utf8(bytes) {
            Ok(str) => Ok((str, size)),
            Err(_) => Err(Error::BadInput { err: "UTF8 Error" }),
        }
    }
}

impl<'a> TryRead<'a, DelimiterUntil> for &'a str {
    #[inline]
    fn try_read(
        bytes: &'a [u8],
        DelimiterUntil(delimiter, len): DelimiterUntil,
    ) -> Result<(Self, usize)> {
        let (bytes, size) = {
            let position = bytes.iter().take(len).position(|c| *c == delimiter);
            match position {
                Some(position) => (&bytes[..position], position + 1),
                None => {
                    let len = check_len(bytes, len)?;
                    (&bytes[..len], len)
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

        bytes[..str_bytes.len()].copy_from_slice(str_bytes);

        Ok(str_bytes.len())
    }
}
