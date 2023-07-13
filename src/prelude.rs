//! Crate prelude

pub use crate::error::IVError;
pub type Result<T> = core::result::Result<T, IVError>;

// Personal preference.
pub use std::format as f;
pub use std::fs;
pub use std::io;
