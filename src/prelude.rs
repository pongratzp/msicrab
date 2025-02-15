pub use crate::error::CustomError;

pub type Result<T> = core::result::Result<T, CustomError>;

pub use std::format as f;