use {TryFromCtx, TryIntoCtx};

impl<'a> TryFromCtx<'a, ()> for bool {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], _ctx: ()) -> Result<(Self, usize), ()> {
        if 1 > scroll.len() {
            return Err(());
        };

        Ok((scroll[0] != 0, 1))
    }
}

impl<'a> TryIntoCtx<()> for bool {
    #[inline]
    fn try_into_ctx(self, scroll: &mut [u8], _ctx: ()) -> Result<usize, ()> {
        if 1 > scroll.len() {
            return Err(());
        };

        scroll[0] = self as u8;

        Ok(1)
    }
}