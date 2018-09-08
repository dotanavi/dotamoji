#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate fnv;
extern crate serde;

mod analyze;
mod as_utf16;
mod dictionary;
mod double_array;
mod double_array_b;
mod matrix;
mod prefix_map;
mod recursive_hash_map;
mod search_cache;
mod transform;
mod trie;
mod trie2;

pub use analyze::analyze;
pub use as_utf16::AsUtf16;
pub use dictionary::*;
pub use double_array::DoubleArray;
pub use double_array_b::DoubleArray as DoubleArrayB;
pub use matrix::Matrix;
pub use prefix_map::{AsChars, PrefixMap as PrefixMap2};
pub use recursive_hash_map::RecursiveHashMap;
pub use transform::Trans;
pub use trie::Trie;
pub use trie2::Trie as Trie2;

pub type DoubleArrayDict = DoubleArray<Info>;
pub type DoubleArrayDictB = DoubleArrayB<Info>;
pub type RecHashDict = RecursiveHashMap<Info>;
pub type TransDict = Trans<Info>;
pub type TrieDictA = Trie<trie::NodeA<Info>>;
pub type TrieDictB = Trie<trie::NodeB<Info>>;
