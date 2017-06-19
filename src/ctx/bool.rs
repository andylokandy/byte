use {TryFromCtx, TryIntoCtx, Result, assert_len};

impl<'a> TryFromCtx<'a, ()> for bool {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], _ctx: ()) -> Result<(Self, usize), ()> {
        assert_len(scroll, 1)?;

        Ok((scroll[0] != 0, 1))
    }
}

impl<'a> TryIntoCtx<()> for bool {
    #[inline]
    fn try_into_ctx(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        assert_len(scroll, 1)?;

        scroll[0] = if self { u8::max_value() } else { 0 };

        Ok(1)
    }
}