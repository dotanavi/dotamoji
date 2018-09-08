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

fn test_all_2<K, D>(file: &str)
where
    for<'a> &'a str: AsChars<K>,
    D: NewDictionary<K>,
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
        "array" => test_all::<DoubleArrayDict>(&file),
        "array16" => test_all_2::<u16, DoubleArrayDict>(&file),
        "hash" => test_all_2::<u16, RecHashDict>(&file),
        "trie8" => test_all_2::<u8, Trie<u8, Info>>(&file),
        "trie16" => test_all_2::<u16, Trie<u16, Info>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
