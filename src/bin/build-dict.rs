extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};
use std::str::FromStr;

use dotamoji::*;

fn read_int<T: FromStr>(str: &str) -> T {
    match str.parse() {
        Ok(x) => x,
        Err(_) => panic!("{:?}を数値に変換できません", str),
    }
}

fn build<D: Dictionary>(file: &str) {
    let stdin = io::stdin();
    let mut dic = D::new();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let mut row = line.split(",");
        let word = row.next().expect("文字が取得できません");
        let left = read_int(row.next().expect("LeftIDが取得できません"));
        let right = read_int(row.next().expect("RightIDが取得できません"));
        let cost = read_int(row.next().expect("コストが取得できません"));
        dic.insert(word, Info::new(left, right, cost));
    }
    dic.save_to_file(file);
    println!("{} を作成しました。", file);
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
        "array" | "array_a" => build::<DoubleArrayDict>(&file),
        "array_b" => build::<DoubleArrayDictB>(&file),
        "hash" => build::<RecHashDict>(&file),
        "trie" | "trie_a" => build::<TrieDictA>(&file),
        "trie_b" => build::<TrieDictB>(&file),
        "trans" => build::<TransDict>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
