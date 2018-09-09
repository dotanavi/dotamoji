#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate fnv;
extern crate serde;

mod analyze;
mod as_chars;
mod dictionary;
mod double_array;
mod info;
mod matrix;
mod prefix_map;
mod recursive_hash_map;
mod search_cache;
mod transform;
mod trie;

pub use analyze::analyze;
pub use as_chars::{AsChars, IntoString};
pub use dictionary::NewDictionary;
pub use info::Info;
pub use matrix::Matrix;
pub use prefix_map::PrefixMap;
pub use recursive_hash_map::RecursiveHashMap;
pub use transform::Trans;
pub use trie::Trie;

pub type DoubleArray<K, V> = double_array::DoubleArray<K, V, search_cache::NoCache>;
