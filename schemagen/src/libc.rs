use core::ops::{Index, RangeFrom, RangeTo};

mod sys {
    use core::ffi::{c_int, c_size_t, c_void};

    unsafe extern "C" {
        #[cfg(not(windows))]
        pub fn memmem(
            haystack: *const c_void,
            hsize: c_size_t,
            needle: *const c_void,
            nsize: c_size_t,
        ) -> *mut c_void;
        pub fn memchr(s: *const c_void, c: c_int, n: c_size_t) -> *mut c_void;
        pub fn memrchr(s: *const c_void, c: c_int, n: c_size_t) -> *mut c_void;
    }
}

#[cfg(not(windows))]
#[must_use]
pub fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let ptr = unsafe {
        sys::memmem(
            haystack.as_ptr().cast(),
            haystack.len(),
            needle.as_ptr().cast(),
            needle.len(),
        )
    };
    (!ptr.is_null()).then(|| (ptr as usize) - (haystack.as_ptr() as usize))
}

#[cfg(windows)]
pub fn memmem(mut haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let mut offset = 0;
    while let Some(pos) = memchr(haystack, needle[0]) {
        offset += pos;
        haystack = &haystack[pos..];
        if haystack.starts_with(needle) {
            return Some(offset);
        }
        offset += 1;
        haystack = &haystack[1..];
    }
    None
}

#[must_use]
pub fn memrmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(haystack.len());
    }

    let mut limit = haystack.len();
    while let Some(pos) = memrchr(&haystack[..limit], needle[0]) {
        if haystack[pos..].starts_with(needle) {
            return Some(pos);
        }
        limit = pos;
    }
    None
}

pub fn memchr(haystack: &[u8], c: impl Into<core::ffi::c_int>) -> Option<usize> {
    let ptr = unsafe { sys::memchr(haystack.as_ptr().cast(), c.into(), haystack.len()) };
    (!ptr.is_null()).then(|| (ptr as usize) - (haystack.as_ptr() as usize))
}

pub fn memrchr(haystack: &[u8], c: impl Into<core::ffi::c_int>) -> Option<usize> {
    let ptr = unsafe { sys::memrchr(haystack.as_ptr().cast(), c.into(), haystack.len()) };
    (!ptr.is_null()).then(|| (ptr as usize) - (haystack.as_ptr() as usize))
}

pub trait Needle: Copy {
    fn find(self, haystack: &[u8]) -> Option<usize>;
    fn len(self) -> usize;
    fn is_empty(self) -> bool {
        self.len() == 0
    }
}

impl Needle for u8 {
    fn find(self, haystack: &[u8]) -> Option<usize> {
        memchr(haystack, self)
    }

    fn len(self) -> usize {
        1
    }
}

impl Needle for &[u8] {
    fn find(self, haystack: &[u8]) -> Option<usize> {
        memmem(haystack, self)
    }

    fn len(self) -> usize {
        <[u8]>::len(self)
    }
}

impl<const N: usize> Needle for &[u8; N] {
    fn find(self, haystack: &[u8]) -> Option<usize> {
        <&[u8]>::find(self, haystack)
    }

    fn len(self) -> usize {
        <&[u8]>::len(self)
    }
}

type IndexOutput<T, I> = <T as Index<I>>::Output;

type UntilOutput<'a, T> = (
    &'a IndexOutput<T, RangeTo<usize>>,
    &'a IndexOutput<T, RangeFrom<usize>>,
);
type SplitOutput<'a, T> = (
    &'a IndexOutput<T, RangeTo<usize>>,
    &'a IndexOutput<T, RangeFrom<usize>>,
);
type BetweenOutput<'a, T> = (
    &'a IndexOutput<IndexOutput<T, RangeFrom<usize>>, RangeTo<usize>>,
    &'a IndexOutput<IndexOutput<T, RangeFrom<usize>>, RangeFrom<usize>>,
);

pub trait MemchrExt {
    fn find_needle(&self, needle: impl Needle) -> Option<usize>;

    fn find_after(&self, needle: impl Needle) -> Option<&IndexOutput<Self, RangeFrom<usize>>>
    where
        Self: Index<RangeFrom<usize>>,
    {
        let pos = self.find_needle(needle)?;
        Some(&self[pos + needle.len()..])
    }

    fn find_until(&self, needle: impl Needle) -> Option<UntilOutput<'_, Self>>
    where
        Self: Index<RangeTo<usize>> + Index<RangeFrom<usize>>,
    {
        let pos = self.find_needle(needle)?;
        Some((&self[..pos], &self[pos..]))
    }

    fn split_exclusive(&self, needle: impl Needle) -> Option<SplitOutput<'_, Self>>
    where
        Self: Index<RangeTo<usize>> + Index<RangeFrom<usize>>,
    {
        let pos = self.find_needle(needle)?;
        Some((&self[..pos], &self[pos + needle.len()..]))
    }

    fn find_between(&self, start: impl Needle, end: impl Needle) -> Option<BetweenOutput<'_, Self>>
    where
        Self: Index<RangeFrom<usize>> + Index<RangeTo<usize>>,
        <Self as Index<RangeFrom<usize>>>::Output:
            Index<RangeTo<usize>> + Index<RangeFrom<usize>> + MemchrExt,
    {
        self.find_after(start)?.split_exclusive(end)
    }
}

impl MemchrExt for [u8] {
    fn find_needle(&self, needle: impl Needle) -> Option<usize> {
        needle.find(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memmem_empty_needle() {
        let hay = b"abcdef";
        let needle: &[u8] = b"";
        assert_eq!(memmem(hay, needle), Some(0));
    }

    #[test]
    fn test_memmem_found_positions() {
        let hay = b"hello world";
        assert_eq!(memmem(hay, b"hello"), Some(0));
        assert_eq!(memmem(hay, b"world"), Some(6));
        assert_eq!(memmem(hay, b"o w"), Some(4));
    }

    #[test]
    fn test_memmem_not_found() {
        let hay = b"abc";
        assert_eq!(memmem(hay, b"z"), None);
    }

    #[test]
    fn test_memmem_multiple_and_overlap() {
        let hay = b"ababab";
        assert_eq!(memmem(hay, b"bab"), Some(1));

        let hay2 = b"aaaaa";
        assert_eq!(memmem(hay2, b"aaa"), Some(0));
    }

    #[test]
    fn test_repeating() {
        let hay = b"abbab";
        assert_eq!(memmem(hay, b"bab"), Some(2));
    }

    #[test]
    fn test_memchr_found_and_not_found() {
        let hay = b"abcdef";
        assert_eq!(memchr(hay, b'c'), Some(2));
        // 'z' not present
        assert_eq!(memchr(hay, b'z'), None);
    }

    #[test]
    fn test_memchr_zero_byte() {
        let hay = b"a\0b";
        assert_eq!(memchr(hay, 0), Some(1));
    }

    #[test]
    fn test_memrmem_empty_needle() {
        let hay = b"abcdef";
        let needle: &[u8] = b"";
        assert_eq!(memrmem(hay, needle), Some(6));
    }

    #[test]
    fn test_memrmem_found_positions() {
        let hay = b"hello world";
        assert_eq!(memrmem(hay, b"hello"), Some(0));
        assert_eq!(memrmem(hay, b"world"), Some(6));
        assert_eq!(memrmem(hay, b"o w"), Some(4));
    }

    #[test]
    fn test_memrmem_not_found() {
        let hay = b"abc";
        assert_eq!(memrmem(hay, b"z"), None);
    }

    #[test]
    fn test_memrmem_multiple_and_overlap() {
        let hay = b"ababab";
        assert_eq!(memrmem(hay, b"bab"), Some(3));

        let hay2 = b"aaaaa";
        assert_eq!(memrmem(hay2, b"aaa"), Some(2));
    }
}
