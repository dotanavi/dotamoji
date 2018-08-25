use std::io::{self, BufRead};

extern crate dotamoji;

#[allow(unused_imports)]
use dotamoji::double_array::DoubleArray;

#[allow(unused_imports)]
use dotamoji::staged_hash::StagedHash;

fn main() {
    let stdin = io::stdin();
    // let mut da = DoubleArray::new();
    let mut da = StagedHash::new();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        da.insert(word, ());
    }
    println!("len = {}", da.len());
}
