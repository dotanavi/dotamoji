extern crate bincode;
extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};

use dotamoji::*;

// type Dic = DoubleArray<()>;
type Dic = RecursiveHashMap<()>;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        panic!("ファイルが指定されていません。");
    }
    let da: Dic = dotamoji::util::load_from_file(&args[1]);

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
