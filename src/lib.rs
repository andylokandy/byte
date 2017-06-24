pub mod ctx;
pub use ctx::num::{LE, BE};

type Result<T, E> = std::result::Result<T, Error<E>>;

#[derive(Debug)]
pub enum Error<E> {
    BadOffset(usize),
    Incomplete,
    Other(E),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::Other(error)
    }
}

#[inline]
pub fn check_len<E>(scroll: &[u8], len: usize) -> Result<usize, E> {
    if scroll.len() < len {
        Err(Error::Incomplete)
    } else {
        Ok(len)
    }
}

pub trait TryRead<'a, Ctx = (), E = ()>
    where Self: Sized
{
    fn try_read(scroll: &'a [u8], ctx: Ctx) -> Result<(Self, usize), E>;
}

pub trait TryWrite<Ctx = (), E = ()> {
    fn try_write(self, scroll: &mut [u8], ctx: Ctx) -> Result<usize, E>;
}

pub trait Pread<'a, Ctx, E> {
    fn pread<T>(&'a self, offset: usize) -> Result<T, E>
        where T: TryRead<'a, Ctx, E>,
              Ctx: Default
    {
        self.pread_with(offset, Default::default())
    }

    fn pread_with<T>(&'a self, offset: usize, ctx: Ctx) -> Result<T, E>
        where T: TryRead<'a, Ctx, E>;

    fn gread<T>(&'a self, offset: &mut usize) -> Result<T, E>
        where T: TryRead<'a, Ctx, E>,
              Ctx: Default
    {
        self.gread_with(offset, Default::default())
    }

    fn gread_with<T>(&'a self, offset: &mut usize, ctx: Ctx) -> Result<T, E>
        where T: TryRead<'a, Ctx, E>;
}

pub trait Pwrite<Ctx, E>
    where Self: Sized
{
    fn pwrite<T>(&mut self, offset: usize, t: T) -> Result<(), E>
        where T: TryWrite<Ctx, E>,
              Ctx: Default
    {
        self.pwrite_with(offset, t, Default::default())
    }

    fn pwrite_with<T>(&mut self, offset: usize, t: T, ctx: Ctx) -> Result<(), E>
        where T: TryWrite<Ctx, E>;

    fn gwrite<T>(&mut self, offset: &mut usize, t: T) -> Result<(), E>
        where T: TryWrite<Ctx, E>,
              Ctx: Default
    {
        self.gwrite_with(offset, t, Default::default())
    }

    fn gwrite_with<T>(&mut self, offset: &mut usize, t: T, ctx: Ctx) -> Result<(), E>
        where T: TryWrite<Ctx, E>;
}


impl<'a, Ctx, E, Slice> Pread<'a, Ctx, E> for Slice
    where Slice: AsRef<[u8]>
{
    #[inline]
    fn pread_with<T>(&'a self, offset: usize, ctx: Ctx) -> Result<T, E>
        where T: TryRead<'a, Ctx, E>
    {
        let slice = self.as_ref();

        if offset >= slice.len() {
            return Err(Error::BadOffset(offset));
        };

        TryRead::try_read(&slice[offset..], ctx).map(|(t, _)| t)
    }

    #[inline]
    fn gread_with<T>(&'a self, offset: &mut usize, ctx: Ctx) -> Result<T, E>
        where T: TryRead<'a, Ctx, E>
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
}

impl<Ctx, E, Slice> Pwrite<Ctx, E> for Slice
    where Slice: AsMut<[u8]>
{
    fn pwrite_with<T>(&mut self, offset: usize, t: T, ctx: Ctx) -> Result<(), E>
        where T: TryWrite<Ctx, E>
    {
        let mut slice = self.as_mut();

        if offset >= slice.len() {
            return Err(Error::BadOffset(offset));
        };

        TryWrite::try_write(t, &mut slice[offset..], ctx).map(|_| ())
    }

    fn gwrite_with<T>(&mut self, offset: &mut usize, t: T, ctx: Ctx) -> Result<(), E>
        where T: TryWrite<Ctx, E>
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