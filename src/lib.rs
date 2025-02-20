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

#[cfg(test)]
mod tests {
    use super::Utf16String;

    #[test]
    fn test_iter() {
        let string = Utf16String::from("a\u{10000}");
        let mut chars = string.chars();
        assert_eq!(chars.next().unwrap(), 'a');
        assert_eq!(chars.next().unwrap(), '\u{10000}');

        let string = Utf16String::from("\u{10000}\u{10FFFF}");
        let mut chars = string.chars();
        assert_eq!(chars.next().unwrap(), '\u{10000}');
        assert_eq!(chars.next().unwrap(), '\u{10FFFF}');
    }

    #[test]
    fn test_length() {
        let mut string1 = Utf16String::from("a\u{10000}");
        assert_eq!(string1.len(), 3);
        string1.pop();
        assert_eq!(string1.len(), 1);
    }

    #[test]
    fn test_slicing() {
        let string1 = Utf16String::from("a\u{10000}");
        assert_eq!(string1[1..3].len(), 2);
        assert_eq!(string1[0..1].len(), 1);
        assert_eq!(string1[0..].len(), 3);
        assert_eq!(string1[1..].len(), 2);
        assert_eq!(string1[..1].len(), 1);
    }
}