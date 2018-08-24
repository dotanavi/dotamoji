use std::io::{self, BufRead};

extern crate dotamoji;

use dotamoji::double_array::DoubleArray;

fn main() {
    let stdin = io::stdin();
    let mut da = DoubleArray::new();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        da.insert(word, ());
    }
    println!("len = {}", da.len());
}
