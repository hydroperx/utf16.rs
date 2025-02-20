use crate::{utils::*, Utf16Str};

pub struct Utf16Chars<'a> {
    pub(crate) slice: &'a Utf16Str,
    pub(crate) index: usize,
}

impl<'a> Iterator for Utf16Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slice.raw.len() {
            let i = self.index;
            let cu1 = self.slice.raw[i];
            if is_high_surrogate(cu1) && (i + 1) < self.slice.raw.len() {
                let cu2 = self.slice.raw[i + 1];
                if is_low_surrogate(cu2) {
                    self.index += 2;
                    return Some(decode_char(cu1, cu2));
                }
            }
            self.index += 1;
            unsafe { Some(char::from_u32_unchecked(cu1 as u32)) }
        } else {
            None
        }
    }
}

pub struct Utf16CharIndices<'a> {
    pub(crate) slice: &'a Utf16Str,
    pub(crate) index: usize,
}

impl<'a> Iterator for Utf16CharIndices<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slice.raw.len() {
            let i = self.index;
            let cu1 = self.slice.raw[i];
            if is_high_surrogate(cu1) && (i + 1) < self.slice.raw.len() {
                let cu2 = self.slice.raw[i + 1];
                if is_low_surrogate(cu2) {
                    self.index += 2;
                    return Some((i, decode_char(cu1, cu2)));
                }
            }
            self.index += 1;
            unsafe { Some((i, char::from_u32_unchecked(cu1 as u32))) }
        } else {
            None
        }
    }
}