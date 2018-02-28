#![allow(unused_parens)]

use {TryRead, TryWrite, Result, check_len};
use core::mem;

/// Endian of numbers.
///
/// Default to machine's native endian.
///
/// # Example
///
/// ```
/// use byte::*;
///
/// let bytes: &[u8] = &[0x00, 0xff];
///
/// let num_be: u16 = bytes.read_with(&mut 0, BE).unwrap();
/// assert_eq!(num_be, 0x00ff);
///
/// let num_le: u16 = bytes.read_with(&mut 0, LE).unwrap();
/// assert_eq!(num_le, 0xff00);
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Endian {
    /// Little Endian byte order context
    Little,
    /// Big Endian byte order context
    Big,
}

impl Default for Endian {
    #[inline]
    fn default() -> Self {
        NATIVE
    }
}

/// Little Endian byte order
pub const LE: Endian = Endian::Little;
/// Big Endian byte order
pub const BE: Endian = Endian::Big;

/// Network endian
pub const NETWORK: Endian = Endian::Little;

/// The machine's native endian
#[cfg(target_endian = "little")]
pub const NATIVE: Endian = LE;
/// The machine's native endian
#[cfg(target_endian = "big")]
pub const NATIVE: Endian = BE;

macro_rules! num_impl {
    ($ty: ty, $size: tt) => {

        impl<'a> TryRead<'a, Endian> for $ty {
            #[inline]
            fn try_read(bytes: &'a [u8], endian: Endian) -> Result<(Self, usize)> {
                check_len(bytes, $size)?;

                let val: $ty = unsafe { *(&bytes[0] as *const _ as *const _) };
                let val = match endian {
                    Endian::Big => val.to_be(),
                    Endian::Little => val.to_le(),
                };

                Ok((val, $size))
            }
        }

        impl TryWrite<Endian> for $ty {
            #[inline]
            fn try_write(self, bytes: &mut [u8], endian: Endian) -> Result<usize> {
                check_len(bytes, $size)?;

                let val = match endian {
                    Endian::Big => self.to_be(),
                    Endian::Little => self.to_le(),
                };

                unsafe { *(&mut bytes[0] as *mut _ as *mut _) = val };

                Ok($size)
            }
        }

    }
}

num_impl!(u8, 1);
num_impl!(u16, 2);
num_impl!(u32, 4);
num_impl!(u64, 8);
num_impl!(i8, 1);
num_impl!(i16, 2);
num_impl!(i32, 4);
num_impl!(i64, 8);
num_impl!(usize, (mem::size_of::<usize>()));
num_impl!(isize, (mem::size_of::<isize>()));

macro_rules! float_impl {
    ($ty: ty, $base: ty) => {

        impl<'a> TryRead<'a, Endian> for $ty {
            #[inline]
            fn try_read(bytes: &'a [u8], endian: Endian) -> Result<(Self, usize)> {
                <$base as TryRead<'a, Endian>>::try_read(bytes, endian)
                    .map(|(val, size)| (unsafe { mem::transmute(val) }, size))
            }
        }

        impl<'a> TryWrite<Endian> for $ty {
            #[inline]
            fn try_write(self, bytes: &mut [u8], endian: Endian) -> Result<usize> {
                <$base as TryWrite<Endian>>::try_write(unsafe { mem::transmute(self) }, bytes, endian)
            }
        }

    }
}

float_impl!(f32, u32);
float_impl!(f64, u64);