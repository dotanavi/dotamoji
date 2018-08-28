use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_int<T: FromStr>(str: &str) -> T {
    match str.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("{:?}を数値に変換できません", str),
    }
}

pub fn from_file(file: &str) -> (u16, u16, Vec<i16>) {
    let file = File::open(file).expect("ファイルが開けません");
    let file = BufReader::new(file);
    let mut lines = file.lines().filter_map(Result::ok);
    let (height, width) = {
        let line: String = lines.next().expect("ヘッダ行がありません");
        let mut row = line.split(",");
        let h = read_int(row.next().expect("No height"));
        let w = read_int(row.next().expect("No width"));
        (h, w)
    };
    let mut vec = vec![0; width as usize * height as usize];
    for line in lines {
        let mut row = line.split(",");
        let h: usize = read_int(row.next().expect("No height"));
        let w: usize = read_int(row.next().expect("No width"));
        let c = read_int(row.next().expect("No cost"));
        vec[h * height as usize + w] = c;
    }
    return (height, width, vec);
}
