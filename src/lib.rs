pub mod slice;
pub mod utils;
mod iterators;
pub use iterators::*;

mod utf16str;
mod utf16string;

/// A UTF-16 string slice consisting of UCS-2 code units.
/// 
/// Indexing this type is equivalent to indexing UTF-16 code units (not bytes),
/// which are represented by `u16`.
#[derive(Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Utf16Str {
    /// UTF-16 code units.
    pub(crate) raw: [u16],
}

/// An owned UTF-16 string consisting of UCS-2 code units.
///
/// Indexing this type is equivalent to indexing UTF-16 code units (not bytes),
/// which are represented by `u16`.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Utf16String {
    /// UTF-16 code units.
    pub(crate) buf: Vec<u16>,
}