#[macro_use]
extern crate serde_derive;

mod analyze;
mod as_chars;
mod dictionary;
mod double_array;
mod fast_build_double_array;
mod info;
mod matrix;
mod prefix_map;
mod recursive_hash_map;
mod search_cache;
mod transform_map;
mod trie;
mod trie_to_double_array;

pub use crate::analyze::analyze;
pub use crate::as_chars::{AsChars, IntoString};
pub use crate::dictionary::{LoadDict, SaveDict};
pub use crate::fast_build_double_array::FastBuildDoubleArray;
pub use crate::info::Info;
pub use crate::matrix::Matrix;
pub use crate::prefix_map::PrefixMap;
pub use crate::recursive_hash_map::RecursiveHashMap;
pub use crate::trie::Trie;
pub use crate::trie_to_double_array::Trie2DAMap;

pub type DoubleArray<K, V> = double_array::DoubleArray<K, V, search_cache::NoCache>;
