#![allow(unused_parens)]

use crate::{check_len, Error, Result, TryRead, TryWrite};

/// Trait for converting between native and big/little endian numbers.
pub trait Endianess: Copy {
    fn convert_u16(i: u16) -> u16;
    fn convert_u32(i: u32) -> u32;
    fn convert_u64(i: u64) -> u64;
    fn convert_u128(i: u128) -> u128;
    fn convert_usize(i: usize) -> usize;
    fn convert_i16(i: i16) -> i16;
    fn convert_i32(i: i32) -> i32;
    fn convert_i64(i: i64) -> i64;
    fn convert_i128(i: i128) -> i128;
    fn convert_isize(i: isize) -> isize;
}

/// Little endian byte order.
///
/// # Example
///
/// ```
/// use byte::*;
///
/// let bytes: &[u8] = &[0x00, 0xff];
/// let num_le: u16 = bytes.read_with(&mut 0, LE).unwrap();
/// assert_eq!(num_le, 0xff00);
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LittleEndian;

impl Endianess for LittleEndian {
    #[inline]
    fn convert_u16(i: u16) -> u16 {
        i.to_le()
    }
    #[inline]
    fn convert_u32(i: u32) -> u32 {
        i.to_le()
    }
    #[inline]
    fn convert_u64(i: u64) -> u64 {
        i.to_le()
    }
    #[inline]
    fn convert_u128(i: u128) -> u128 {
        i.to_le()
    }
    #[inline]
    fn convert_usize(i: usize) -> usize {
        i.to_le()
    }
    #[inline]
    fn convert_i16(i: i16) -> i16 {
        i.to_le()
    }
    #[inline]
    fn convert_i32(i: i32) -> i32 {
        i.to_le()
    }
    #[inline]
    fn convert_i64(i: i64) -> i64 {
        i.to_le()
    }
    #[inline]
    fn convert_i128(i: i128) -> i128 {
        i.to_le()
    }
    #[inline]
    fn convert_isize(i: isize) -> isize {
        i.to_le()
    }
}

/// Big endian byte order.
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
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BigEndian;

impl Endianess for BigEndian {
    #[inline]
    fn convert_u16(i: u16) -> u16 {
        i.to_be()
    }
    #[inline]
    fn convert_u32(i: u32) -> u32 {
        i.to_be()
    }
    #[inline]
    fn convert_u64(i: u64) -> u64 {
        i.to_be()
    }
    #[inline]
    fn convert_u128(i: u128) -> u128 {
        i.to_be()
    }
    #[inline]
    fn convert_usize(i: usize) -> usize {
        i.to_be()
    }
    #[inline]
    fn convert_i16(i: i16) -> i16 {
        i.to_be()
    }
    #[inline]
    fn convert_i32(i: i32) -> i32 {
        i.to_be()
    }
    #[inline]
    fn convert_i64(i: i64) -> i64 {
        i.to_be()
    }
    #[inline]
    fn convert_i128(i: i128) -> i128 {
        i.to_be()
    }
    #[inline]
    fn convert_isize(i: isize) -> isize {
        i.to_be()
    }
}

/// Little endian byte order
pub const LE: LittleEndian = LittleEndian;
/// Big endian byte order
pub const BE: BigEndian = BigEndian;

/// Network endian
pub const NETWORK: LittleEndian = LittleEndian;

/// The machine's native endianness
#[cfg(target_endian = "little")]
pub const NATIVE: LittleEndian = LittleEndian;
/// The machine's native endiannes
#[cfg(target_endian = "big")]
pub const NATIVE: BigEndian = BigEndian;

macro_rules! num_impl {
    ($ty: ty, $convert: ident) => {
        impl<'a, Ctx: Endianess> TryRead<'a, Ctx> for $ty {
            #[inline]
            fn try_read(bytes: &'a [u8], _: Ctx) -> Result<(Self, usize)> {
                let size = ::core::mem::size_of::<$ty>();
                check_len(bytes, size)?;
                let val = <$ty>::from_ne_bytes(bytes[..size].try_into().map_err(|_e| {
                    Error::BadInput {
                        err: "TryIntoSliceError",
                    }
                })?);
                Ok((<Ctx as Endianess>::$convert(val), size))
            }
        }

        impl<Ctx: Endianess> TryWrite<Ctx> for $ty {
            #[inline]
            fn try_write(self, bytes: &mut [u8], _: Ctx) -> Result<usize> {
                let size = ::core::mem::size_of::<$ty>();
                check_len(bytes, size)?;
                bytes[..size].copy_from_slice(&<Ctx as Endianess>::$convert(self).to_ne_bytes());
                Ok(size)
            }
        }
    };
}

num_impl!(u16, convert_u16);
num_impl!(u32, convert_u32);
num_impl!(u64, convert_u64);
num_impl!(u128, convert_u128);
num_impl!(i16, convert_i16);
num_impl!(i32, convert_i32);
num_impl!(i64, convert_i64);
num_impl!(i128, convert_i128);
num_impl!(usize, convert_usize);
num_impl!(isize, convert_isize);

macro_rules! float_impl {
    ($ty: ty, $base: ty) => {
        impl<'a, Ctx: Endianess> TryRead<'a, Ctx> for $ty {
            #[inline]
            fn try_read(bytes: &'a [u8], endian: Ctx) -> Result<(Self, usize)> {
                <$base as TryRead<'a, Ctx>>::try_read(bytes, endian)
                    .map(|(val, size)| (<$ty>::from_bits(val), size))
            }
        }

        impl<'a, Ctx: Endianess> TryWrite<Ctx> for $ty {
            #[inline]
            fn try_write(self, bytes: &mut [u8], endian: Ctx) -> Result<usize> {
                <$base as TryWrite<Ctx>>::try_write(self.to_bits(), bytes, endian)
            }
        }
    };
}

float_impl!(f32, u32);
float_impl!(f64, u64);

impl<Ctx> TryRead<'_, Ctx> for u8 {
    #[inline]
    fn try_read(bytes: &[u8], _: Ctx) -> Result<(Self, usize)> {
        check_len(bytes, 1)?;
        Ok((bytes[0], 1))
    }
}

impl<Ctx> TryWrite<Ctx> for u8 {
    #[inline]
    fn try_write(self, bytes: &mut [u8], _: Ctx) -> Result<usize> {
        check_len(bytes, 1)?;
        bytes[0] = self;
        Ok(1)
    }
}

impl<Ctx> TryRead<'_, Ctx> for i8 {
    #[inline]
    fn try_read(bytes: &[u8], _: Ctx) -> Result<(Self, usize)> {
        check_len(bytes, 1)?;
        Ok((bytes[0] as i8, 1))
    }
}

impl<Ctx> TryWrite<Ctx> for i8 {
    #[inline]
    fn try_write(self, bytes: &mut [u8], _: Ctx) -> Result<usize> {
        check_len(bytes, 1)?;
        bytes[0] = self as u8;
        Ok(1)
    }
}
