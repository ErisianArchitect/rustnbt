#![allow(unused)]
pub mod family;
pub mod io;
pub(crate) mod table;
pub mod tag;
pub mod macros;

use thiserror::Error as ThisError;

/// This is the Error type returned from NbtRead and NbtWrite operations that fail.
#[derive(ThisError, Debug)]
pub enum NbtError {
    #[error("io error.")]
    IO(#[from] std::io::Error),
    #[error("Failed to read UTF-8 string.")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Unsupported Tag ID.")]
    Unsupported,
}

#[cfg(feature = "preserve_order")]
use indexmap::IndexMap;
#[cfg(feature = "preserve_order")]
pub type Map = IndexMap<String, tag::Tag>;
#[cfg(not(feature = "preserve_order"))]
pub type Map = std::collections::HashMap<String, tag::Tag>;

/// A const function that returns the number of bytes that size kibibytes would be.
pub const fn kibibytes(size: usize) -> usize {
    size << 10
}

/// A const function that returns the number of bytes that size mebibytes would be.
pub const fn mebibytes(size: usize) -> usize {
    size << 20
}

/// A const function that returns the number of bytes that size gibibytes would be.
pub const fn gibibytes(size: usize) -> usize {
    size << 30
}

/// This function converts a Vec<u8> into a Vec<i8> safely using compiler magic.
fn safe_vec_u8_to_vec_i8(v: Vec<u8>) -> Vec<i8> {
    v.into_iter().map(|x| x as i8).collect()
}