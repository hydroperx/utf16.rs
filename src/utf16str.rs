use super::{Utf16Str, Utf16String};

impl Utf16Str {
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
}

impl std::ops::Index<usize> for Utf16Str {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        self.raw.get(index).expect("Reading position out of bounds of Utf16Str.")
    }
}