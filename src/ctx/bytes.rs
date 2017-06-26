use {TryRead, TryWrite, Error, Result, check_len};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ByteCtx {
    Length(usize),
    Pattern(&'static [u8]),
    UntilPattern(&'static [u8]),
}

impl<'a> TryRead<'a, ByteCtx> for &'a [u8] {
    #[inline]
    fn try_read(bytes: &'a [u8], ctx: ByteCtx) -> Result<(Self, usize)> {
        let len = match ctx {
            ByteCtx::Length(len) => check_len(bytes, len),
            ByteCtx::Pattern(pattern) => {
                check_len(bytes, pattern.len())?;
                if bytes.starts_with(pattern) {
                    Ok(pattern.len())
                } else {
                    Err(Error::BadInput("Pattern Mismatch"))
                }
            }
            ByteCtx::UntilPattern(pattern) => {
                check_len(bytes, pattern.len())?;
                (0..bytes.len() - pattern.len() + 1)
                    .map(|n| bytes[n..].starts_with(pattern))
                    .position(|p| p)
                    .map(|len| len + pattern.len())
                    .ok_or(Error::Incomplete)
            }
        }?;

        Ok((&bytes[..len], len))
    }
}

impl<'a> TryWrite for &'a [u8] {
    #[inline]
    fn try_write(self, bytes: &mut [u8], _ctx: ()) -> Result<usize> {
        check_len(bytes, self.len())?;

        bytes[..self.len()].clone_from_slice(self);

        Ok(self.len())
    }
}