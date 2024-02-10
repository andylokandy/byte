//! Context for primitives

mod bool;
mod bytes;
mod num;
mod str;

/// No context.
pub const NONE: () = ();

pub use self::bytes::*;
pub use self::num::*;
pub use self::str::*;
