use std::iter::Cloned;
use std::iter::FromIterator;
use std::slice::Iter;
use std::str::{self, Chars, EncodeUtf16};

pub trait AsChars<T> {
    type I: Iterator<Item = T>;
    fn as_chars(&self) -> Self::I;
}

impl<'a> AsChars<u8> for &'a str {
    type I = Cloned<Iter<'a, u8>>;

    #[inline]
    fn as_chars(&self) -> Self::I {
        self.as_bytes().iter().cloned()
    }
}

impl<'a> AsChars<u16> for &'a str {
    type I = EncodeUtf16<'a>;

    #[inline]
    fn as_chars(&self) -> Self::I {
        self.encode_utf16()
    }
}

impl<'a> AsChars<char> for &'a str {
    type I = Chars<'a>;

    #[inline]
    fn as_chars(&self) -> Self::I {
        self.chars()
    }
}

impl<'a, T: Copy> AsChars<T> for &'a [T] {
    type I = Cloned<Iter<'a, T>>;

    #[inline]
    fn as_chars(&self) -> Self::I {
        self.iter().cloned()
    }
}

// -----------------------------------------------------------------------------

pub trait IntoString: Sized {
    fn into_string(chars: &[Self]) -> String;
}

impl IntoString for u8 {
    #[inline]
    fn into_string(chars: &[u8]) -> String {
        String::from_utf8_lossy(chars).into_owned()
    }
}

impl IntoString for u16 {
    #[inline]
    fn into_string(chars: &[u16]) -> String {
        String::from_utf16_lossy(chars)
    }
}

impl IntoString for char {
    #[inline]
    fn into_string(chars: &[char]) -> String {
        String::from_iter(chars.iter())
    }
}

// -----------------------------------------------------------------------------

pub trait AsUsize: Copy {
    const MAX: usize;

    fn as_usize(self) -> usize;
    fn from_usize(n: usize) -> Self;
}

impl AsUsize for u8 {
    const MAX: usize = <u8>::max_value() as usize;

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_usize(n: usize) -> u8 {
        n as u8
    }
}

impl AsUsize for u16 {
    const MAX: usize = <u16>::max_value() as usize;

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_usize(n: usize) -> u16 {
        n as u16
    }
}

impl AsUsize for char {
    const MAX: usize = <u32>::max_value() as usize;

    #[inline]
    fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    fn from_usize(n: usize) -> char {
        use std::char::from_u32_unchecked;
        unsafe { from_u32_unchecked(n as u32) }
    }
}
