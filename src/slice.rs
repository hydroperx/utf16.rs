// Based in https://github.com/getsentry/utf16string/blob/main/src/slicing.rs

use std::ops::{Index, IndexMut, Range, RangeFull, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use super::{Utf16Str, Utf16String};

mod private {
    use super::*;

    pub trait SealedSliceIndex {}

    impl SealedSliceIndex for RangeFull {}
    impl SealedSliceIndex for Range<usize> {}
    impl SealedSliceIndex for RangeFrom<usize> {}
    impl SealedSliceIndex for RangeTo<usize> {}
    impl SealedSliceIndex for RangeInclusive<usize> {}
    impl SealedSliceIndex for RangeToInclusive<usize> {}
}

pub trait SliceIndex<T>: private::SealedSliceIndex
where
    T: ?Sized,
{
    type Output: ?Sized;

    fn get(self, slice: &T) -> Option<&Self::Output>;

    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;

    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;

    fn index(self, slice: &T) -> &Self::Output;

    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

impl SliceIndex<Utf16Str> for RangeFull {
    type Output = Utf16Str;

    #[inline]
    fn get(self, slice: &Utf16Str) -> Option<&Self::Output> {
        Some(slice)
    }

    #[inline]
    fn get_mut(self, slice: &mut Utf16Str) -> Option<&mut Self::Output> {
        Some(slice)
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &Utf16Str) -> &Self::Output {
        slice
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        slice
    }

    #[inline]
    fn index(self, slice: &Utf16Str) -> &Self::Output {
        slice
    }

    #[inline]
    fn index_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        slice
    }
}

impl SliceIndex<Utf16Str> for Range<usize> {
    type Output = Utf16Str;

    #[inline]
    fn get(self, slice: &Utf16Str) -> Option<&Self::Output> {
        if self.start <= self.end && self.end <= slice.len() { Some(unsafe { self.get_unchecked(slice) }) } else { None }
    }

    #[inline]
    fn get_mut(self, slice: &mut Utf16Str) -> Option<&mut Self::Output> {
        if self.start <= self.end && self.end <= slice.len() { Some(unsafe { self.get_unchecked_mut(slice) }) } else { None }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &Utf16Str) -> &Self::Output {
        let ptr = unsafe { slice.as_ptr().add(self.start) };
        let len = self.end - self.start;
        unsafe { Utf16Str::from_utf16_unchecked(std::slice::from_raw_parts(ptr, len)) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        let ptr = unsafe { slice.as_mut_ptr().add(self.start) };
        let len = self.end - self.start;
        unsafe { Utf16Str::from_utf16_unchecked_mut(std::slice::from_raw_parts_mut(ptr, len)) }
    }

    #[inline]
    fn index(self, slice: &Utf16Str) -> &Self::Output {
        self.get(slice).expect("slice index out of bounds")
    }

    #[inline]
    fn index_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        self.get_mut(slice).expect("slice index out of bounds")
    }
}

impl SliceIndex<Utf16Str> for RangeTo<usize> {
    type Output = Utf16Str;

    #[inline]
    fn get(self, slice: &Utf16Str) -> Option<&Self::Output> {
        if self.end <= slice.len() { Some(unsafe { self.get_unchecked(slice) }) } else { None }
    }

    #[inline]
    fn get_mut(self, slice: &mut Utf16Str) -> Option<&mut Self::Output> {
        if self.end <= slice.len() { Some(unsafe { self.get_unchecked_mut(slice) }) } else { None }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &Utf16Str) -> &Self::Output {
        let ptr = slice.as_ptr();
        unsafe { Utf16Str::from_utf16_unchecked(std::slice::from_raw_parts(ptr, self.end)) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        let ptr = slice.as_mut_ptr();
        unsafe { Utf16Str::from_utf16_unchecked_mut(std::slice::from_raw_parts_mut(ptr, self.end)) }
    }

    #[inline]
    fn index(self, slice: &Utf16Str) -> &Self::Output {
        self.get(slice).expect("slice index out of bounds")
    }

    #[inline]
    fn index_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        self.get_mut(slice).expect("slice index out of bounds")
    }
}

impl SliceIndex<Utf16Str> for RangeFrom<usize> {
    type Output = Utf16Str;

    #[inline]
    fn get(self, slice: &Utf16Str) -> Option<&Self::Output> {
        if self.start <= slice.len() { Some(unsafe { self.get_unchecked(slice) }) } else { None }
    }

    #[inline]
    fn get_mut(self, slice: &mut Utf16Str) -> Option<&mut Self::Output> {
        if self.start <= slice.len() { Some(unsafe { self.get_unchecked_mut(slice) }) } else { None }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &Utf16Str) -> &Self::Output {
        let ptr = unsafe { slice.as_ptr().add(self.start) };
        let len = slice.len() - self.start;
        unsafe { Utf16Str::from_utf16_unchecked(std::slice::from_raw_parts(ptr, len)) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        let ptr = unsafe { slice.as_mut_ptr().add(self.start) };
        let len = slice.len() - self.start;
        unsafe { Utf16Str::from_utf16_unchecked_mut(std::slice::from_raw_parts_mut(ptr, len)) }
    }

    #[inline]
    fn index(self, slice: &Utf16Str) -> &Self::Output {
        self.get(slice).expect("slice index out of bounds")
    }

    #[inline]
    fn index_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        self.get_mut(slice).expect("slice index out of bounds")
    }
}

impl SliceIndex<Utf16Str> for RangeInclusive<usize> {
    type Output = Utf16Str;

    #[inline]
    fn get(self, slice: &Utf16Str) -> Option<&Self::Output> {
        if *self.end() == usize::MAX {
            None
        } else {
            (*self.start()..self.end() + 1).get(slice)
        }
    }

    #[inline]
    fn get_mut(self, slice: &mut Utf16Str) -> Option<&mut Self::Output> {
        if *self.end() == usize::MAX {
            None
        } else {
            (*self.start()..self.end() + 1).get_mut(slice)
        }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &Utf16Str) -> &Self::Output {
        unsafe { (*self.start()..self.end() + 1).get_unchecked(slice) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        unsafe { (*self.start()..self.end() + 1).get_unchecked_mut(slice) }
    }

    #[inline]
    fn index(self, slice: &Utf16Str) -> &Self::Output {
        assert!(*self.end() != usize::MAX, "index overflow");
        (*self.start()..self.end() + 1).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        assert!(*self.end() != usize::MAX, "index overflow");
        (*self.start()..self.end() + 1).index_mut(slice)
    }
}

impl SliceIndex<Utf16Str> for RangeToInclusive<usize> {
    type Output = Utf16Str;

    #[inline]
    fn get(self, slice: &Utf16Str) -> Option<&Self::Output> {
        if self.end == usize::MAX {
            None
        } else {
            (..self.end + 1).get(slice)
        }
    }

    #[inline]
    fn get_mut(self, slice: &mut Utf16Str) -> Option<&mut Self::Output> {
        if self.end == usize::MAX {
            None
        } else {
            (..self.end + 1).get_mut(slice)
        }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: &Utf16Str) -> &Self::Output {
        unsafe { (..self.end + 1).get_unchecked(slice) }
    }

    #[inline]
    unsafe fn get_unchecked_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        unsafe { (..self.end + 1).get_unchecked_mut(slice) }
    }

    #[inline]
    fn index(self, slice: &Utf16Str) -> &Self::Output {
        assert!(self.end != usize::MAX, "index overflow");
        (..self.end + 1).index(slice)
    }

    #[inline]
    fn index_mut(self, slice: &mut Utf16Str) -> &mut Self::Output {
        assert!(self.end != usize::MAX, "index overflow");
        (..self.end + 1).index_mut(slice)
    }
}

impl<I> Index<I> for Utf16Str
where
    I: SliceIndex<Utf16Str>
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}

impl<I> IndexMut<I> for Utf16Str
where
    I: SliceIndex<Utf16Str>
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        index.index_mut(self)
    }
}

impl<I> Index<I> for Utf16String
where
    I: SliceIndex<Utf16Str>
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}

impl<I> IndexMut<I> for Utf16String
where
    I: SliceIndex<Utf16Str>
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        index.index_mut(self)
    }
}