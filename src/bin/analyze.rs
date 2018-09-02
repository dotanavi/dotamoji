extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};
use std::time::Instant;

use dotamoji::*;

fn analyze<D: Dictionary>(dic_file: &str, mat_file: &str) {
    let start = Instant::now();
    let dic = D::load_from_file(dic_file);
    eprintln!("load_dic: {:?}", start.elapsed());

    let start = Instant::now();
    let mat = Matrix::load_from_file(mat_file);
    eprintln!("load_mat: {:?}", start.elapsed());

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        dotamoji::analyze(&dic, &mat, line.trim());
        println!();
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args
        .next()
        .expect("タイプが指定されていません。");
    let dic_file = args
        .next()
        .expect("辞書ファイルが指定されていません。");
    let mat_file = args
        .next()
        .expect("コスト行列ファイルが指定されていません。");

    match dictype.as_str() {
        "array" => analyze::<DoubleArrayDict>(&dic_file, &mat_file),
        "hash" => analyze::<RecHashDict>(&dic_file, &mat_file),
        "trie" | "trie_a" => analyze::<TrieDictA>(&dic_file, &mat_file),
        "trie_b" => analyze::<TrieDictB>(&dic_file, &mat_file),
        _ => panic!("不明なタイプです。"),
    }
}
