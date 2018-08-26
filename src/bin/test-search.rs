extern crate bincode;
extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};
use std::fs::File;

use dotamoji::*;

// type Dic = DoubleArray<()>;
type Dic = RecursiveHashMap<()>;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        panic!("ファイルが指定されていません。");
    }
    let f = &args[1];
    let file = File::open(&f).unwrap_or_else(|_| panic!("ファイルが開けません"));
    let da: Dic = bincode::deserialize_from(file).unwrap_or_else(|_| panic!("辞書の復元に失敗しました。"));

    let stdin = io::stdin();
    let mut cnt = 0;
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        da.each_prefix(word, |_, v| cnt += v.len())
    }
    println!("全 {}", cnt);
}
