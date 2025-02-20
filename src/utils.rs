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