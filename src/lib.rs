#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

mod analyze;
mod as_utf16;
mod dictionary;
mod double_array;
mod matrix;
mod recursive_hash_map;
mod trie;
mod transform;

pub use analyze::analyze;
pub use as_utf16::AsUtf16;
pub use dictionary::*;
pub use double_array::DoubleArray;
pub use matrix::Matrix;
pub use recursive_hash_map::RecursiveHashMap;
pub use trie::Trie;
pub use transform::Trans;

pub type DoubleArrayDict = DoubleArray<Info>;
pub type RecHashDict = RecursiveHashMap<Info>;
pub type TransDict = Trans<Info>;
pub type TrieDict = Trie<Info>;

pub mod util {
    pub fn decode_utf16(chars: &[u16]) -> String {
        use std::char;

        char::decode_utf16(chars.iter().cloned())
            .filter_map(Result::ok)
            .collect()
    }
}
