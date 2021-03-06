use crate::as_chars::AsUsize;
use crate::double_array::DoubleArray;
#[allow(unused_imports)]
use crate::prefix_map::PrefixMap;
use crate::search_cache::*;
use crate::transform_map::Transform;
use crate::transform_map::TransformMap;
use crate::trie::{Node, Trie};
use std::mem::swap;

pub type Trie2DAMap<K, V> = TransformMap<Trie<K, V>, DoubleArray<K, V, NoCache>, Trie2DoubleArray>;

pub enum Trie2DoubleArray {}

impl<K: AsUsize, V> Transform<Trie<K, V>, DoubleArray<K, V, NoCache>> for Trie2DoubleArray {
    fn transform(trie: Trie<K, V>) -> DoubleArray<K, V, NoCache> {
        transform(trie)
    }
}

pub fn transform<K: AsUsize, V>(trie: Trie<K, V>) -> DoubleArray<K, V, NoCache> {
    // show_stats(&trie.root);

    let mut base = vec![0, 0];
    let mut check = vec![0, 0];
    let mut data = vec![vec![], vec![]];

    // let mut cache = NoCache::new(2);
    // let mut cache = BoolCache::new(2);
    // let mut cache = LinkCache::new(2);
    // let mut cache = BitCache0::new(2);
    let mut cache = BitCache1::new(2);
    // let mut cache = DoubleCheck::<BitCache0, BitCache1>::new(2);

    put_rec(trie.root, 1, &mut base, &mut check, &mut data, &mut cache);
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
            index = cache.find_empty(ch + index, check) - ch;
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
    type Trie8<V> = super::super::trie::Trie<u8, V>;

    macro_rules! combine_cache {
        ($c:ty, $($cs:tt)* ) => (DoubleCheck<$c, combine_cache!($($cs)*)>);
        ($c:ty) => ($c);
    }
    type TestSearchCache = combine_cache!(BitCache0, BitCache1, BoolCache, LinkCache, NoCache);

    #[test]
    fn test_cache_size() {
        use std::mem::size_of;

        assert_eq!(
            size_of::<TestSearchCache>(),
            (0 + size_of::<BitCache0>()
                + size_of::<BitCache1>()
                + size_of::<BoolCache>()
                + size_of::<LinkCache>()
                + size_of::<NoCache>())
        );
    }

    pub fn test_transform<K: AsUsize, V>(trie: Trie<K, V>) -> DoubleArray<K, V, NoCache> {
        let mut base = vec![0, 0];
        let mut check = vec![0, 0];
        let mut data = vec![vec![], vec![]];
        let mut cache = TestSearchCache::new(2);

        put_rec(trie.root, 1, &mut base, &mut check, &mut data, &mut cache);
        return DoubleArray::from_raw_parts(base, check, data);
    }

    #[test]
    fn test_single() {
        let mut trie = Trie8::new();
        trie.insert("a", 1);
        trie.insert("b", 2);
        trie.insert("c", 3);
        let count = trie.count();
        let ary = test_transform(trie);
        assert_eq!(count, ary.count());
        assert_eq!(&[1], ary.get("a").unwrap());
        assert_eq!(&[2], ary.get("b").unwrap());
        assert_eq!(&[3], ary.get("c").unwrap());
        assert_eq!(None, ary.get("d"));
    }

    #[test]
    fn test_multi() {
        let mut trie = Trie8::new();
        trie.insert("a", 1);
        trie.insert("b", 2);
        trie.insert("b", 3);
        trie.insert("c", 4);
        trie.insert("c", 5);
        trie.insert("c", 6);
        let count = trie.count();
        let ary = test_transform(trie);
        assert_eq!(count, ary.count());
        assert_eq!(&[1], ary.get("a").unwrap());
        assert_eq!(&[2, 3], ary.get("b").unwrap());
        assert_eq!(&[4, 5, 6], ary.get("c").unwrap());
        assert_eq!(None, ary.get("d"));
    }

    #[test]
    fn test_nest() {
        let mut trie = Trie8::new();
        trie.insert("a", 1);
        trie.insert("aa", 2);
        trie.insert("aaa", 3);
        let count = trie.count();
        let ary = test_transform(trie);
        assert_eq!(count, ary.count());
        assert_eq!(&[1], ary.get("a").unwrap());
        assert_eq!(&[2], ary.get("aa").unwrap());
        assert_eq!(&[3], ary.get("aaa").unwrap());
        assert_eq!(None, ary.get("aaaa"));
    }

    #[test]
    fn test_multi_nest() {
        let mut trie = Trie8::new();
        trie.insert("a", 1);
        trie.insert("aa", 2);
        trie.insert("ab", 3);
        trie.insert("b", 4);
        trie.insert("ba", 5);
        trie.insert("bb", 6);
        let count = trie.count();
        let ary = test_transform(trie);
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
