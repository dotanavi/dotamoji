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
mod transform_map;
mod trie;

pub use analyze::analyze;
pub use as_chars::{AsChars, IntoString};
pub use dictionary::{LoadDict, SaveDict};
pub use info::Info;
pub use matrix::Matrix;
pub use prefix_map::PrefixMap;
pub use recursive_hash_map::RecursiveHashMap;
// pub use transform::Trans;
pub use transform::Trie2DoubleArray;
pub use trie::Trie;
pub use transform_map::TransformMap;

pub type DoubleArray<K, V> = double_array::DoubleArray<K, V, search_cache::NoCache>;
pub type Trans<K, V> = transform_map::TransformMap<Trie<K, V>, DoubleArray<K, V>, transform::Trie2DoubleArray>;
