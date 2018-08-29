use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[inline]
fn read_int<T: FromStr>(str: &str) -> T {
    match str.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("{:?}を数値に変換できません", str),
    }
}

pub struct Matrix {
    pub width: u16,
    pub height: u16,
    pub cost: Vec<i16>,
}

impl Matrix {

    #[inline]
    pub fn get(&self, src_id: u16, dst_id: u16) -> i16 {
        self.cost[src_id as usize * self.height as usize + dst_id as usize]
    }

    pub fn load_from_file(file: &str) -> Matrix {
        let file = File::open(file).expect("ファイルが開けません");
        let file = BufReader::new(file);
        let mut lines = file.lines().filter_map(Result::ok);
        let (height, width) = {
            let line: String = lines.next().expect("ヘッダ行がありません");
            let mut row = line.split(" ");
            let h = read_int(row.next().expect("No height"));
            let w = read_int(row.next().expect("No width"));
            (h, w)
        };
        let mut cost = vec![0; width as usize * height as usize];
        for line in lines {
            let mut row = line.split(" ");
            let h: usize = read_int(row.next().expect("No height"));
            let w: usize = read_int(row.next().expect("No width"));
            let c = read_int(row.next().expect("No cost"));
            cost[h * height as usize + w] = c;
        }
        Matrix { width, height, cost }
    }
}
