extern crate bincode;
extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};
use std::fs::File;

use dotamoji::*;

type Dic = DoubleArray<()>;

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
        if let Some(_) = da.get(word) {
            cnt += 1;
        } else {
            panic!("{} が見つかりません。", word);
        }
    }
    println!("{} 件のデータすべてが存在しました。", cnt);
}
