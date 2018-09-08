// use std::fmt::{Display, Debug};
use super::prefix_map::{AsChars, PrefixMap};

#[derive(Debug)]
pub struct Node<K, V> {
    pub data: Vec<V>,
    pub children: Vec<(K, Node<K, V>)>,
}
impl<K: Copy + Ord, V> Node<K, V> {
    #[inline]
    fn new() -> Self {
        Self {
            children: vec![],
            data: vec![],
        }
    }

    #[inline]
    fn search(&self, ch: K) -> Result<usize, usize> {
        self.children.binary_search_by_key(&ch, |&(c, _)| c)
    }

    #[inline]
    fn get<I: Iterator<Item = K>>(&self, key: I) -> Option<&[V]> {
        let mut cursor = self;
        for ch in key {
            match cursor.search(ch) {
                Ok(ix) => cursor = &cursor.children[ix].1,
                Err(_) => return None,
            }
        }
        if cursor.data.len() > 0 {
            Some(&cursor.data[..])
        } else {
            None
        }
    }

    #[inline]
    fn search_or_create<'a>(&'a mut self, ch: K) -> &'a mut Self {
        let index = match self.search(ch) {
            Ok(index) => index,
            Err(index) => {
                self.children.insert(index, (ch, Node::new()));
                index
            }
        };
        &mut self.children[index].1
    }

    #[inline]
    fn insert_rec<I: Iterator<Item = K>>(&mut self, mut iter: I, value: V) {
        if let Some(ch) = iter.next() {
            self.search_or_create(ch).insert_rec(iter, value);
        } else {
            self.data.push(value);
        }
    }

    #[inline]
    fn each_prefix<I: Iterator<Item = K>, F: FnMut(usize, &[V])>(&self, iter: I, mut f: F) {
        let mut cursor = self;
        for (chix, ch) in iter.enumerate() {
            match cursor.search(ch) {
                Ok(ix) => cursor = &cursor.children[ix].1,
                Err(_) => return,
            }
            if cursor.data.len() > 0 {
                f(chix + 1, &cursor.data[..]);
            }
        }
    }
}

pub struct Trie<K, V> {
    root: Node<K, V>,
}

impl<K: Copy + Ord, V> PrefixMap<K, V> for Trie<K, V> {
    #[inline]
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    #[inline]
    fn get<T: AsChars<K>>(&self, key: T) -> Option<&[V]> {
        self.root.get(key.as_chars())
    }

    #[inline]
    fn insert<T: AsChars<K>>(&mut self, key: T, value: V) {
        self.root.insert_rec(key.as_chars(), value);
    }

    #[inline]
    fn each_prefix<T: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: T, f: F) {
        self.root.each_prefix(key.as_chars(), f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Dic16<T> = Trie<u16, T>;
    type Dic8<T> = Trie<u8, T>;

    #[test]
    // "未登録の要素を取り出そうとするとNoneを返す"
    fn test_not_registered() {
        {
            let dic: Dic16<()> = Dic16::new();
            assert_eq!(dic.get("abc"), None);
        }
        {
            let dic: Dic8<()> = Dic8::new();
            assert_eq!(dic.get("abc"), None);
        }
    }

    #[test]
    // "配列の長さが足りない場合はNoneを返す"
    fn test_small() {
        {
            let dic: Dic16<()> = Dic16::new();
            assert_eq!(dic.get("abc"), None);
        }
        {
            let dic: Dic8<()> = Dic8::new();
            assert_eq!(dic.get("abc"), None);
        }
    }

    #[test]
    // "途中までのキーが登録されている場合はNoneを返す"
    fn test_mid() {
        {
            let mut dic = Dic16::new();
            dic.insert("ab", 1);
            assert_eq!(dic.get("abc"), None);
        }
        {
            let mut dic = Dic8::new();
            dic.insert("ab", 1);
            assert_eq!(dic.get("abc"), None);
        }
    }

    #[test]
    // "遷移は可能だがdataが登録されていない場合はNoneを返す"
    fn test_over() {
        {
            let mut dic = Dic16::new();
            dic.insert("abcd", 1);
            assert_eq!(dic.get("abc"), None);
        }
        {
            let mut dic = Dic8::new();
            dic.insert("abcd", 1);
            assert_eq!(dic.get("abc"), None);
        }
    }

    #[test]
    // "衝突しない要素の登録"
    fn test_no_conflict() {
        {
            let mut dic = Dic16::new();
            dic.insert("abc", 1);
            dic.insert("ab", 2);
            assert_eq!(dic.get("abc"), Some(&[1][..]));
            assert_eq!(dic.get("ab"), Some(&[2][..]));
        }
        {
            let mut dic = Dic8::new();
            dic.insert("abc", 1);
            dic.insert("ab", 2);
            assert_eq!(dic.get("abc"), Some(&[1][..]));
            assert_eq!(dic.get("ab"), Some(&[2][..]));
        }
    }

    #[test]
    // "重複していない値の登録"
    fn test_dup_value() {
        {
            let mut dic = Dic16::new();
            dic.insert("ab", 1);
            dic.insert("ab", 2);
            assert_eq!(dic.get("ab"), Some(&[1, 2][..]));
        }
        {
            let mut dic = Dic8::new();
            dic.insert("ab", 1);
            dic.insert("ab", 2);
            assert_eq!(dic.get("ab"), Some(&[1, 2][..]));
        }
    }

    #[test]
    // "衝突する場合"
    fn test_conflict() {
        {
            let mut dic = Dic16::new();
            dic.insert("abc", 1);
            dic.insert("ad", 2);
            dic.insert("ac", 3);

            assert_eq!(dic.get("abc"), Some(&[1][..]));
            assert_eq!(dic.get("ad"), Some(&[2][..]));
            assert_eq!(dic.get("ac"), Some(&[3][..]));
        }
        {
            let mut dic = Dic8::new();
            dic.insert("abc", 1);
            dic.insert("ad", 2);
            dic.insert("ac", 3);

            assert_eq!(dic.get("abc"), Some(&[1][..]));
            assert_eq!(dic.get("ad"), Some(&[2][..]));
            assert_eq!(dic.get("ac"), Some(&[3][..]));
        }
    }

    #[test]
    // "マルチバイト文字"
    fn test_multibyte() {
        {
            let mut dic = Dic16::new();
            dic.insert("おはよう", 1);
            dic.insert("およごう", 2);
            dic.insert("🍣", 3);
            dic.insert("🍺", 4);

            assert_eq!(dic.get("おはよう"), Some(&[1][..]));
            assert_eq!(dic.get("およごう"), Some(&[2][..]));
            assert_eq!(dic.get("🍣"), Some(&[3][..]));
            assert_eq!(dic.get("🍺"), Some(&[4][..]));
        }
        {
            let mut dic = Dic8::new();
            dic.insert("おはよう", 1);
            dic.insert("およごう", 2);
            dic.insert("🍣", 3);
            dic.insert("🍺", 4);

            assert_eq!(dic.get("おはよう"), Some(&[1][..]));
            assert_eq!(dic.get("およごう"), Some(&[2][..]));
            assert_eq!(dic.get("🍣"), Some(&[3][..]));
            assert_eq!(dic.get("🍺"), Some(&[4][..]));
        }
    }

    #[test]
    // "遷移先ノードを正確に取得できているか"
    fn test_transite() {
        {
            let mut dic = Dic16::new();
            dic.insert("ba", 1);
            dic.insert("bb", 2);

            assert_eq!(dic.get("ba"), Some(&[1][..]));
            assert_eq!(dic.get("bb"), Some(&[2][..]));
        }
        {
            let mut dic = Dic8::new();
            dic.insert("ba", 1);
            dic.insert("bb", 2);

            assert_eq!(dic.get("ba"), Some(&[1][..]));
            assert_eq!(dic.get("bb"), Some(&[2][..]));
        }
    }

    #[test]
    // "前方一致検索。"
    fn test_prefix() {
        {
            let mut dic = Dic16::new();
            dic.insert("abc", 1);
            dic.insert("ad", 2);
            dic.insert("ac", 3);
            dic.insert("a", 4);
            dic.insert("a", 5);

            let mut vec = vec![];
            dic.each_prefix("abcd", |len, data| {
                vec.push((len, data.to_owned()));
            });
            assert_eq!(vec, vec![(1, vec![4, 5]), (3, vec![1])]);
        }
        {
            let mut dic = Dic8::new();
            dic.insert("abc", 1);
            dic.insert("ad", 2);
            dic.insert("ac", 3);
            dic.insert("a", 4);
            dic.insert("a", 5);

            let mut vec = vec![];
            dic.each_prefix("abcd", |len, data| {
                vec.push((len, data.to_owned()));
            });
            assert_eq!(vec, vec![(1, vec![4, 5]), (3, vec![1])]);
        }
    }
}
