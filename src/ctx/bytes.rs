use {TryFromCtx, TryIntoCtx};

impl<'a> TryFromCtx<'a, usize> for &'a [u8] {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], len: usize) -> Result<(Self, usize), ()> {
        if len > scroll.len() {
            return Err(());
        };

        Ok((&scroll[..len], len))
    }
}

impl<'a> TryIntoCtx<()> for &'a [u8] {
    #[inline]
    fn try_into_ctx(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        if self.len() > scroll.len() {
            return Err(());
        };

        scroll[..self.len()].clone_from_slice(self);

        Ok(self.len())
    }
}