use prelude::*;
use std::str;

#[derive(Debug)]
pub enum StrCtx {
    Delimiter(u8),
    Length(usize),
}

impl<'a> TryFromCtx<'a, StrCtx> for &'a str {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], ctx: StrCtx) -> Result<(Self, usize), ()> {
        let len = match ctx {
            StrCtx::Delimiter(delimiter) => scroll.iter().take_while(|c| **c != delimiter).count(),
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
    fn try_into_ctx(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        let bytes = self.as_bytes();

        if bytes.len() > scroll.len() {
            return Err(());
        };

        scroll[..bytes.len()].clone_from_slice(bytes);

        Ok(bytes.len())
    }
}
