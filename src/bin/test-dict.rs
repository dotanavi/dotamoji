extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};

use dotamoji::*;

fn test_all<D: Dictionary>(file: &str) {
    let dic = D::load_from_file(file);

    let stdin = io::stdin();
    let mut cnt = 0;
    for (ix, line) in stdin.lock().lines().filter_map(Result::ok).enumerate() {
        let word = line.split(",").next().unwrap();
        if let Some(_) = dic.get(word) {
            cnt += 1;
        } else {
            panic!("{} が見つかりません。({}行目)", word, ix + 1);
        }
    }
    println!("{} 件のデータすべてが存在しました。", cnt);
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args
        .next()
        .expect("タイプが指定されていません。");
    let file = args
        .next()
        .expect("ファイルが指定されていません。");

    match dictype.as_str() {
        "array" => test_all::<DoubleArrayDict>(&file),
        "hash" => test_all::<RecHashDict>(&file),
        "trie" | "trie_a" => test_all::<TrieDictA>(&file),
        "trie_b" => test_all::<TrieDictB>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
