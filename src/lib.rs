pub mod slice;

mod utf16str;
mod utf16string;

/// A UTF-16 string slice consisting of UCS-2 code units.
#[derive(Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Utf16Str {
    /// UTF-16 code units.
    raw: [u16],
}

/// An owned UTF-16 string consisting of UCS-2 code units.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Utf16String {
    /// UTF-16 code units.
    buf: Vec<u16>,
}