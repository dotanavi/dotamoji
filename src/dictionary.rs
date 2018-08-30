use std::fs::File;
use std::io::{BufReader, BufWriter};

use bincode;
use serde::{Serialize, de::DeserializeOwned};

use super::AsUtf16;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Info {
    pub left_id: u16,
    pub right_id: u16,
    pub cost: i16,
}

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

pub trait Dictionary: PrefixMap<Info> {
    fn load_from_file(file: &str) -> Self;

    fn save_to_file(self, file: &str) -> Self;
}
impl <D> Dictionary for D where D: PrefixMap<Info> + Serialize + DeserializeOwned {
    fn load_from_file(file: &str) -> Self {
        let file = File::open(file).expect("ファイルが開けません");
        let file = BufReader::new(file);
        bincode::deserialize_from(file).expect("辞書の復元に失敗しました。")
    }

    fn save_to_file(self, file: &str) -> Self {
        let file = File::create(file).expect("ファイルを作成できません。");
        let file = BufWriter::new(file);
        bincode::serialize_into(file, &self).expect("保存に失敗しました。");
        self
    }
}

