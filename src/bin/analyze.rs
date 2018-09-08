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
        let start = Instant::now();
        let result = dotamoji::analyze(line.trim(), &dic, &mat);

        if let Ok(analyzed) = result {
            eprintln!("analyze: {:?}", start.elapsed());

            println!("cost = {}", analyzed.cost);
            for token in analyzed.iter() {
                let word = String::from_utf16_lossy(token.word);
                println!("id:{:>5} | cost:{:>6} | {}", token.id, token.cost, word);
            }
        } else {
            println!("形態素解析に失敗しました。");
        }
        println!();
    }
}

fn analyze_2<K, D>(dic_file: &str, mat_file: &str)
where
    for<'a> &'a str: AsChars<K>,
    K: Copy + IntoString,
    D: NewDictionary<K>,
{
    let start = Instant::now();
    let dic = D::load_from_file(dic_file);
    eprintln!("load_dic: {:?}", start.elapsed());

    let start = Instant::now();
    let mat = Matrix::load_from_file(mat_file);
    eprintln!("load_mat: {:?}", start.elapsed());

    let stdin = io::stdin();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let start = Instant::now();
        let result = dotamoji::analyze2(line.trim(), &dic, &mat);

        if let Ok(analyzed) = result {
            eprintln!("analyze: {:?}", start.elapsed());

            println!("cost = {}", analyzed.cost);
            for token in analyzed.iter() {
                let word = IntoString::into_string(token.word);
                println!("id:{:>5} | cost:{:>6} | {}", token.id, token.cost, word);
            }
        } else {
            println!("形態素解析に失敗しました。");
        }
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
        "array_b" => analyze::<DoubleArrayDict>(&dic_file, &mat_file),
        "hash" => analyze_2::<u16, RecHashDict>(&dic_file, &mat_file),
        "trie" | "trie_a" => analyze::<TrieDictA>(&dic_file, &mat_file),
        "trie_b" => analyze::<TrieDictB>(&dic_file, &mat_file),
        "trie8" => analyze_2::<u8, Trie2<u8, Info>>(&dic_file, &mat_file),
        "trie16" => analyze_2::<u16, Trie2<u16, Info>>(&dic_file, &mat_file),
        _ => panic!("不明なタイプです。"),
    }
}
