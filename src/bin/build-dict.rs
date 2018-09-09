extern crate dotamoji;

use dotamoji::*;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, BufRead};
use std::str::FromStr;

#[inline]
fn read_int<T: FromStr>(str: &str) -> T {
    match str.parse() {
        Ok(x) => x,
        Err(_) => panic!("{:?}を数値に変換できません", str),
    }
}

#[inline]
fn line_to_info<'a>(line: &'a str) -> (&'a str, Info) {
    let mut row = line.split(",");
    let word = row.next().expect("文字が取得できません");
    let left = read_int(row.next().expect("LeftIDが取得できません"));
    let right = read_int(row.next().expect("RightIDが取得できません"));
    let cost = read_int(row.next().expect("コストが取得できません"));
    (word, Info::new(left, right, cost))
}

#[inline]
fn build<K, D>(file_path: &str)
where
    for<'a> &'a str: AsChars<K>,
    D: SaveDict<K, Info> + Default,
{
    let stdin = io::stdin();
    let mut dic = D::default();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let (word, info) = line_to_info(&line);
        dic.insert(word, info);
    }
    if file_path == "-" {
        let stdout = io::stdout();
        let handle = stdout.lock();
        dic.save_to_file(handle);
    } else {
        let file = File::create(file_path).expect("ファイルを作成できません。");
        let file = BufWriter::new(file);
        dic.save_to_file(file);
        println!("{} を作成しました。", file_path);
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("実行ファイルが不明！？");
    let dictype = args
        .next()
        .expect("タイプが指定されていません。");
    let file = args
        .next()
        .expect("ファイルが指定されていません。");

    match dictype.as_str() {
        "array8" => build::<u8, DoubleArray<u8, Info>>(&file),
        "array16" => build::<u16, DoubleArray<u16, Info>>(&file),
        "hash8" => build::<u8, RecursiveHashMap<u8, Info>>(&file),
        "hash16" => build::<u16, RecursiveHashMap<u16, Info>>(&file),
        "hash32" => build::<char, RecursiveHashMap<char, Info>>(&file),
        "trie8" => build::<u8, Trie<u8, Info>>(&file),
        "trie16" => build::<u16, Trie<u16, Info>>(&file),
        "trie32" => build::<char, Trie<char, Info>>(&file),
        "trans8" => build::<u8, Trans<u8, Info>>(&file),
        "trans16" => build::<u16, Trans<u16, Info>>(&file),
        "trans32" => build::<char, Trans<char, Info>>(&file),
        _ => panic!("不明なタイプです。"),
    }
}
