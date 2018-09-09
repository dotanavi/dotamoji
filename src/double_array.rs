use as_chars::{AsChars, AsUsize};
use prefix_map::PrefixMap;
use search_cache::{NoCache, SearchCache};
use std::cmp::{max, min};
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Eq, PartialEq)]
enum Index {
    Zero,
    Transit,
    Empty,
    Conflict,
    OutOfRange,
}

#[derive(Serialize, Deserialize)]
pub struct DoubleArray<K, V, C> {
    base: Vec<u32>,
    check: Vec<u32>,
    data: Vec<Vec<V>>,
    phantom: PhantomData<K>,
    search_cache: C,
}

impl<K, V, C: SearchCache> DoubleArray<K, V, C> {
    #[inline]
    pub fn new() -> Self {
        DoubleArray {
            base: vec![0, 0],
            check: vec![0, 0],
            data: vec![vec![], vec![]],
            phantom: PhantomData,
            search_cache: C::new(2),
        }
    }
}

impl<K: AsUsize, V> DoubleArray<K, V, NoCache> {
    #[inline]
    pub fn from_raw_parts(base: Vec<u32>, check: Vec<u32>, data: Vec<Vec<V>>) -> Self {
        Self {
            base,
            check,
            data,
            phantom: PhantomData,
            search_cache: NoCache,
        }
    }
}

impl<K: AsUsize, V, C: SearchCache> DoubleArray<K, V, C> {
    #[inline]
    pub fn count(&self) -> usize {
        self.data.iter().map(|v| v.len()).sum()
    }

    pub fn get<I: AsChars<K>>(&self, key: I) -> Option<&[V]> {
        let mut current_ix = 1;
        for ch in key.as_chars() {
            if let (Index::Transit, next_ix) = self.next_index(current_ix, ch) {
                current_ix = next_ix;
            } else {
                return None;
            }
        }
        let vec: &Vec<_> = &self.data[current_ix];
        if vec.len() > 0 {
            Some(&vec[..])
        } else {
            None
        }
    }

    #[inline]
    fn each_prefix<I: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: I, mut f: F) {
        let mut current_ix = 1;
        for (ix, ch) in key.as_chars().enumerate() {
            if let (Index::Transit, next_ix) = self.next_index(current_ix, ch) {
                current_ix = next_ix;
                if let Some(v) = self.data.get(current_ix) {
                    if v.len() > 0 {
                        f(ix + 1, &v[..]);
                    }
                }
            } else {
                return;
            }
        }
    }

    #[inline]
    fn next_index(&self, current_index: usize, ch: K) -> (Index, usize) {
        let current_base = self.base[current_index];
        if current_base == 0 {
            return (Index::Zero, 0);
        }
        let next_ix = current_base as usize + ch.as_usize();
        if next_ix < self.check.len() {
            let check_ix = self.check[next_ix] as usize;
            if check_ix == current_index {
                (Index::Transit, next_ix)
            } else if check_ix == 0 {
                (Index::Empty, next_ix)
            } else {
                (Index::Conflict, next_ix)
            }
        } else {
            (Index::OutOfRange, next_ix)
        }
    }

    pub fn insert<I: AsChars<K>>(&mut self, key: I, value: V) {
        let mut current_ix = 1;
        for ch in key.as_chars() {
            let (state, next_ix) = self.next_index(current_ix, ch);
            current_ix = match state {
                Index::Transit => next_ix,
                Index::Empty => self.update(current_ix, next_ix),
                Index::Zero => {
                    let new_next_ix = self.put_first_one(current_ix, ch);
                    self.update(current_ix, new_next_ix)
                }
                Index::Conflict => {
                    let new_next_ix = self.rebase(current_ix, ch);
                    self.update(current_ix, new_next_ix)
                }
                Index::OutOfRange => {
                    self.extend(next_ix + 1);
                    self.update(current_ix, next_ix)
                }
            };
        }
        // データを登録
        self.data[current_ix].push(value);
    }

    #[inline]
    fn update(&mut self, current_ix: usize, next_ix: usize) -> usize {
        self.base[next_ix] = 0;
        self.check[next_ix] = current_ix as u32;
        next_ix
    }

    #[inline]
    fn put_first_one(&mut self, current_ix: usize, ch: K) -> usize {
        let position = self.find_new_base_one(ch);
        self.base[current_ix] = position as u32 - ch.as_usize() as u32;
        return position;
    }

    #[inline]
    fn find_new_base_one(&mut self, ch: K) -> usize {
        for i in ch.as_usize() + 1..self.check.len() {
            if self.check[i] == 0 {
                return i;
            }
        }
        let pos = max(self.check.len(), ch.as_usize() + 1);
        self.extend(pos + 1);
        return pos;
    }

    fn rebase(&mut self, current_ix: usize, ch: K) -> usize {
        let current_base = self.base[current_ix] as usize;
        debug_assert!(current_base > 0);
        // 1. currIdx から遷移しているすべてのノード(遷移先ノード)を取得 (index, char)
        let mut next_nodes = vec![];
        for i in current_base..min(self.check.len(), current_base + K::MAX) {
            if self.check[i] as usize == current_ix {
                next_nodes.push(K::from_usize(i - current_base));
            }
        }
        debug_assert!(next_nodes.len() > 0);
        // 2. 遷移先ノードと currChar が遷移可能なbaseを求める
        let new_base = self.find_new_base(&next_nodes, ch);
        self.base[current_ix] = new_base as u32;
        for ch in next_nodes {
            let src_ix = current_base + ch.as_usize();
            let dst_ix = new_base as usize + ch.as_usize();

            // 3. 遷移先ノードを新しい base で計算した index にコピー
            debug_assert!(self.base[dst_ix] == 0);
            debug_assert!(self.check[dst_ix] == 0);
            debug_assert!(self.data[dst_ix].len() == 0);
            let src_base = self.base[src_ix] as usize;
            self.base[dst_ix] = self.base[src_ix];
            self.check[dst_ix] = self.check[src_ix];
            self.data.swap(src_ix, dst_ix);

            if src_base > 0 {
                // 4. 旧遷移先ノードから更に遷移しているノードの check を新遷移先ノードの index で更新
                let src_ix = src_ix as u32;
                let dst_ix = dst_ix as u32;
                let range = src_base..min(self.check.len(), src_base + K::MAX);
                for mut c in &mut self.check[range] {
                    if *c == src_ix {
                        *c = dst_ix
                    }
                }
            }
            // 5. 旧遷移先ノードの base, check, data をリセット
            self.base[src_ix] = 0;
            self.check[src_ix] = 0;
        }
        new_base as usize + ch.as_usize()
    }

    fn find_new_base(&mut self, next_nodes: &[K], ch: K) -> usize {
        debug_assert!(next_nodes.len() > 0);

        let ch = ch.as_usize();
        let mut new_base = 0;
        'out: loop {
            new_base += 1;
            let mut ix = new_base + ch;
            while ix < self.check.len() && self.check[ix] != 0 {
                ix += 1;
            }
            new_base = ix - ch;

            for ch in next_nodes {
                let new_ix = new_base + ch.as_usize();
                if new_ix < self.check.len() && self.check[new_ix] != 0 {
                    continue 'out;
                }
            }
            // next_nodes は昇順のため最後の要素が最大である。
            let last_ix = max(ix, new_base + next_nodes.last().unwrap().as_usize());
            if last_ix >= self.check.len() {
                self.extend(last_ix + 1);
            }
            return new_base;
        }
    }

    fn extend(&mut self, size: usize) {
        debug_assert!(self.base.len() < size);

        self.base.resize(size, 0);
        self.check.resize(size, 0);
        let n = self.data.len();
        self.data.extend((n..size).map(|_| vec![]));

        debug_assert!(self.base.len() == size);
        debug_assert!(self.check.len() == size);
        debug_assert!(self.data.len() == size);
    }
}

impl<K, V: Debug, C> DoubleArray<K, V, C> {
    pub fn show_debug(&self) {
        use std::char::from_u32;

        for i in 0..self.check.len() {
            if self.base[i] == 0 {
                continue;
            }
            let ch = from_u32(i as u32 - self.base[self.check[i] as usize] as u32);
            println!(
                "{}\t, {}\t, {}\t, {}\t, {:?}",
                i,
                self.base[i],
                self.check[i],
                ch.unwrap(),
                self.data[i]
            );
        }
    }
}

impl<K, V, C: SearchCache> Default for DoubleArray<K, V, C> {
    #[inline]
    fn default() -> Self {
        DoubleArray::new()
    }
}

impl<K: AsUsize, V, C: SearchCache> PrefixMap<K, V> for DoubleArray<K, V, C> {
    #[inline]
    fn count(&self) -> usize {
        self.count()
    }

    #[inline]
    fn get<T: AsChars<K>>(&self, key: T) -> Option<&[V]> {
        self.get(key)
    }

    #[inline]
    fn insert<T: AsChars<K>>(&mut self, key: T, value: V) {
        self.insert(key, value)
    }

    #[inline]
    fn each_prefix<T: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: T, f: F) {
        self.each_prefix(key, f)
    }
}

#[cfg(test)]
mod tests {
    use search_cache::NoCache;

    type DoubleArray<T> = super::DoubleArray<u8, T, NoCache>;

    #[test]
    // "未登録の要素を取り出そうとするとNoneを返す"
    fn test_not_registered() {
        let pt: DoubleArray<()> = DoubleArray::new();
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "配列の長さが足りない場合はNoneを返す"
    fn test_small() {
        let pt: DoubleArray<()> = DoubleArray::new();
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "途中までのキーが登録されている場合はNoneを返す"
    fn test_mid() {
        let mut pt = DoubleArray::new();
        pt.insert("ab", 1);
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "遷移は可能だがdataが登録されていない場合はNoneを返す"
    fn test_over() {
        let mut pt = DoubleArray::new();
        pt.insert("abcd", 1);
        assert_eq!(pt.get("abc"), None);
    }

    #[test]
    // "衝突しない要素の登録"
    fn test_no_conflict() {
        let mut pt = DoubleArray::new();
        pt.insert("abc", 1);
        pt.insert("ab", 2);
        assert_eq!(pt.get("abc"), Some(&[1][..]));
        assert_eq!(pt.get("ab"), Some(&[2][..]));
    }

    #[test]
    // "重複していない値の登録"
    fn test_dup_value() {
        let mut pt = DoubleArray::new();
        pt.insert("ab", 1);
        pt.insert("ab", 2);
        assert_eq!(pt.get("ab"), Some(&[1, 2][..]));
    }

    #[test]
    // "衝突する場合"
    fn test_conflict() {
        let mut pt = DoubleArray::new();
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
        let mut pt = DoubleArray::new();
        pt.insert("おはよう", 1);
        pt.insert("およごう", 2);

        assert_eq!(pt.get("おはよう"), Some(&[1][..]));
        assert_eq!(pt.get("およごう"), Some(&[2][..]));
    }

    #[test]
    // "遷移先ノードを正確に取得できているか"
    fn test_transite() {
        let mut pt = DoubleArray::new();
        pt.insert("ba", 1);
        pt.insert("bb", 2);

        assert_eq!(pt.get("ba"), Some(&[1][..]));
        assert_eq!(pt.get("bb"), Some(&[2][..]));
    }

    #[test]
    // "前方一致検索。"
    fn test_prefix() {
        let mut pt = DoubleArray::new();
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
