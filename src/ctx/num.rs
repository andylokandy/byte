#![allow(unused_parens)]

use crate::{check_len, Error, Measure, Result, TryRead, TryWrite};
use core::mem;

/// Endiannes of numbers.
///
/// Defaults to the machine's endianness.
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

/// Little endian byte order
pub const LE: Endian = Endian::Little;
/// Big endian byte order
pub const BE: Endian = Endian::Big;

/// Network endian
pub const NETWORK: Endian = Endian::Little;

/// The machine's native endianness
#[cfg(target_endian = "little")]
pub const NATIVE: Endian = LE;
/// The machine's native endiannes
#[cfg(target_endian = "big")]
pub const NATIVE: Endian = BE;

macro_rules! num_impl {
    ($ty: ty, $size: tt) => {
        impl<Ctx> Measure<Ctx> for $ty {
            #[inline]
            fn measure(self, _: Ctx) -> usize {
                ::core::mem::size_of::<$ty>()
            }
        }

        impl<'a> TryRead<'a, Endian> for $ty {
            #[inline]
            fn try_read(bytes: &'a [u8], endian: Endian) -> Result<(Self, usize)> {
                check_len(bytes, $size)?;

                let val = match endian {
                    Endian::Big => {
                        <$ty>::from_be_bytes(bytes[..$size].try_into().map_err(|_e| {
                            Error::BadInput {
                                err: "TryIntoSliceError",
                            }
                        })?)
                    }
                    Endian::Little => {
                        <$ty>::from_le_bytes(bytes[..$size].try_into().map_err(|_e| {
                            Error::BadInput {
                                err: "TryIntoSliceError",
                            }
                        })?)
                    }
                };

                Ok((val, $size))
            }
        }

        impl TryWrite<Endian> for $ty {
            #[inline]
            fn try_write(self, bytes: &mut [u8], endian: Endian) -> Result<usize> {
                check_len(bytes, $size)?;

                let _val = match endian {
                    Endian::Big => bytes[..$size].copy_from_slice(&self.to_be_bytes()),
                    Endian::Little => bytes[..$size].copy_from_slice(&self.to_le_bytes()),
                };

                Ok($size)
            }
        }
    };
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
        impl<Ctx> Measure<Ctx> for $ty {
            #[inline]
            fn measure(self, _: Ctx) -> usize {
                ::core::mem::size_of::<$ty>()
            }
        }

        impl<'a> TryRead<'a, Endian> for $ty {
            #[inline]
            fn try_read(bytes: &'a [u8], endian: Endian) -> Result<(Self, usize)> {
                <$base as TryRead<'a, Endian>>::try_read(bytes, endian)
                    .map(|(val, size)| (<$ty>::from_bits(val), size))
            }
        }

        impl<'a> TryWrite<Endian> for $ty {
            #[inline]
            fn try_write(self, bytes: &mut [u8], endian: Endian) -> Result<usize> {
                <$base as TryWrite<Endian>>::try_write(self.to_bits(), bytes, endian)
            }
        }
    };
}

float_impl!(f32, u32);
float_impl!(f64, u64);
