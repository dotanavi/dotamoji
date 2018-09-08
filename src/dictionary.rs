use std::fs::File;
use std::io::{BufReader, BufWriter};

use bincode;
use prefix_map::PrefixMap;
use serde::{de::DeserializeOwned, Serialize};
use info::Info;

pub trait NewDictionary<K>: PrefixMap<K, Info> + Default {
    fn load_from_file(file: &str) -> Self;
    fn save_to_file(self, file: &str) -> Self;
}

impl<K, D> NewDictionary<K> for D
where
    D: PrefixMap<K, Info> + Default + Serialize + DeserializeOwned,
{
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
