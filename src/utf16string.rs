use crate::{Utf16Str, Utf16String};
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