use as_chars::AsChars;
use as_chars::AsUsize;
use bincode;
use dictionary::NewDictionary;
use double_array::DoubleArray;
use info::Info;
use prefix_map::PrefixMap;
use search_cache::*;
use std::fs::File;
use std::io::BufWriter;
use std::mem::swap;
use std::time::Instant;
use trie::{Node, Trie};

pub enum Trans<K, V> {
    Array(Box<DoubleArray<K, V>>),
    Trie(Box<Trie<K, V>>),
}

impl<K, V> Default for Trans<K, V> {
    #[inline]
    fn default() -> Self {
        Trans::Trie(Box::new(Trie::new()))
    }
}

impl<K: AsUsize + Ord, V> PrefixMap<K, V> for Trans<K, V> {
    #[inline]
    fn count(&self) -> usize {
        match self {
            Trans::Array(ref x) => x.count(),
            Trans::Trie(ref x) => x.count(),
        }
    }

    #[inline]
    fn get<T: AsChars<K>>(&self, key: T) -> Option<&[V]> {
        match self {
            Trans::Array(ref x) => x.get(key),
            Trans::Trie(ref x) => x.get(key),
        }
    }

    #[inline]
    fn insert<T: AsChars<K>>(&mut self, key: T, value: V) {
        match self {
            Trans::Array(ref mut x) => x.insert(key, value),
            Trans::Trie(ref mut x) => x.insert(key, value),
        }
    }

    #[inline]
    fn each_prefix<T: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: T, f: F) {
        use std::ops::Deref;

        match self {
            Trans::Array(ref x) => PrefixMap::each_prefix(x.deref(), key, f),
            Trans::Trie(ref x) => x.each_prefix(key, f),
        }
    }
}

impl<K: AsUsize + Ord> NewDictionary<K> for Trans<K, Info> {
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

pub fn transform<K: AsUsize, V>(trie: Trie<K, V>) -> DoubleArray<K, V> {
    // show_stats(&trie.root);

    let mut base = vec![0, 0];
    let mut check = vec![0, 0];
    let mut data = vec![vec![], vec![]];

    // let mut cache = NoCache::new(2);
    // let mut cache = BoolCache::new(2);
    // let mut cache = BitCache::new(2);
    let mut cache = BitCache0::new(2);
    // let mut cache = LinkCache::new(2);
    // let mut cache = DoubleCheck::<BitCache, BitCache0>::new(2);

    let start = Instant::now();
    put_rec(trie.root, 1, &mut base, &mut check, &mut data, &mut cache);
    eprintln!("transform: {:?}", start.elapsed());
    return DoubleArray::from_raw_parts(base, check, data);
}

fn put_rec<K: AsUsize, V, C: SearchCache>(
    mut node: Node<K, V>,
    base_index: usize,
    base: &mut Vec<u32>,
    check: &mut Vec<u32>,
    data: &mut Vec<Vec<V>>,
    cache: &mut C,
) {
    if node.data.len() > 0 {
        swap(&mut node.data, &mut data[base_index]);
    }
    if node.children.len() == 0 {
        return;
    }

    let new_base = {
        let ch = node.children[0].0.as_usize();

        let mut index = 0;
        'outer: loop {
            index = cache.find_empty(ch, index, check);
            for &(ch, _) in &node.children[1..] {
                if cache.is_filled(index + ch.as_usize(), check) {
                    continue 'outer;
                }
            }
            break index;
        }
    };
    base[base_index] = new_base as u32;

    let requred_size = new_base + node.children.last().unwrap().0.as_usize() + 1;
    if requred_size > base.len() {
        base.resize(requred_size, 0);
        check.resize(requred_size, 0);
        let n = data.len();
        data.extend((n..requred_size).map(|_| vec![]));
        cache.extend(requred_size);
    }
    for &(ch, _) in &node.children {
        let index = new_base + ch.as_usize();
        cache.mark(index);
        check[index] = base_index as u32;
    }
    for (ch, child_node) in node.children {
        put_rec(
            child_node,
            new_base + ch.as_usize(),
            base,
            check,
            data,
            cache,
        );
    }
}

#[allow(unused)]
fn show_stats<K, V>(node: &Node<K, V>) {
    let mut table = vec![];
    calc_stats_rec(node, &mut table);

    for (ix, cnt) in table.iter().enumerate() {
        if *cnt > 0 {
            println!("{:>5}:{:>5}", ix, cnt);
        }
    }
}

fn calc_stats_rec<K, V>(node: &Node<K, V>, table: &mut Vec<u8>) {
    let len = node.children.len();
    if len >= table.len() {
        table.resize(len + 1, 0);
    }
    table[len] += 1;
    for (_, node) in &node.children {
        calc_stats_rec(&node, table);
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
