extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};

use dotamoji::*;

fn test_all<T: SerdeDic<Info>>(file: &str) {
    let dic = T::load_from_file(file);

    let stdin = io::stdin();
    let mut cnt = 0;
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        if let Some(_) = dic.get(word) {
            cnt += 1;
        } else {
            panic!("{} が見つかりません。", word);
        }
    }
    println!("{} 件のデータすべてが存在しました。", cnt);
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args.next().expect("タイプが指定されていません。");
    let file = args.next().expect("ファイルが指定されていません。");

    match dictype.as_str() {
        "array" => test_all::<DoubleArray<Info>>(&file),
        "hash" => test_all::<RecursiveHashMap<Info>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
