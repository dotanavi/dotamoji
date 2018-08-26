#[macro_use]
extern crate serde_derive;
extern crate serde;

mod double_array;
mod recursive_hash_map;

pub use double_array::DoubleArray;
pub use recursive_hash_map::RecursiveHashMap;

pub trait PrefixTree<T> {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn get(&self, key: &str) -> Option<&[T]>;
    fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, f: F);
    fn insert(&mut self, key: &str, value: T);
}

pub mod util {
    pub fn decode_utf16(chars: &[u16]) -> String {
        use std::char;

        char::decode_utf16(chars.iter().cloned())
            .filter_map(Result::ok)
            .collect()
    }
}
