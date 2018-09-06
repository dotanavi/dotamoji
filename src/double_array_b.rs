use super::{AsUtf16, PrefixMap};
use std::{
    char,
    cmp::{max, min},
    fmt::Debug,
    u16,
};

#[derive(Eq, PartialEq)]
enum Index {
    Zero,
    Transit,
    Empty,
    Conflict,
    OutOfRange,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Double {
    base: u32,
    check: u32,
}

impl Double {
    fn new() -> Double {
        Double { base: 0, check: 0 }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DoubleArray<T> {
    array: Vec<Double>,
    data: Vec<Vec<T>>,
}

impl<T> DoubleArray<T> {
    #[inline]
    pub fn new() -> Self {
        DoubleArray {
            array: vec![Double::new(), Double::new()],
            data: vec![vec![], vec![]],
        }
    }

    #[inline]
    pub fn from_raw_parts(array: Vec<Double>, data: Vec<Vec<T>>) -> Self {
        Self { array, data }
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.data.iter().map(|v| v.len()).sum()
    }

    pub fn get(&self, key: impl AsUtf16) -> Option<&[T]> {
        let mut current_ix = 1;
        for ch in key.as_utf16() {
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
    pub fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, mut f: F) {
        let mut chars: Vec<u16> = vec![];
        let mut current_ix = 1;
        for ch in key.encode_utf16() {
            chars.push(ch);
            if let (Index::Transit, next_ix) = self.next_index(current_ix, ch) {
                current_ix = next_ix;
                if let Some(v) = self.data.get(current_ix) {
                    if v.len() > 0 {
                        f(&chars, &v[..]);
                    }
                }
            } else {
                return;
            }
        }
    }

    #[inline]
    fn each_prefix16<F: FnMut(usize, &[T])>(&self, key: &[u16], mut f: F) {
        let mut current_ix = 1;
        for (ix, ch) in key.iter().enumerate() {
            if let (Index::Transit, next_ix) = self.next_index(current_ix, *ch) {
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
    fn next_index(&self, current_index: usize, ch: u16) -> (Index, usize) {
        let current_base = self.array[current_index].base;
        if current_base == 0 {
            return (Index::Zero, 0);
        }
        let next_ix = current_base as usize + ch as usize;
        if next_ix < self.array.len() {
            let check_ix = self.array[next_ix].check as usize;
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

    pub fn insert(&mut self, key: impl AsUtf16, value: T) {
        let mut current_ix = 1;
        for ch in key.as_utf16() {
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
        self.array[next_ix] = Double {
            base: 0,
            check: current_ix as u32,
        };
        next_ix
    }

    #[inline]
    fn put_first_one(&mut self, current_ix: usize, ch: u16) -> usize {
        let position = self.find_new_base_one(ch);
        self.array[current_ix].base = position as u32 - ch as u32;
        return position;
    }

    #[inline]
    fn find_new_base_one(&mut self, ch: u16) -> usize {
        for i in ch as usize + 1..self.array.len() {
            if self.array[i].check == 0 {
                return i;
            }
        }
        let pos = max(self.array.len(), ch as usize + 1);
        self.extend(pos + 1);
        return pos;
    }

    fn rebase(&mut self, current_ix: usize, ch: u16) -> usize {
        let current_base = self.array[current_ix].base as usize;
        debug_assert!(current_base > 0);
        // 1. currIdx から遷移しているすべてのノード(遷移先ノード)を取得 (index, char)
        let mut next_nodes = vec![];
        for i in current_base..min(self.array.len(), current_base + u16::MAX as usize) {
            if self.array[i].check as usize == current_ix {
                next_nodes.push((i - current_base) as u16);
            }
        }
        debug_assert!(next_nodes.len() > 0);
        // 2. 遷移先ノードと currChar が遷移可能なbaseを求める
        let new_base = self.find_new_base(&next_nodes, ch);
        self.array[current_ix].base = new_base as u32;
        for ch in next_nodes {
            let src_ix = current_base + ch as usize;
            let dst_ix = new_base as usize + ch as usize;

            // 3. 遷移先ノードを新しい base で計算した index にコピー
            debug_assert!(self.array[dst_ix].base == 0);
            debug_assert!(self.array[dst_ix].check == 0);
            debug_assert!(self.data[dst_ix].len() == 0);
            let src_base = self.array[src_ix].base as usize;
            self.array[dst_ix] = self.array[src_ix];
            self.data.swap(src_ix, dst_ix);

            if src_base > 0 {
                // 4. 旧遷移先ノードから更に遷移しているノードの check を新遷移先ノードの index で更新
                let src_ix = src_ix as u32;
                let dst_ix = dst_ix as u32;
                let range = src_base..min(self.array.len(), src_base + u16::MAX as usize);
                for mut d in &mut self.array[range] {
                    if d.check == src_ix {
                        d.check = dst_ix
                    }
                }
            }
            // 5. 旧遷移先ノードの base, check, data をリセット
            self.array[src_ix].base = 0;
            self.array[src_ix].check = 0;
        }
        new_base as usize + ch as usize
    }

    fn find_new_base(&mut self, next_nodes: &[u16], ch: u16) -> usize {
        debug_assert!(next_nodes.len() > 0);

        let ch = ch as usize;
        let mut new_base = 0;
        'out: loop {
            new_base += 1;
            let mut ix = new_base + ch;
            while ix < self.array.len() && self.array[ix].check != 0 {
                ix += 1;
            }
            new_base = ix - ch as usize;

            for ch in next_nodes {
                let new_ix = new_base + *ch as usize;
                if new_ix < self.array.len() && self.array[new_ix].check != 0 {
                    continue 'out;
                }
            }
            // next_nodes は昇順のため最後の要素が最大である。
            let last_ix = max(ix, new_base + *next_nodes.last().unwrap() as usize);
            if last_ix >= self.array.len() {
                self.extend(last_ix + 1);
            }
            return new_base;
        }
    }

    fn extend(&mut self, size: usize) {
        debug_assert!(self.array.len() < size);

        self.array.resize(size, Double::new());
        let n = self.data.len();
        self.data.extend((n..size).map(|_| vec![]));

        debug_assert!(self.array.len() == size);
        debug_assert!(self.data.len() == size);
    }
}

impl<T: Debug> DoubleArray<T> {
    pub fn show_debug(&self) {
        for i in 0..self.array.len() {
            if self.array[i].base == 0 {
                continue;
            }
            let ch =
                char::from_u32(i as u32 - self.array[self.array[i].check as usize].base as u32);
            println!(
                "{}\t, {}\t, {}\t, {}\t, {:?}",
                i,
                self.array[i].base,
                self.array[i].check,
                ch.unwrap(),
                self.data[i]
            );
        }
    }
}

impl<T> PrefixMap<T> for DoubleArray<T> {
    #[inline]
    fn new() -> Self {
        DoubleArray::new()
    }
    #[inline]
    fn count(&self) -> usize {
        self.count()
    }
    #[inline]
    fn get(&self, key: impl AsUtf16) -> Option<&[T]> {
        self.get(key)
    }
    #[inline]
    fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, f: F) {
        self.each_prefix(key, f)
    }
    #[inline]
    fn each_prefix16<F: FnMut(usize, &[T])>(&self, key: &[u16], f: F) {
        self.each_prefix16(key, f)
    }
    #[inline]
    fn insert(&mut self, key: impl AsUtf16, value: T) {
        self.insert(key, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        pt.each_prefix("abcd", |chars, data| {
            vec.push((String::from_utf16_lossy(chars), data.to_owned()));
        });
        assert_eq!(
            vec,
            vec![("a".to_string(), vec![4, 5]), ("abc".to_string(), vec![1])]
        );
    }
}
