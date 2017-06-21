use {TryFromCtx, Error, Result, assert_len};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Padding;

pub enum Ctx {
    UntilByte(u8),
    UntilSlice(&'static [u8]),
}

impl<'a> TryFromCtx<'a, Ctx> for Padding {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], ctx: Ctx) -> Result<(Self, usize), ()> {
        let len = match ctx {
                Ctx::UntilByte(byte) => {
                    scroll
                        .iter()
                        .position(|c| *c == byte)
                        .map(|len| len + 1)
                }
                Ctx::UntilSlice(slice) => {
                    assert_len(scroll, slice.len())?;
                    (0..scroll.len() - slice.len() + 1)
                        .map(|n| scroll[n..].starts_with(slice))
                        .position(|p| p)
                        .map(|len| len + slice.len())
                }
            }
            .ok_or(Error::Incomplete)?;

        Ok((Padding, len))
    }
}