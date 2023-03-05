//! Crate prelude

pub use crate::error::FBIError;
pub type Result<T> = core::result::Result<T, FBIError>;

// Personal preference.
pub use std::format as f;
pub use std::fs;
pub use std::io;
