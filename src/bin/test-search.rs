extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};

use dotamoji::*;

fn search_test<T: SerdeDic<()>>(file: &str) {
    let pt = T::load_from_file(file);
    let mut cnt = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        pt.each_prefix(word, |_, v| cnt += v.len())
    }
    println!("全 {}", cnt);
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args.next().expect("タイプが指定されていません。");
    let file = args.next().expect("ファイルが指定されていません。");

    match dictype.as_str() {
        "array" => search_test::<DoubleArray<()>>(&file),
        "hash" => search_test::<RecursiveHashMap<()>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
