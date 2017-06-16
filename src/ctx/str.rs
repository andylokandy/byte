use {TryFromCtx, TryIntoCtx};
use std::str;

#[derive(Debug)]
pub enum StrCtx {
    Delimiter(u8),
    DelimiterUntil(u8, usize),
    Length(usize),
}

pub const NULL: StrCtx = StrCtx::Delimiter(0);
pub const SPACE: StrCtx = StrCtx::Delimiter(0x20);
pub const RET: StrCtx = StrCtx::Delimiter(0x0a);
pub const TAB: StrCtx = StrCtx::Delimiter(0x09);

impl<'a> TryFromCtx<'a, StrCtx> for &'a str {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], ctx: StrCtx) -> Result<(Self, usize), ()> {
        let len = match ctx {
            StrCtx::Delimiter(delimiter) => scroll.iter().take_while(|c| **c != delimiter).count(),
            StrCtx::DelimiterUntil(delimiter, len) => {
                scroll
                    .iter()
                    .take_while(|c| **c != delimiter)
                    .take(len)
                    .count()
            }
            StrCtx::Length(len) => len,
        };

        if len > scroll.len() {
            return Err(());
        };

        str::from_utf8(&scroll[..len])
            .map(|s| (s, len))
            .map_err(|_| ())
    }
}

impl<'a> TryIntoCtx<()> for &'a str {
    #[inline]
    fn try_into_ctx(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        let bytes = self.as_bytes();

        if bytes.len() > scroll.len() {
            return Err(());
        };

        scroll[..bytes.len()].clone_from_slice(bytes);

        Ok(bytes.len())
    }
}