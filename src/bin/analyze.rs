extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};

use dotamoji::*;

fn analyze<T: SerdeDic<Info>>(dic_file: &str, mat_file: &str) {
    let dic = T::load_from_file(dic_file);
    let mat = Matrix::load_from_file(mat_file);

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        dotamoji::analyze(&dic, &mat, line.trim());
    }
}


fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args.next().expect("タイプが指定されていません。");
    let dic_file = args.next().expect("辞書ファイルが指定されていません。");
    let mat_file = args.next().expect("コスト行列ファイルが指定されていません。");

    match dictype.as_str() {
        "array" => analyze::<DoubleArray<Info>>(&dic_file, &mat_file),
        "hash" => analyze::<RecursiveHashMap<Info>>(&dic_file, &mat_file),
        _ => panic!("不明なタイプです。"),
    }

}