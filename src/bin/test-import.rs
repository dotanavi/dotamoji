extern crate bincode;
extern crate dotamoji;

use std::env;

use dotamoji::*;
use dotamoji::util::load_from_file;

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap_or_else(|| panic!("実行ファイルが不明！？"));
    let dictype: String = args.next().unwrap_or_else(|| panic!("タイプが指定されていません。"));
    let file = args.next().unwrap_or_else(|| panic!("ファイルが指定されていません。"));

    let len = match dictype.as_str() {
        "array" => load_from_file::<DoubleArray<()>>(&file).len(),
        "hash" => load_from_file::<RecursiveHashMap<()>>(&file).len(),
        _ => panic!("不明なタイプです。"),
    };
    println!("len = {}", len);
}
