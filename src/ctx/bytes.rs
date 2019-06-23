use {check_len, Error, Result, TryRead, TryWrite};

/// Context for &[u8] to determine where the slice ends.
///
/// Pattern will be included in the result
///
/// # Example
///
/// ```
/// use byte::*;
/// use byte::ctx::*;
///
/// let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef, 0x00, 0xff];
///
/// let sub: &[u8] = bytes.read_with(&mut 0, Bytes::Len(2)).unwrap();
/// assert_eq!(sub, &[0xde, 0xad]);
///
/// static PATTERN: &'static [u8; 2] = &[0x00, 0xff];
///
/// let sub: &[u8] = bytes.read_with(&mut 0, Bytes::Pattern(PATTERN)).unwrap();
/// assert_eq!(sub, &[0xde, 0xad, 0xbe, 0xef, 0x00, 0xff]);
///
/// let sub: &[u8] = bytes.read_with(&mut 0, Bytes::PatternUntil(PATTERN, 4)).unwrap();
/// assert_eq!(sub, &[0xde, 0xad, 0xbe, 0xef]);
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Bytes {
    /// Take fix-length bytes
    Len(usize),
    /// Take bytes until reaching a byte pattern
    Pattern(&'static [u8]),
    /// Take bytes until either byte pattern or length reached
    PatternUntil(&'static [u8], usize),
}

impl<'a> TryRead<'a, Bytes> for &'a [u8] {
    #[inline]
    fn try_read(bytes: &'a [u8], ctx: Bytes) -> Result<(Self, usize)> {
        let len = match ctx {
            Bytes::Len(len) => check_len(bytes, len)?,
            Bytes::Pattern(pattern) => {
                if pattern.len() == 0 {
                    return Err(Error::BadInput {
                        err: "Pattern is empty",
                    });
                }
                check_len(bytes, pattern.len())?;
                (0..bytes.len() - pattern.len() + 1)
                    .map(|n| bytes[n..].starts_with(pattern))
                    .position(|p| p)
                    .map(|len| len + pattern.len())
                    .ok_or(Error::Incomplete)?
            }
            Bytes::PatternUntil(pattern, len) => {
                if pattern.len() == 0 {
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
                (0..bytes.len() - pattern.len() + 1)
                    .map(|n| bytes[n..].starts_with(pattern))
                    .take(len - pattern.len())
                    .position(|p| p)
                    .map(|position| position + pattern.len())
                    .unwrap_or(check_len(bytes, len)?)
            }
        };

        Ok((&bytes[..len], len))
    }
}

impl<'a> TryWrite for &'a [u8] {
    #[inline]
    fn try_write(self, bytes: &mut [u8], _ctx: ()) -> Result<usize> {
        check_len(bytes, self.len())?;

        bytes[..self.len()].clone_from_slice(self);

        Ok(self.len())
    }
}
