use {TryRead, TryWrite, Error, Result, check_len};

#[derive(Debug)]
pub enum ByteCtx {
    Length(usize),
    Pattern(&'static [u8]),
    UntilPattern(&'static [u8]),
}

#[derive(Debug)]
pub struct Mismatch;

impl<'a> TryRead<'a, ByteCtx, Mismatch> for &'a [u8] {
    #[inline]
    fn try_read(scroll: &'a [u8], ctx: ByteCtx) -> Result<(Self, usize), Mismatch> {
        let len = match ctx {
            ByteCtx::Length(len) => check_len(scroll, len),
            ByteCtx::Pattern(pattern) => {
                check_len(scroll, pattern.len())?;
                if scroll.starts_with(pattern) {
                    Ok(pattern.len())
                } else {
                    Err(Error::Other(Mismatch))
                }
            }
            ByteCtx::UntilPattern(pattern) => {
                check_len(scroll, pattern.len())?;
                (0..scroll.len() - pattern.len() + 1)
                    .map(|n| scroll[n..].starts_with(pattern))
                    .position(|p| p)
                    .map(|len| len + pattern.len())
                    .ok_or(Error::Incomplete)
            }
        }?;

        Ok((&scroll[..len], len))
    }
}

impl<'a> TryWrite for &'a [u8] {
    #[inline]
    fn try_write(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        check_len(scroll, self.len())?;

        scroll[..self.len()].clone_from_slice(self);

        Ok(self.len())
    }
}