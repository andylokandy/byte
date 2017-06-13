use std::str;

pub trait TryFromCtx<'a, Ctx = ()>
    where Self: 'a + Sized
{
    fn try_from_ctx(scroll: &'a [u8], ctx: Ctx) -> Result<(Self, usize), ()>;
}

trait Pread<'a> {
    fn pread<T, Ctx>(self, offset: usize) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Ctx: Default,
              Self: 'a
    {
        self.pread_with(offset, Default::default())
    }

    fn pread_with<T, Ctx>(self, offset: usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Self: 'a;

    fn gread<T, Ctx>(self, offset: &mut usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Ctx: Default,
              Self: 'a
    {
        self.gread_with(offset, Default::default())
    }

    fn gread_with<T, Ctx>(self, offset: &mut usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Self: 'a;
}

#[derive(Debug)]
pub enum StrCtx {
    Delimiter(u8),
    Length(usize),
}

impl<'a> TryFromCtx<'a, StrCtx> for &'a str {
    #[inline]
    fn try_from_ctx(scroll: &'a [u8], ctx: StrCtx) -> Result<(Self, usize), ()> {
        let len = match ctx {
            StrCtx::Delimiter(delimiter) => scroll.iter().take_while(|c| **c != delimiter).count(),
            StrCtx::Length(len) => len,
        };

        if len > scroll.len() {
            return Err(());
        };

        str::from_utf8(&scroll[..len])
            .map(|s| (s, len))
            .map_err(|_| ())
    }
}

impl<'a> Pread<'a> for &'a [u8] {
    #[inline]
    fn pread_with<T, Ctx>(self: &'a [u8], offset: usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Self: 'a
    {
        if offset >= scroll.len() {
            return Err(());
        };

        TryFromCtx::try_from_ctx(&self[offset..], ctx).map(|(t, _)| t)
    }

    #[inline]
    fn gread_with<T, Ctx>(self: &'a [u8], offset: &mut usize, ctx: Ctx) -> Result<T, ()>
        where T: TryFromCtx<'a, Ctx>,
              Self: 'a
    {
        if offset >= scroll.len() {
            return Err(());
        };

        TryFromCtx::try_from_ctx(&self[*offset..], ctx).map(|(t, size)| {
                                                                *offset += size;
                                                                t
                                                            })
    }
}

#[test]
fn test_str() {
    let bytes: &[u8] = b"hello, world!\0some_other_things";
    let s: &str = bytes.pread_with(0, StrCtx::Delimiter(0)).unwrap();
    assert_eq!(s, "hello, world!");

    let bytes: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let s: &str = bytes.pread_with(0, StrCtx::Length(15)).unwrap();
    assert_eq!(s, "abcdefghijklmno");
    assert!(bytes
                .pread_with::<&str, _>(0, StrCtx::Length(27))
                .is_err());

    let bytes: &[u8] = b"hello, world!\0some_other_things";
    let mut offset = 0;
    let s: &str = bytes
        .gread_with(&mut offset, StrCtx::Delimiter(0))
        .unwrap();
    assert_eq!(s, "hello, world!");
    assert_eq!(offset, 13);
}