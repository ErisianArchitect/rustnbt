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
    Unsupported{ id_encountered: u8 },
    #[error("Encountered the End tag ID marker.")]
    End,
}

// indexmap preserves the insertion order of elements.
// Minecraft does not care what order elements are in, one thing to note
// is that without having insertion order preserved, the order of elements
// in a compound is indeterminable.
// Preserving the order ensures isomorphism between input and output.
#[cfg(feature = "preserve_order")]
use indexmap::IndexMap;
#[cfg(feature = "preserve_order")]
/// The mapping type used for Tag::Compound.
pub type Map = IndexMap<String, tag::Tag>;
// Fallback to HashMap.
#[cfg(not(feature = "preserve_order"))]
/// The mapping type used for Tag::Compound.
pub type Map = std::collections::HashMap<String, tag::Tag>;

// The following three functions are to make it easier
// to create buffers of appropriate sizes.
/// A const function for unit conversion of kibibytes to bytes
pub const fn kibibytes(size: usize) -> usize {
    size << 10
}

/// A const function for unit conversion of mebibytes to bytes
pub const fn mebibytes(size: usize) -> usize {
    size << 20
}

/// A const function for unit conversion of gibibytes to bytes
pub const fn gibibytes(size: usize) -> usize {
    size << 30
}

/// This function converts a Vec<u8> into a Vec<i8> safely using compiler magic.
fn safe_vec_u8_to_vec_i8(v: Vec<u8>) -> Vec<i8> {
    v.into_iter().map(|x| x as i8).collect()
}