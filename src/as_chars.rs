use std::iter::Cloned;
use std::slice::Iter;
use std::str::{self, EncodeUtf16};

pub trait AsChars<T> {
    type I: Iterator<Item = T>;
    fn as_chars(&self) -> Self::I;
}

impl<'a> AsChars<u16> for &'a str {
    type I = EncodeUtf16<'a>;

    #[inline]
    fn as_chars(&self) -> Self::I {
        self.encode_utf16()
    }
}

impl<'a> AsChars<u8> for &'a str {
    type I = Cloned<Iter<'a, u8>>;

    #[inline]
    fn as_chars(&self) -> Self::I {
        self.as_bytes().iter().cloned()
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
    fn into_string(chars: &[u8]) -> String {
        String::from_utf8_lossy(chars).into_owned()
    }
}

impl IntoString for u16 {
    fn into_string(chars: &[u16]) -> String {
        String::from_utf16_lossy(chars)
    }
}
