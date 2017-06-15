pub mod ctx;
pub use ctx::num::{LE, BE};

pub trait TryFromCtx<'a, Ctx>
    where Self: Sized
{
    fn try_from_ctx(scroll: &'a [u8], ctx: Ctx) -> Result<(Self, usize), ()>;
}

pub trait TryIntoCtx<Ctx> {
    fn try_into_ctx(self, scroll: &mut [u8], ctx: Ctx) -> Result<usize, ()>;
}

pub trait Pread<'a, Ctx> {
    fn pread<T>(&'a self, offset: usize) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Ctx: Default
    {
        self.pread_with(offset, Default::default())
    }

    fn pread_with<T>(&'a self, offset: usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>;

    fn gread<T>(&'a self, offset: &mut usize) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Ctx: Default
    {
        self.gread_with(offset, Default::default())
    }

    fn gread_with<T>(&'a self, offset: &mut usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>;
}

pub trait Pwrite<Ctx>
    where Self: Sized
{
    fn pwrite<T>(self, offset: usize, t: T) -> Result<(), ()>
        where T: TryIntoCtx<Ctx>,
              Ctx: Default
    {
        self.pwrite_with(offset, t, Default::default())
    }

    fn pwrite_with<T>(self, offset: usize, t: T, ctx: Ctx) -> Result<(), ()>
        where T: TryIntoCtx<Ctx>;

    fn gwrite<T>(self, offset: &mut usize, t: T) -> Result<(), ()>
        where T: TryIntoCtx<Ctx>,
              Ctx: Default
    {
        self.gwrite_with(offset, t, Default::default())
    }

    fn gwrite_with<T>(self, offset: &mut usize, t: T, ctx: Ctx) -> Result<(), ()>
        where T: TryIntoCtx<Ctx>;
}


impl<'a, Ctx, Slice> Pread<'a, Ctx> for Slice
    where Slice: AsRef<[u8]>
{
    #[inline]
    fn pread_with<T>(&'a self, offset: usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>
    {
        let slice = self.as_ref();

        if offset >= slice.len() {
            return Err(());
        };

        TryFromCtx::try_from_ctx(&slice[offset..], ctx).map(|(t, _)| t)
    }

    #[inline]
    fn gread_with<T>(&'a self, offset: &mut usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>
    {
        let slice = self.as_ref();

        if *offset >= slice.len() {
            return Err(());
        };

        TryFromCtx::try_from_ctx(&slice[*offset..], ctx).map(|(t, size)| {
                                                                 *offset += size;
                                                                 t
                                                             })
    }
}

impl<Ctx, Slice> Pwrite<Ctx> for Slice
    where Slice: AsMut<[u8]>
{
    fn pwrite_with<T>(mut self, offset: usize, t: T, ctx: Ctx) -> Result<(), ()>
        where T: TryIntoCtx<Ctx>
    {
        let mut slice = self.as_mut();

        if offset >= slice.len() {
            return Err(());
        };

        TryIntoCtx::try_into_ctx(t, &mut slice[offset..], ctx).map(|_| ())
    }

    fn gwrite_with<T>(mut self, offset: &mut usize, t: T, ctx: Ctx) -> Result<(), ()>
        where T: TryIntoCtx<Ctx>
    {
        let mut slice = self.as_mut();

        if *offset >= slice.len() {
            return Err(());
        };

        TryIntoCtx::try_into_ctx(t, &mut slice[*offset..], ctx).map(|size| {
                                                                        *offset += size;
                                                                        ()
                                                                    })
    }
}