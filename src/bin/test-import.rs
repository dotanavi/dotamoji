extern crate bincode;
extern crate dotamoji;

use std::env;

use dotamoji::*;

fn len<T: SerdeDic<Info>>(file: &str) -> usize {
    T::load_from_file(&file).len()
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args.next().expect("タイプが指定されていません。");
    let file = args.next().expect("ファイルが指定されていません。");

    let len = match dictype.as_str() {
        "array" => len::<DoubleArray<Info>>(&file),
        "hash" => len::<RecursiveHashMap<Info>>(&file),
        _ => panic!("不明なタイプです。"),
    };
    println!("len = {}", len);
}
