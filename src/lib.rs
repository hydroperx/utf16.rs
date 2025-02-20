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

    #[test]
    fn test_offset_conversion() {
        use crate::utils::*;

        // \u{10000} is 4 bytes, so 'b' is at 1 + 4 = 5
        let utf8string = "a\u{10000}b";
        // \u{10000} is 2 code units, so 'b' is at 1 + 2 = 3
        let utf16string = Utf16String::from(utf8string);

        // test '\u{10000}' as first offset and 'b' as last offset
        assert_eq!(two_utf16_offsets_as_utf8_offsets(utf8string, &utf16string, 1, 3), (1, 5));
        assert_eq!(two_utf8_offsets_as_utf16_offsets(&utf16string, utf8string, 1, 5), (1, 3));

        // 'b' is at 1 + 4 = 5, and \u{10000} is at 1 + 4 + 1 = 6
        let utf8string = "a\u{10FFFF}b\u{10000}";
        // 'b' is at 1 + 2 = 3, and \u{10000} is at 1 + 2 + 1 = 4
        let utf16string = Utf16String::from(utf8string);

        // test 'b' as first offset and '\u{10000}' as last offset
        assert_eq!(two_utf16_offsets_as_utf8_offsets(utf8string, &utf16string, 3, 4), (5, 6));
        assert_eq!(two_utf8_offsets_as_utf16_offsets(&utf16string, utf8string, 5, 6), (3, 4));
    }
}