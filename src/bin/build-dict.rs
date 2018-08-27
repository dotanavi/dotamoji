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
    dotamoji::util::save_to_file(file, &dic);
    // let _: T = dotamoji::util::load_from_file(file);
    println!("{} を作成しました。", file);
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap_or_else(|| panic!("実行ファイルが不明！？"));
    let dictype: String = args.next().unwrap_or_else(|| panic!("タイプが指定されていません。"));
    let file = args.next().unwrap_or_else(|| panic!("ファイルが指定されていません。"));

    match dictype.as_str() {
        "array" => build::<DoubleArray<()>>(&file),
        "hash" => build::<RecursiveHashMap<()>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
