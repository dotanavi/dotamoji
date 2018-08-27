extern crate serde;
extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};
use serde::{Serialize, de::DeserializeOwned};

use dotamoji::*;
use dotamoji::util::load_from_file;

fn search_test<T: Dictionary<()> + Serialize + DeserializeOwned>(file: &str) {
    let pt: T = load_from_file(file);
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
    let _ = args.next().unwrap_or_else(|| panic!("実行ファイルが不明！？"));
    let dictype: String = args.next().unwrap_or_else(|| panic!("タイプが指定されていません。"));
    let file = args.next().unwrap_or_else(|| panic!("ファイルが指定されていません。"));

    match dictype.as_str() {
        "array" => search_test::<DoubleArray<()>>(&file),
        "hash" => search_test::<RecursiveHashMap<()>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
