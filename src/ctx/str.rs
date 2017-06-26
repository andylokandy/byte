use {TryRead, TryWrite, Error, Result, check_len};
use core::str;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum StrCtx {
    Delimiter(u8),
    DelimiterUntil(u8, usize),
    Length(usize),
}

pub const NULL: u8 = 0;
pub const SPACE: u8 = 0x20;
pub const RET: u8 = 0x0a;
pub const TAB: u8 = 0x09;

impl<'a> TryRead<'a, StrCtx> for &'a str {
    #[inline]
    fn try_read(bytes: &'a [u8], ctx: StrCtx) -> Result<(Self, usize)> {
        let (bytes, size) = match ctx {
            StrCtx::Length(len) => {
                let len = check_len(bytes, len)?;
                (&bytes[..len], len)
            }
            StrCtx::Delimiter(delimiter) => {
                let position = bytes
                    .iter()
                    .position(|c| *c == delimiter)
                    .ok_or(Error::Incomplete)?;
                (&bytes[..position], position + 1)
            }
            StrCtx::DelimiterUntil(delimiter, len) => {
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

        str::from_utf8(bytes)
            .map(|str| (str, size))
            .map_err(|_| Error::BadInput("UTF8 Error"))
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