extern crate bincode;
extern crate dotamoji;

use std::env;
use std::io::{self, BufRead};
use std::fs::File;

use dotamoji::*;

type Dic = DoubleArray<()>;
// type Dic = StagedHash<()>;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        panic!("ファイルが指定されていません。");
    }
    let f = &args[1];

    let stdin = io::stdin();
    let mut dic = Dic::new();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        dic.insert(word, ());
    }

    {
        let file = File::create(&f).unwrap_or_else(|_| panic!("ファイルを作成できません。"));
        bincode::serialize_into(file, &dic).unwrap_or_else(|_| panic!("保存に失敗しました。"));
    }
    {
        let file = File::open(&f).unwrap_or_else(|_| panic!("ファイルが開けません"));
        let _: Dic = bincode::deserialize_from(file).unwrap_or_else(|_| panic!("辞書の復元に失敗しました。"));
    }
    println!("{} を作成しました。", f);
}
