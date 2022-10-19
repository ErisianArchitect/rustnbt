#![allow(unused)]
pub(crate) mod family;
pub mod io;
pub(crate) mod table;
pub mod tag;

use thiserror::Error as ThisError;

#[cfg(feature = "preserve_order")]
use indexmap::IndexMap;
#[cfg(feature = "preserve_order")]
pub type Map = IndexMap<String, tag::Tag>;
#[cfg(not(feature = "preserve_order"))]
pub type Map = std::collections::HashMap<String, tag::Tag>;