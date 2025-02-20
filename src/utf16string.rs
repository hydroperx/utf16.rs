use crate::{utils::*, Utf16Str, Utf16String};
use std::ops::{Deref, DerefMut};

impl Deref for Utf16String {
    type Target = Utf16Str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { Utf16Str::from_utf16_unchecked(self.buf.as_slice()) }
    }
}

impl DerefMut for Utf16String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { Utf16Str::from_utf16_unchecked_mut(self.buf.as_mut_slice()) }
    }
}

impl<T: AsRef<str>> From<T> for Utf16String {
    fn from(value: T) -> Self {
        let mut r = Utf16String::new();
        r.push_utf8_str(value.as_ref());
        r
    }
}

impl From<Utf16String> for String {
    fn from(value: Utf16String) -> Self {
        value.to_utf8()
    }
}

impl Utf16String {
    pub fn new() -> Self {
        Utf16String {
            buf: vec![],
        }
    }

    #[inline]
    pub fn as_mut_utf16_str(&mut self) -> &mut Utf16Str {
        self
    }

    #[inline]
    pub unsafe fn as_mut_vec(&mut self) -> &mut Vec<u16> {
        &mut self.buf
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn insert(&mut self, mut index: usize, ch: char) {
        for cu in encode_char(ch) {
            self.buf.insert(index, cu);
            index += 1;
        }
    }

    pub fn insert_utf16_str(&mut self, mut index: usize, string: &Utf16Str) {
        for cu in string.raw.iter() {
            self.buf.insert(index, *cu);
            index += 1;
        }
    }

    pub fn insert_utf8_str(&mut self, mut index: usize, string: &str) {
        for ch in string.chars() {
            for cu in encode_char(ch) {
                self.buf.insert(index, cu);
                index += 1;
            }
        }
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        self.buf.extend(encode_char(ch));
    }

    pub fn push_utf16_str(&mut self, string: &Utf16Str) {
        self.buf.extend(&string.raw);
    }

    pub fn push_utf8_str(&mut self, string: &str) {
        for ch in string.chars() {
            self.buf.extend(encode_char(ch));
        }
    }

    /// Removes a surrogate pair or a code unit from the specified
    /// index in code units, and returns the code point that was
    /// removed.
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> char {
        assert!(index < self.len(), "removing character out of bounsd");
        let cu1 = self.buf[index];
        if is_high_surrogate(cu1) && (index + 1) < self.len() {
            let cu2 = self.buf[index + 1];
            if is_low_surrogate(cu2) {
                self.buf.remove(index);
                self.buf.remove(index);
                return decode_char(cu1, cu2);
            }
        }
        self.buf.remove(index);
        unsafe { char::from_u32_unchecked(cu1 as u32) }
    }

    /// Removes the last surrogate pair or code unit, and returns the code point that was
    /// removed.
    pub fn pop(&mut self) -> Option<char> {
        let l = self.len();
        if l == 0 {
            return None;
        }
        let i = l - 1;
        let cu2 = self.buf[i];
        if is_low_surrogate(cu2) && l > 1 {
            let cu1 = self.buf[i - 1];
            if is_high_surrogate(cu1) {
                let i2 = i - 1;
                self.buf.remove(i2);
                self.buf.remove(i2);
                return Some(decode_char(cu1, cu2));
            }
        }
        self.buf.remove(i);
        Some(unsafe { char::from_u32_unchecked(cu2 as u32) })
    }
}

impl Default for Utf16String {
    fn default() -> Self {
        Utf16String::new()
    }
}