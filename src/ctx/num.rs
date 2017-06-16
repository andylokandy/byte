use {TryFromCtx, TryIntoCtx, Error, Result};
use std::mem;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

impl Default for Endian {
    #[inline]
    fn default() -> Self {
        NATIVE
    }
}

pub const LE: Endian = Endian::Little;
pub const BE: Endian = Endian::Big;

#[cfg(target_endian = "little")]
pub const NATIVE: Endian = LE;
#[cfg(target_endian = "big")]
pub const NATIVE: Endian = BE;

macro_rules! num_impl {
    ($ty: ty) => {

        impl<'a> TryFromCtx<'a, Endian> for $ty {
            #[inline]
            fn try_from_ctx(scroll: &'a [u8], endian: Endian) -> Result<(Self, usize), ()> {
                let size = mem::size_of::<$ty>();

                if size > scroll.len() {
                    return Err(Error::Incomplete);
                };

                let val: $ty = unsafe { *(&scroll[0] as *const _ as *const _) };
                let val = match endian {
                    Endian::Big => val.to_be(),
                    Endian::Little => val.to_le(),
                };

                Ok((val, size))
            }
        }

        impl<'a> TryIntoCtx<Endian> for $ty {
            #[inline]
            fn try_into_ctx(self, scroll: &mut [u8], endian: Endian) -> Result<usize, ()> {
                let size = mem::size_of::<$ty>();

                if size > scroll.len() {
                    return Err(Error::Incomplete);
                };

                let val = match endian {
                    Endian::Big => self.to_be(),
                    Endian::Little => self.to_le(),
                };

                unsafe { *(&mut scroll[0] as *mut _ as *mut _) = val };

                Ok(size)
            }
        }

    }
}

num_impl!(u8);
num_impl!(u16);
num_impl!(u32);
num_impl!(u64);
num_impl!(usize);
num_impl!(i8);
num_impl!(i16);
num_impl!(i32);
num_impl!(i64);
num_impl!(isize);

macro_rules! float_impl {
    ($ty: ty, $base: ty) => {

        impl<'a> TryFromCtx<'a, Endian> for $ty {
            #[inline]
            fn try_from_ctx(scroll: &'a [u8], endian: Endian) -> Result<(Self, usize), ()> {
                <$base as TryFromCtx<'a, Endian>>::try_from_ctx(scroll, endian)
                    .map(|(val, size)| (unsafe { mem::transmute(val) }, size))
            }
        }

        impl<'a> TryIntoCtx<Endian> for $ty {
            #[inline]
            fn try_into_ctx(self, scroll: &mut [u8], endian: Endian) -> Result<usize, ()> {
                <$base as TryIntoCtx<Endian>>::try_into_ctx(unsafe { mem::transmute(self) }, scroll, endian)
            }
        }

    }
}

float_impl!(f32, u32);
float_impl!(f64, u64);