use crate::Utf16Str;

/// Encodes a code point into a sequence of code units.
/// The maximum number of elements returned is 2.
pub fn encode_char(ch: char) -> Vec<u16> {
    let mut val = ch as usize;
    if val >> 16 == 0 {
        return vec![val as u16];
    }
    val = val - 0x10000;
    let hi = (val >> 10) + 0xD800;
    let low = (val & 0b1111111111) + 0xDC00;
    vec![hi as u16, low as u16]
}

/// Decodes a high surrogate and low surrogate into a code point.
pub fn decode_char(hi: u16, low: u16) -> char {
    let hi = ((hi - 0xD800) as u32) * 0x400;
    let low = (low - 0xDC00) as u32;
    char::from_u32(hi + low + 0x10000).unwrap_or('\x00')
}

/// Determines whether an UTF-16 code unit is a high surrogate.
#[inline]
pub fn is_high_surrogate(cu: u16) -> bool {
    cu >> 10 == 0b110110
}

/// Determines whether an UTF-16 code unit is a low surrogate.
#[inline]
pub fn is_low_surrogate(cu: u16) -> bool {
    cu >> 10 == 0b110111
}

/// Assuming two strings are equal but in different encodings, returns
/// the UTF-16 offset equivalent to the given UTF-8 offset.
pub fn utf8_offset_as_utf16_offset(utf16string: &Utf16Str, utf8string: &str, utf8offset: usize) -> usize {
    let offset = utf8string[..utf8offset].chars().count();
    let mut i = 0usize;
    for (j, _) in utf16string.char_indices() {
        if i == offset {
            return j;
        }
        i += 1;
    }
    utf16string.len()
}

/// Assuming two strings are equal but in different encodings, returns
/// the UTF-16 offsets equivalent to the two given UTF-8 offsets;
/// every given offset is assummed to be successor to the previous one.
pub fn two_utf8_offsets_as_utf16_offsets(utf16string: &Utf16Str, utf8string: &str, utf8offset1: usize, utf8offset2: usize) -> (usize, usize) {
    let mut chars = utf8string[..utf8offset2].char_indices();
    let mut offset1 = 0usize;
    while let Some((j, _)) = chars.next() {
        if j == utf8offset1 {
            break;
        }
        offset1 += 1;
    }
    let offset2 = offset1 + chars.count();
    let mut i = 0usize;
    let mut utf16offset1 = 0;
    let mut chars = utf16string.char_indices();
    while let Some((j, _)) = chars.next() {
        if i == offset1 {
            utf16offset1 = j;
            break;
        }
        i += 1;
    }
    let mut utf16offset2 = 0;
    while let Some((j, _)) = chars.next() {
        if i == offset2 {
            utf16offset2 = j;
            break;
        }
        i += 1;
    }
    (utf16offset1, utf16offset2)
}

/// Assuming two strings are equal but in different encodings, returns
/// the UTF-8 offset equivalent to the given UTF-16 offset.
pub fn utf16_offset_as_utf8_offset(utf8string: &str, utf16string: &Utf16Str, utf16offset: usize) -> usize {
    let offset = utf16string[..utf16offset].chars().count();
    let mut i = 0usize;
    for (j, _) in utf8string.char_indices() {
        if i == offset {
            return j;
        }
        i += 1;
    }
    utf8string.len()
}

/// Assuming two strings are equal but in different encodings, returns
/// the UTF-8 offsets equivalent to the two given UTF-16 offsets;
/// every given offset is assummed to be successor to the previous one.
pub fn two_utf16_offsets_as_utf8_offsets(utf8string: &str, utf16string: &Utf16Str, utf16offset1: usize, utf16offset2: usize) -> (usize, usize) {
    let mut chars = utf16string[..utf16offset2].char_indices();
    let mut offset1 = 0usize;
    while let Some((j, _)) = chars.next() {
        if j == utf16offset1 {
            break;
        }
        offset1 += 1;
    }
    let offset2 = offset1 + chars.count();
    let mut i = 0usize;
    let mut utf8offset1 = 0;
    let mut chars = utf8string.char_indices();
    while let Some((j, _)) = chars.next() {
        if i == offset1 {
            utf8offset1 = j;
            break;
        }
        i += 1;
    }
    let mut utf8offset2 = 0;
    while let Some((j, _)) = chars.next() {
        if i == offset2 {
            utf8offset2 = j;
            break;
        }
        i += 1;
    }
    (utf8offset1, utf8offset2)
}