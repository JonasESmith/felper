//! Crate Prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// generic wrapper tuple struct for newtype patterns
pub struct W<T>(pub T);

// alliase for format
pub use std::format as f;
