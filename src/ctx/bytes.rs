use {TryFromCtx, TryIntoCtx, Result, assert_len};

impl<'a> TryFromCtx<'a, usize> for &'a [u8] {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], len: usize) -> Result<(Self, usize), ()> {
        assert_len(scroll, len)?;

        Ok((&scroll[..len], len))
    }
}

impl<'a> TryIntoCtx<()> for &'a [u8] {
    #[inline]
    fn try_into_ctx(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        assert_len(scroll, self.len())?;

        scroll[..self.len()].clone_from_slice(self);

        Ok(self.len())
    }
}