extern crate dotamoji;

use std::io::{self, BufRead};
use dotamoji::*;

fn main() {
    let stdin = io::stdin();
    // let mut da = DoubleArray::new();
    let mut da = RecursiveHashMap::new();
    for line in stdin.lock().lines().filter_map(Result::ok) {
        let word = line.split(",").next().unwrap();
        da.insert(word, ());
    }
    println!("len = {}", da.len());
}
