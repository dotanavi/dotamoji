use fnv::FnvHashMap as HashMap;
// use std::collections::HashMap;
use as_chars::AsChars;
use prefix_map::PrefixMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize)]
pub struct RecursiveHashMap<K: Eq + Hash, V> {
    id: u32,
    link: HashMap<(u32, K), u32>,
    data: HashMap<u32, Vec<V>>,
}

impl<K: Eq + Hash, V> RecursiveHashMap<K, V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            id: 0,
            link: Default::default(),
            data: Default::default(),
        }
    }
}

impl<K: Eq + Hash, V> Default for RecursiveHashMap<K, V> {
    fn default() -> Self {
        RecursiveHashMap::new()
    }
}

impl<K: Eq + Hash, V> PrefixMap<K, V> for RecursiveHashMap<K, V> {
    #[inline]
    fn count(&self) -> usize {
        self.data.values().map(|v| v.len()).sum()
    }

    #[inline]
    fn get<T: AsChars<K>>(&self, key: T) -> Option<&[V]> {
        let mut current_id = 0;
        for ch in key.as_chars() {
            match self.link.get(&(current_id, ch)) {
                Some(next_id) => current_id = *next_id,
                None => return None,
            }
        }
        match self.data.get(&current_id) {
            Some(vec) if vec.len() > 0 => Some(&vec[..]),
            _ => None,
        }
    }

    #[inline]
    fn insert<T: AsChars<K>>(&mut self, key: T, value: V) {
        let id = &mut self.id;
        let link = &mut self.link;
        let data = &mut self.data;

        let mut current_id = 0;
        for ch in key.as_chars() {
            let entry = link.entry((current_id, ch));
            let next_id = entry.or_insert_with(|| {
                *id += 1;
                *id
            });
            current_id = *next_id;
        }
        let vec = data.entry(current_id).or_insert_with(Default::default);
        vec.push(value);
    }

    #[inline]
    fn each_prefix<T: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: T, mut f: F) {
        let mut current_id = 0;
        for (ix, ch) in key.as_chars().enumerate() {
            match self.link.get(&(current_id, ch)) {
                None => return,
                Some(next_id) => {
                    if let Some(vec) = self.data.get(next_id) {
                        f(ix + 1, &vec[..]);
                    }
                    current_id = *next_id;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type RecursiveHashMap<T> = super::RecursiveHashMap<u8, T>;

    #[test]
    // "未登録の要素を取り出そうとするとNoneを返す"
    fn test_not_registered() {
        let pt: RecursiveHashMap<()> = RecursiveHashMap::new();
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "配列の長さが足りない場合はNoneを返す"
    fn test_small() {
        let pt: RecursiveHashMap<()> = RecursiveHashMap::new();
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "途中までのキーが登録されている場合はNoneを返す"
    fn test_mid() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("ab", 1);
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "遷移は可能だがdataが登録されていない場合はNoneを返す"
    fn test_over() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("abcd", 1);
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "衝突しない要素の登録"
    fn test_no_conflict() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("abc", 1);
        pt.insert("ab", 2);
        assert_eq!(pt.get("abc"), Some(&[1][..]));
        assert_eq!(pt.get("ab"), Some(&[2][..]));
    }

    #[test]
    // "重複していない値の登録"
    fn test_dup_value() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("ab", 1);
        pt.insert("ab", 2);
        assert_eq!(pt.get("ab"), Some(&[1, 2][..]));
    }

    #[test]
    // "衝突する場合"
    fn test_conflict() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("abc", 1);
        pt.insert("ad", 2);
        pt.insert("ac", 3);

        assert_eq!(pt.get("abc"), Some(&[1][..]));
        assert_eq!(pt.get("ad"), Some(&[2][..]));
        assert_eq!(pt.get("ac"), Some(&[3][..]));
    }

    #[test]
    // "マルチバイト文字"
    fn test_multibyte() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("おはよう", 1);
        pt.insert("およごう", 2);

        assert_eq!(pt.get("おはよう"), Some(&[1][..]));
        assert_eq!(pt.get("およごう"), Some(&[2][..]));
    }

    #[test]
    // "遷移先ノードを正確に取得できているか"
    fn test_transite() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("ba", 1);
        pt.insert("bb", 2);

        assert_eq!(pt.get("ba"), Some(&[1][..]));
        assert_eq!(pt.get("bb"), Some(&[2][..]));
    }

    #[test]
    // "前方一致検索。"
    fn test_prefix() {
        let mut pt = RecursiveHashMap::new();
        pt.insert("abc", 1);
        pt.insert("ad", 2);
        pt.insert("ac", 3);
        pt.insert("a", 4);
        pt.insert("a", 5);

        let mut vec = vec![];
        pt.each_prefix("abcd", |len, data| {
            vec.push((len, data.to_owned()));
        });
        assert_eq!(vec, vec![(1, vec![4, 5]), (3, vec![1])]);
    }
}
