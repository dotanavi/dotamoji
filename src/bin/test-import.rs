extern crate bincode;
extern crate dotamoji;

use std::env;

use dotamoji::*;

fn count<D: Dictionary>(file: &str) -> usize {
    D::load_from_file(&file).count()
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args.next().expect("タイプが指定されていません。");
    let file = args.next().expect("ファイルが指定されていません。");

    let count = match dictype.as_str() {
        "array" => count::<DoubleArrayDict>(&file),
        "hash" => count::<RecHashDict>(&file),
        "trie" => count::<TrieDict>(&file),
        _ => panic!("不明なタイプです。"),
    };
    println!("count = {}", count);
}
