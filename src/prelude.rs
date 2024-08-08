//! Crate Prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// generic wrapper tuple struct for newtype patterns
// pub struct W<T> { pub field1: T }
