extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};

use dotamoji::*;

fn build<T: SerdeDic<()>>(file: &str) {
    let stdin = io::stdin();
    let mut dic = T::new();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        dic.insert(word, ());
    }
    dic.save_to_file(file);
    println!("{} を作成しました。", file);
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args.next().expect("タイプが指定されていません。");
    let file = args.next().expect("ファイルが指定されていません。");

    match dictype.as_str() {
        "array" => build::<DoubleArray<()>>(&file),
        "hash" => build::<RecursiveHashMap<()>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
