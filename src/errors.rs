use std::{num::TryFromIntError, string::FromUtf8Error};

use thiserror::Error;

#[derive(Error, Debug)]
/// Common errors that can occur in hkscdis-rs.
pub enum HkscError {
    #[error("Unknown type: {0}!")]
    /// This error occurs when an unknown type (outside of `HSType`) is found.
    UnknownType(u8),
    #[error("Failed to read from buffer!")]
    /// This error occurs when any kind of IO error occurs.
    ReadError(#[from] std::io::Error),
    #[error("Incorrect LIGHTUSERDATA size: {0}!")]
    /// LIGHTUSERDATA can have either 4 or 8 byte values.
    /// This error occurs when a value other than those are found.
    InvalidLightUserDataSize(u8),
    #[error("Unsupported constant type: {0}!")]
    /// This error occurs when an unknown constant type (outside of `HSValue`) is found.
    UnsupportedConstantType(u8),
    #[error("Invalid UTF-8 string read!")]
    /// This error occurs when invalid UTF-8 encoding is found while reading `HSType::TSTRING`.
    InvalidUTF8(#[from] FromUtf8Error),
    #[error("Invalid number size: {0}!")]
    /// Numbers can be either `f32` or `f64`.
    /// This error occurs when a value other than those are found.
    InvalidNumberSize(u8),
    #[error("Invalid string size: {0}!")]
    /// String length can be either 4 or 8 bytes.
    /// This error occurs when a value other than those are found.
    InvalidStringSize(u8),
    #[error("Integer overflow!")]
    /// This error occurs when an integer cast overflows.
    TryFromInt(#[from] TryFromIntError)
}
