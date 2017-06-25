use {TryRead, TryWrite, Error, Result, check_len};
use std::str;

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
    fn try_read(scroll: &'a [u8], ctx: StrCtx) -> Result<(Self, usize)> {
        let (scroll, size) = match ctx {
            StrCtx::Length(len) => {
                let len = check_len(scroll, len)?;
                (&scroll[..len], len)
            }
            StrCtx::Delimiter(delimiter) => {
                let position = scroll
                    .iter()
                    .position(|c| *c == delimiter)
                    .ok_or(Error::Incomplete)?;
                (&scroll[..position], position + 1)
            }
            StrCtx::DelimiterUntil(delimiter, len) => {
                let position = scroll.iter().take(len).position(|c| *c == delimiter);
                match position {
                    Some(position) => (&scroll[..position], position + 1),
                    None => {
                        let len = check_len(scroll, len)?;
                        (&scroll[..len], len)
                    }
                }
            }
        };

        str::from_utf8(scroll)
            .map(|str| (str, size))
            .map_err(|_| Error::BadInput("UTF8 Error"))
    }
}

impl<'a> TryWrite for &'a str {
    #[inline]
    fn try_write(self, scroll: &mut [u8], _ctx: ()) -> Result<usize> {
        let bytes = self.as_bytes();

        check_len(scroll, bytes.len())?;

        scroll[..bytes.len()].clone_from_slice(bytes);

        Ok(bytes.len())
    }
}