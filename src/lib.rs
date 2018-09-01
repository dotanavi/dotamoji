#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;
extern crate fnv;

mod analyze;
mod as_utf16;
mod dictionary;
mod double_array;
mod matrix;
mod recursive_hash_map;
mod trie;
mod transform;
mod search_cache;

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
