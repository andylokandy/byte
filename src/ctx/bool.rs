use {TryRead, TryWrite, Result, check_len};

impl<'a> TryRead<'a> for bool {
    #[inline]
    fn try_read(scroll: &'a [u8], _ctx: ()) -> Result<(Self, usize)> {
        check_len(scroll, 1)?;

        Ok((scroll[0] != 0, 1))
    }
}

impl TryWrite for bool {
    #[inline]
    fn try_write(self, scroll: &mut [u8], _ctx: ()) -> Result<usize> {
        check_len(scroll, 1)?;

        scroll[0] = if self { u8::max_value() } else { 0 };

        Ok(1)
    }
}