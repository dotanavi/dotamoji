#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate fnv;
extern crate serde;

mod analyze;
mod analyze2;
mod as_chars;
mod dictionary;
mod double_array;
mod matrix;
mod prefix_map;
mod recursive_hash_map;
mod search_cache;
mod transform;
mod trie;
mod trie2;

pub use analyze::analyze;
pub use analyze2::analyze as analyze2;
pub use as_chars::{AsChars, IntoString};
pub use dictionary::*;
pub use double_array::DoubleArray;
pub use matrix::Matrix;
pub use prefix_map::PrefixMap;
pub use recursive_hash_map::RecursiveHashMap;
pub use transform::Trans;
pub use trie::Trie;
pub use trie2::Trie as Trie2;

pub type DoubleArrayDict = DoubleArray<Info>;
pub type RecHashDict = RecursiveHashMap<Info>;
pub type TrieDictA = Trie<trie::NodeA<Info>>;
pub type TrieDictB = Trie<trie::NodeB<Info>>;
