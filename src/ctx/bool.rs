use {TryRead, TryWrite, Result, check_len};

impl<'a> TryRead<'a> for bool {
    #[inline]
    fn try_read(bytes: &'a [u8], _ctx: ()) -> Result<(Self, usize)> {
        check_len(bytes, 1)?;

        Ok((bytes[0] != 0, 1))
    }
}

impl TryWrite for bool {
    #[inline]
    fn try_write(self, bytes: &mut [u8], _ctx: ()) -> Result<usize> {
        check_len(bytes, 1)?;

        bytes[0] = if self { u8::max_value() } else { 0 };

        Ok(1)
    }
}