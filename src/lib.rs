#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde::{Serialize, de::DeserializeOwned};

mod double_array;
mod recursive_hash_map;
mod trie;
mod matrix;
mod analyze;

pub use double_array::DoubleArray;
pub use recursive_hash_map::RecursiveHashMap;
pub use trie::Trie;
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
    fn count(&self) -> usize;
    fn get(&self, key: impl AsUtf16) -> Option<&[T]>;
    fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, f: F);
    fn each_prefix16<F: FnMut(usize, &[T])>(&self, key: &[u16], f: F);
    fn insert(&mut self, key: impl AsUtf16, value: T);
}

pub trait AsUtf16 {
    type I: Iterator<Item = u16>;
    fn as_utf16(&self) -> Self::I;
}

use std::slice::Iter;
use std::iter::Cloned;
use std::str::EncodeUtf16;

impl <'a> AsUtf16 for &'a [u16] {
    type I = Cloned<Iter<'a, u16>>;

    #[inline]
    fn as_utf16(&self) -> Self::I { self.iter().cloned() }
}

impl <'a> AsUtf16 for &'a str {
    type I = EncodeUtf16<'a>;

    #[inline]
    fn as_utf16(&self) -> Self::I { self.encode_utf16() }
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
pub type TrieDict = Trie<Info>;

pub mod util {
    pub fn decode_utf16(chars: &[u16]) -> String {
        use std::char;

        char::decode_utf16(chars.iter().cloned())
            .filter_map(Result::ok)
            .collect()
    }
}
