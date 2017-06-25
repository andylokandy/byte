pub mod ctx;
pub use ctx::num::{LE, BE};
use std::marker::PhantomData;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadOffset(usize),
    BadInput(&'static str),
    Incomplete,
}

#[inline]
pub fn check_len(scroll: &[u8], len: usize) -> Result<usize> {
    if scroll.len() < len {
        Err(Error::Incomplete)
    } else {
        Ok(len)
    }
}

pub trait TryRead<'a, Ctx = ()>
    where Self: Sized
{
    fn try_read(scroll: &'a [u8], ctx: Ctx) -> Result<(Self, usize)>;
}

pub trait TryWrite<Ctx = ()> {
    fn try_write(self, scroll: &mut [u8], ctx: Ctx) -> Result<usize>;
}

pub trait Read<'a, Ctx> {
    fn read<T>(&'a self, offset: &mut usize) -> Result<T>
        where T: TryRead<'a, Ctx>,
              Ctx: Default
    {
        self.read_with(offset, Default::default())
    }

    fn read_with<T>(&'a self, offset: &mut usize, ctx: Ctx) -> Result<T> where T: TryRead<'a, Ctx>;

    fn read_iter<'i, T>(&'a self, offset: &'i mut usize, ctx: Ctx) -> Iter<'a, 'i, T, Ctx>
        where T: TryRead<'a, Ctx>,
              Ctx: Clone;
}

pub trait Write<Ctx>
    where Self: Sized
{
    fn write<T>(&mut self, offset: &mut usize, t: T) -> Result<()>
        where T: TryWrite<Ctx>,
              Ctx: Default
    {
        self.write_with(offset, t, Default::default())
    }

    fn write_with<T>(&mut self, offset: &mut usize, t: T, ctx: Ctx) -> Result<()>
        where T: TryWrite<Ctx>;
}


impl<'a, Ctx, Slice> Read<'a, Ctx> for Slice
    where Slice: AsRef<[u8]>
{
    #[inline]
    fn read_with<T>(&'a self, offset: &mut usize, ctx: Ctx) -> Result<T>
        where T: TryRead<'a, Ctx>
    {
        let slice = self.as_ref();

        if *offset >= slice.len() {
            return Err(Error::BadOffset(*offset));
        };

        TryRead::try_read(&slice[*offset..], ctx).map(|(t, size)| {
                                                          *offset += size;
                                                          t
                                                      })
    }

    fn read_iter<'i, T>(&'a self, offset: &'i mut usize, ctx: Ctx) -> Iter<'a, 'i, T, Ctx>
        where T: TryRead<'a, Ctx>,
              Ctx: Clone
    {
        Iter {
            scroll: self.as_ref(),
            offset: offset,
            ctx: ctx,
            phantom: PhantomData,
        }
    }
}

impl<Ctx, Slice> Write<Ctx> for Slice
    where Slice: AsMut<[u8]>
{
    fn write_with<T>(&mut self, offset: &mut usize, t: T, ctx: Ctx) -> Result<()>
        where T: TryWrite<Ctx>
    {
        let mut slice = self.as_mut();

        if *offset >= slice.len() {
            return Err(Error::BadOffset(*offset));
        };

        TryWrite::try_write(t, &mut slice[*offset..], ctx).map(|size| {
                                                                   *offset += size;
                                                                   ()
                                                               })
    }
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Iter<'a, 'i, T, Ctx>
    where T: TryRead<'a, Ctx>,
          Ctx: Clone
{
    scroll: &'a [u8],
    offset: &'i mut usize,
    ctx: Ctx,
    phantom: PhantomData<T>,
}

impl<'a, 'i, T, Ctx> Iterator for Iter<'a, 'i, T, Ctx>
    where T: TryRead<'a, Ctx>,
          Ctx: Clone
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        TryRead::try_read(&self.scroll[*self.offset..], self.ctx.clone())
            .ok()
            .map(|(t, size)| {
                     *self.offset += size;
                     t
                 })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}