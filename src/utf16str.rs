use crate::{Utf16Chars, Utf16CharIndices, Utf16Str, Utf16String};

impl Utf16Str {
    /// Returns the number of UTF-16 code units representing the string.
    #[inline]
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u16 {
        self.raw.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        self.raw.as_mut_ptr()
    }

    pub unsafe fn from_utf16_unchecked(raw: &[u16]) -> &Self {
        unsafe { &*(raw as *const [u16] as *const Self) }
    }

    pub unsafe fn from_utf16_unchecked_mut(raw: &mut [u16]) -> &mut Self {
        unsafe { &mut *(raw as *mut [u16] as *mut Self) }
    }

    /// Iterates the UTF-16 code units.
    pub fn code_units(&self) -> std::slice::Iter<u16> {
        self.raw.iter()
    }

    /// Iterates the code points in the string.
    pub fn chars(&self) -> Utf16Chars {
        Utf16Chars {
            slice: self,
            index: 0,
        }
    }

    /// Iterates the indices and their code pointss in the string.
    pub fn char_indices(&self) -> Utf16CharIndices {
        Utf16CharIndices {
            slice: self,
            index: 0,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.raw.len() == 0
    }

    pub fn to_owned(&self) -> Utf16String {
        Utf16String {
            buf: self.raw.to_owned(),
        }
    }
}

impl std::ops::Index<usize> for Utf16Str {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        self.raw.get(index).expect("Reading position out of bounds of Utf16Str.")
    }
}