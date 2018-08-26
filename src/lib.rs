#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

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
    use super::*;
    use std::io::{BufReader, BufWriter};
    use std::fs::File;
    use serde::{Serialize, de::DeserializeOwned};

    pub fn decode_utf16(chars: &[u16]) -> String {
        use std::char;

        char::decode_utf16(chars.iter().cloned())
            .filter_map(Result::ok)
            .collect()
    }

    pub fn load_from_file<T: DeserializeOwned>(file: &str) -> T {
        let file = File::open(file).unwrap_or_else(|_| panic!("ファイルが開けません"));
        let file = BufReader::new(file);
        bincode::deserialize_from(file).unwrap_or_else(|_| panic!("辞書の復元に失敗しました。"))
    }

    pub fn save_to_file<T: Serialize>(file: &str, dic: &T) {
        let file = File::create(file).unwrap_or_else(|_| panic!("ファイルを作成できません。"));
        let file = BufWriter::new(file);
        bincode::serialize_into(file, &dic).unwrap_or_else(|_| panic!("保存に失敗しました。"));
    }
}
