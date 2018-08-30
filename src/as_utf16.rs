use std::slice::Iter;
use std::iter::Cloned;
use std::str::EncodeUtf16;

pub trait AsUtf16 {
    type I: Iterator<Item = u16>;
    fn as_utf16(&self) -> Self::I;
}

impl <'a> AsUtf16 for &'a [u16] {
    type I = Cloned<Iter<'a, u16>>;

    #[inline]
    fn as_utf16(&self) -> Self::I { self.iter().cloned() }
}

impl <'a> AsUtf16 for &'a str {
    type I = EncodeUtf16<'a>;

    #[inline]
    fn as_utf16(&self) -> Self::I { self.encode_utf16() }
}
