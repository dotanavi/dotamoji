use std::fs::File;
use std::io::{BufWriter};
use std::mem::swap;

use bincode;

use super::AsUtf16;
use dictionary::{PrefixMap, Dictionary, Info};
use double_array::DoubleArray;
use trie::{Node, Trie};

pub enum Trans<T> {
    Array(Box<DoubleArray<T>>),
    Trie(Box<Trie<T>>),
}

impl<T> PrefixMap<T> for Trans<T> {
    #[inline]
    fn new() -> Self {
        Trans::Trie(Box::new(Trie::new()))
    }

    #[inline]
    fn count(&self) -> usize {
        match self {
            Trans::Array(ref x) => x.count(),
            Trans::Trie(ref x) => x.count(),
        }
    }

    #[inline]
    fn get(&self, key: impl AsUtf16) -> Option<&[T]> {
        match self {
            Trans::Array(ref x) => x.get(key),
            Trans::Trie(ref x) => x.get(key),
        }
    }

    #[inline]
    fn insert(&mut self, key: impl AsUtf16, value: T) {
        match self {
            Trans::Array(ref mut x) => x.insert(key, value),
            Trans::Trie(ref mut x) => x.insert(key, value),
        }
    }

    #[inline]
    fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, f: F) {
        match self {
            Trans::Array(ref x) => x.each_prefix(key, f),
            Trans::Trie(ref x) => x.each_prefix(key, f),
        }
    }

    #[inline]
    fn each_prefix16<F: FnMut(usize, &[T])>(&self, key: &[u16], f: F) {
        match self {
            Trans::Array(ref x) => x.each_prefix16(key, f),
            Trans::Trie(ref x) => x.each_prefix16(key, f),
        }
    }
}

impl Dictionary for Trans<Info> {
    fn load_from_file(_: &str) -> Self {
        panic!("ファイルのロードには対応していません。");
    }

    fn save_to_file(self, file: &str) -> Self {
        let file = File::create(file).expect("ファイルを作成できません。");
        let file = BufWriter::new(file);

        let array = match self {
            Trans::Array(x) => x,
            Trans::Trie(x) => transform(*x).into(),
        };
        bincode::serialize_into(file, &array).expect("保存に失敗しました。");
        Trans::Array(array)
    }
}

pub fn transform<T>(trie: Trie<T>) -> DoubleArray<T> {
    // let before_count = trie.count();
    let mut base = vec![0, 0];
    let mut check = vec![0, 0];
    let mut data = vec![vec![], vec![]];

    put_rec(trie.root, 1, &mut base, &mut check, &mut data);
    let da = DoubleArray::from_raw_parts(base, check, data);

    // let after_count = da.count();
    // if before_count != after_count {
    //     panic!("正しく変換できていません。 before = {}, after = {}", before_count, after_count);
    // }
    return da;
}

#[inline]
fn find_base_one(ch: usize, search_start: usize, check: &[u32]) -> usize {
    let mut ix = ch + search_start + 1;
    while ix < check.len() && check[ix] != 0 {
        ix += 1;
    }
    return ix - ch;
}

#[inline]
fn find_base<T>(check: &[u32], children: &[(u16, T)]) -> usize {
    let ch = children[0].0 as usize;

    let mut index = 0;
    'outer: loop {
        index = find_base_one(ch, index, check);
        for &(ch, _) in &children[1..] {
            let ix = index + ch as usize;
            if ix < check.len() && check[ix] != 0 {
                continue 'outer;
            }
        }
        return index;
    }
}

#[inline]
fn extend<T: Default>(vec: &mut Vec<T>, new_size: usize) {
    let len = vec.len();
    debug_assert!(len < new_size);
    vec.extend((len .. new_size).map(|_| Default::default()));
}

fn put_rec<T>(mut node: Node<T>, base_index: usize, base: &mut Vec<u32>, check: &mut Vec<u32>, data: &mut Vec<Vec<T>>) {
    if node.data.len() > 0 {
        swap(&mut node.data, &mut data[base_index]);
    }
    if node.children.len() == 0 {
        return;
    }

    let new_base = find_base(&check[..], &node.children);
    base[base_index] = new_base as u32;

    let requred_size = new_base + node.children.last().expect("childは存在する").0 as usize + 1;
    if requred_size > base.len() {
        extend(base, requred_size);
        extend(check, requred_size);
        extend(data, requred_size);
    }
    for &(ch, _) in &node.children {
        check[new_base + ch as usize] = base_index as u32;
    }
    for (ch, child_node) in node.children {
        put_rec(child_node, new_base + ch as usize, base, check, data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let mut trie = Trie::new();
        trie.insert("a", 1);
        trie.insert("b", 2);
        trie.insert("c", 3);
        let count = trie.count();
        let ary = transform(trie);
        assert_eq!(count, ary.count());
        assert_eq!(&[1], ary.get("a").unwrap());
        assert_eq!(&[2], ary.get("b").unwrap());
        assert_eq!(&[3], ary.get("c").unwrap());
        assert_eq!(None, ary.get("d"));
    }

    #[test]
    fn test_multi() {
        let mut trie = Trie::new();
        trie.insert("a", 1);
        trie.insert("b", 2);
        trie.insert("b", 3);
        trie.insert("c", 4);
        trie.insert("c", 5);
        trie.insert("c", 6);
        let count = trie.count();
        let ary = transform(trie);
        assert_eq!(count, ary.count());
        assert_eq!(&[1], ary.get("a").unwrap());
        assert_eq!(&[2, 3], ary.get("b").unwrap());
        assert_eq!(&[4, 5, 6], ary.get("c").unwrap());
        assert_eq!(None, ary.get("d"));
    }

    #[test]
    fn test_nest() {
        let mut trie = Trie::new();
        trie.insert("a", 1);
        trie.insert("aa", 2);
        trie.insert("aaa", 3);
        let count = trie.count();
        let ary = transform(trie);
        assert_eq!(count, ary.count());
        assert_eq!(&[1], ary.get("a").unwrap());
        assert_eq!(&[2], ary.get("aa").unwrap());
        assert_eq!(&[3], ary.get("aaa").unwrap());
        assert_eq!(None, ary.get("aaaa"));
    }

    #[test]
    fn test_multi_nest() {
        let mut trie = Trie::new();
        trie.insert("a", 1);
        trie.insert("aa", 2);
        trie.insert("ab", 3);
        trie.insert("b", 4);
        trie.insert("ba", 5);
        trie.insert("bb", 6);
        let count = trie.count();
        let ary = transform(trie);
        assert_eq!(count, ary.count());
        assert_eq!(&[1], ary.get("a").unwrap());
        assert_eq!(&[2], ary.get("aa").unwrap());
        assert_eq!(&[3], ary.get("ab").unwrap());
        assert_eq!(&[4], ary.get("b").unwrap());
        assert_eq!(&[5], ary.get("ba").unwrap());
        assert_eq!(&[6], ary.get("bb").unwrap());
        assert_eq!(None, ary.get("aaaa"));
    }
}
