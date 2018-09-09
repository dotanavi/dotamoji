extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};
use std::time::Instant;

use dotamoji::*;

#[inline]
fn analyze<K, D>(dic_file: &str, mat_file: &str)
where
    for<'a> &'a str: AsChars<K>,
    K: Copy + IntoString,
    D: LoadDict<K, Info>,
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
        let result = dotamoji::analyze(line.trim(), &dic, &mat);

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
        "array8" => analyze::<u8, DoubleArray<u8, Info>>(&dic_file, &mat_file),
        "array16" => analyze::<u16, DoubleArray<u16, Info>>(&dic_file, &mat_file),
        "array32" => analyze::<char, DoubleArray<char, Info>>(&dic_file, &mat_file),
        "hash8" => analyze::<u8, RecursiveHashMap<u8, Info>>(&dic_file, &mat_file),
        "hash16" => analyze::<u16, RecursiveHashMap<u16, Info>>(&dic_file, &mat_file),
        "hash32" => analyze::<char, RecursiveHashMap<char, Info>>(&dic_file, &mat_file),
        "trie8" => analyze::<u8, Trie<u8, Info>>(&dic_file, &mat_file),
        "trie16" => analyze::<u16, Trie<u16, Info>>(&dic_file, &mat_file),
        "trie32" => analyze::<char, Trie<char, Info>>(&dic_file, &mat_file),
        _ => panic!("不明なタイプです。"),
    }
}
