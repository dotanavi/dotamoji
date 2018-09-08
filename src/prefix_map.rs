use std::iter::Cloned;
use std::slice::Iter;
use std::str::EncodeUtf16;

pub trait PrefixMap<K, V> {
    fn new() -> Self;
    fn get<T: AsChars<K>>(&self, key: T) -> Option<&[V]>;
    fn insert<T: AsChars<K>>(&mut self, key: T, value: V);
    fn each_prefix<T: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: T, f: F);
}

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
