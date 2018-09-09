extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};

use dotamoji::*;

#[inline]
fn test_all<K, D>(file: &str)
where
    for<'a> &'a str: AsChars<K>,
    D: LoadDict<K, Info>,
{
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
        "array8" => test_all::<u8, DoubleArray<u8, Info>>(&file),
        "array16" => test_all::<u16, DoubleArray<u16, Info>>(&file),
        "array32" => test_all::<char, DoubleArray<char, Info>>(&file),
        "hash8" => test_all::<u8, RecursiveHashMap<u8, Info>>(&file),
        "hash16" => test_all::<u16, RecursiveHashMap<u16, Info>>(&file),
        "hash32" => test_all::<char, RecursiveHashMap<char, Info>>(&file),
        "trie8" => test_all::<u8, Trie<u8, Info>>(&file),
        "trie16" => test_all::<u16, Trie<u16, Info>>(&file),
        "trie32" => test_all::<char, Trie<char, Info>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
