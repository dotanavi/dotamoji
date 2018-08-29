#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde::{Serialize, de::DeserializeOwned};

mod double_array;
mod recursive_hash_map;
mod matrix;
mod analyze;

pub use double_array::DoubleArray;
pub use recursive_hash_map::RecursiveHashMap;
pub use matrix::Matrix;
pub use analyze::analyze;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Info { left_id: u16, right_id: u16, cost: i16 }

impl Info {
    pub fn new(left_id: u16, right_id: u16, cost: i16) -> Self {
        Info { left_id, right_id, cost }
    }
}

pub trait PrefixMap<T> {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn get(&self, key: &str) -> Option<&[T]>;
    fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, f: F);
    fn each_prefix16<F: FnMut(usize, &[T])>(&self, key: &[u16], f: F);
    fn insert(&mut self, key: &str, value: T);
}

pub trait Dictionary: PrefixMap<Info> + Serialize + DeserializeOwned {
    fn load_from_file(file: &str) -> Self {
        let file = File::open(file).expect("ファイルが開けません");
        let file = BufReader::new(file);
        bincode::deserialize_from(file).expect("辞書の復元に失敗しました。")
    }

    fn save_to_file(&self, file: &str) {
        let file = File::create(file).expect("ファイルを作成できません。");
        let file = BufWriter::new(file);
        bincode::serialize_into(file, self).expect("保存に失敗しました。");
    }
}
impl <D> Dictionary for D where D: PrefixMap<Info> + Serialize + DeserializeOwned {}

pub type DoubleArrayDict = DoubleArray<Info>;
pub type RecHashDict = RecursiveHashMap<Info>;

pub mod util {
    pub fn decode_utf16(chars: &[u16]) -> String {
        use std::char;

        char::decode_utf16(chars.iter().cloned())
            .filter_map(Result::ok)
            .collect()
    }
}
