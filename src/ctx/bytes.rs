use crate::{check_len, Error, Result, TryRead, TryWrite};

use super::{Len, Pattern, PatternUntil};

impl<'a> TryRead<'a, Len> for &'a [u8] {
    #[inline]
    fn try_read(bytes: &'a [u8], Len(len): Len) -> Result<(Self, usize)> {
        check_len(bytes, len)?;
        Ok((&bytes[..len], len))
    }
}

impl<'a> TryRead<'a, Pattern> for &'a [u8] {
    #[inline]
    fn try_read(bytes: &'a [u8], Pattern(pattern): Pattern) -> Result<(Self, usize)> {
        if pattern.is_empty() {
            return Err(Error::BadInput {
                err: "Pattern is empty",
            });
        }
        check_len(bytes, pattern.len())?;
        let len = (0..bytes.len() - pattern.len() + 1)
            .map(|n| bytes[n..].starts_with(pattern))
            .position(|p| p)
            .map(|len| len + pattern.len())
            .ok_or(Error::Incomplete)?;
        Ok((&bytes[..len], len))
    }
}

impl<'a> TryRead<'a, PatternUntil> for &'a [u8] {
    #[inline]
    fn try_read(
        bytes: &'a [u8],
        PatternUntil(pattern, len): PatternUntil,
    ) -> Result<(Self, usize)> {
        if pattern.is_empty() {
            return Err(Error::BadInput {
                err: "Pattern is empty",
            });
        }
        if pattern.len() > len {
            return Err(Error::BadInput {
                err: "Pattern is longer than restricted length",
            });
        }
        check_len(bytes, pattern.len())?;
        let len = (0..bytes.len() - pattern.len() + 1)
            .map(|n| bytes[n..].starts_with(pattern))
            .take(len - pattern.len())
            .position(|p| p)
            .map(|position| position + pattern.len())
            .unwrap_or(check_len(bytes, len)?);
        Ok((&bytes[..len], len))
    }
}

impl<'a> TryWrite for &'a [u8] {
    #[inline]
    fn try_write(self, bytes: &mut [u8], _ctx: ()) -> Result<usize> {
        check_len(bytes, self.len())?;

        bytes[..self.len()].copy_from_slice(self);

        Ok(self.len())
    }
}
