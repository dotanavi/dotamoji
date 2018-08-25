use std::{char, u16, cmp::min, iter::once, fmt::Debug};

#[derive(Eq, PartialEq)]
enum Index { Ok, Empty, Conflict, OutOfRange }

pub struct DoubleArray<T> {
    base: Vec<u32>,
    check: Vec<u32>,
    data: Vec<Vec<T>>,
}

impl<T> DoubleArray<T> {
    pub fn new() -> Self {
        DoubleArray {
            base: vec![0, 1],
            check: vec![0, 0],
            data: vec![vec![], vec![]],
        }
    }

    #[inline]
    pub fn len(&self) -> usize { self.base.len() }

    pub fn get(&self, key: &str) -> Option<&[T]> {
        let mut current_ix = 1;
        for ch in key.encode_utf16() {
            if let (Index::Ok, next_ix) = self.next_index(current_ix, ch) {
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

    pub fn each_prefix<F: FnMut(String, &[T])>(&self, key: &str, mut f: F) {
        let mut chars: Vec<u16> = vec![];
        let mut current_ix = 1;
        for ch in key.encode_utf16() {
            chars.push(ch);
            if let (Index::Ok, next_ix) = self.next_index(current_ix, ch) {
                current_ix = next_ix;
                if let Some(v) = self.data.get(current_ix) {
                    if v.len() > 0 {
                        f(decode_utf16(&chars), &v[..]);
                    }
                }
            }
        }
    }

    #[inline]
    fn next_index(&self, current_index: usize, ch: u16) -> (Index, usize) {
        let next_ix = self.base[current_index] as usize + ch as usize;
        if next_ix < self.check.len() {
            let check_ix = self.check[next_ix] as usize;
            if check_ix == current_index {
                (Index::Ok, next_ix)
            } else if check_ix == 0 {
                (Index::Empty, next_ix)
            } else {
                (Index::Conflict, next_ix)
            }
        } else {
            (Index::OutOfRange, next_ix)
        }
    }

    pub fn insert(&mut self, key: &str, value: T) {
        let mut current_ix = 1;
        for ch in key.encode_utf16() {
            let (state, next_ix) = self.next_index(current_ix, ch);
            current_ix = match state {
                Index::Ok => next_ix,
                Index::Empty => {
                    self.update(current_ix, next_ix)
                }
                Index::Conflict => {
                    let new_next_ix = self.rebase(current_ix, ch);
                    self.update(current_ix, new_next_ix)
                },
                Index::OutOfRange => {
                    self.extend(next_ix + 1);
                    self.update(current_ix, next_ix)
                },
            };
        }
        // データを登録
        self.data[current_ix].push(value);
    }

    #[inline]
    fn update(&mut self, current_ix: usize, next_ix: usize) -> usize {
        self.base[next_ix] = 1;
        self.check[next_ix] = current_ix as u32;
        next_ix
    }

    fn rebase(&mut self, current_ix: usize, ch: u16) -> usize {
        let current_base = self.base[current_ix] as usize;
        // 1. currIdx から遷移しているすべてのノード(遷移先ノード)を取得 (index, char)
        let mut next_nodes = vec![];
        for i in current_base .. min(self.check.len(), current_base + u16::MAX as usize) {
            if self.check[i] as usize == current_ix {
                next_nodes.push((i - current_base) as u16);
            }
        }
        // 2. 遷移先ノードと currChar が遷移可能なbaseを求める
        let new_base = self.find_new_base(&next_nodes, ch);
        self.base[current_ix] = new_base as u32;
        for ch in next_nodes {
            let src_ix = current_base + ch as usize;
            let src_base = self.base[src_ix] as usize;
            let dst_ix = new_base as usize + ch as usize;

            debug_assert!(self.base[dst_ix] == 0);
            debug_assert!(self.check[dst_ix] == 0);
            debug_assert!(self.data[dst_ix].len() == 0);
            // 3. 遷移先ノードを新しい base で計算した index にコピー
            self.base[dst_ix] = self.base[src_ix];
            self.check[dst_ix] = self.check[src_ix];
            self.data.swap(src_ix, dst_ix);

            // 4. 旧遷移先ノードから更に遷移しているノードの check を新遷移先ノードの index で更新
            for i in src_base .. min(self.check.len(), src_base + u16::MAX as usize) {
                if self.check[i] as usize == src_ix {
                    self.check[i] = dst_ix as u32;
                }
            }
            // 5. 旧遷移先ノードの base, check, data をリセット
            self.base[src_ix] = 0;
            self.check[src_ix] = 0;
        }
        new_base as usize + ch as usize
    }

    fn find_new_base(&mut self, next_nodes: &[u16], ch: u16) -> usize {
        let mut new_base = 0;
        'out: loop {
            new_base += 1;
            for ch in next_nodes.iter().chain(once(&ch)) {
                let new_ix = new_base + *ch as usize;
                if new_ix < self.check.len() {
                    if self.check[new_ix] == 0 {
                        // OK
                    } else {
                        continue 'out;
                    }
                } else {
                    self.extend(new_ix + 1);
                }
            }
            return new_base;
        }
    }

    fn extend(&mut self, size: usize) {
        debug_assert!(self.base.len() < size);

        self.base.resize(size, 0);
        self.check.resize(size, 0);
        let n = self.data.len();
        self.data.extend((n .. size).map(|_| vec![]));

        debug_assert!(self.base.len() == size);
        debug_assert!(self.check.len() == size);
        debug_assert!(self.data.len() == size);
    }
}

impl<T: Debug> DoubleArray<T> {
    pub fn show_debug(&self) {
        for i in 0 .. self.check.len() {
            if self.base[i] == 0 { continue; }
            let ch = char::from_u32(i as u32 - self.base[self.check[i] as usize] as u32);
            println!("{}\t, {}\t, {}\t, {}\t, {:?}",
                i, self.base[i], self.check[i], ch.unwrap(), self.data[i]);
        }
    }
}

fn decode_utf16(chars: &[u16]) -> String {
    char::decode_utf16(chars.iter().cloned())
        .filter_map(Result::ok)
        .collect()
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
        pt.each_prefix("abcd", |string, data| {
            vec.push((string, data.to_owned()));
        });
        assert_eq!(vec, vec![
            ("a".to_string(), vec![4, 5]),
            ("abc".to_string(), vec![1]),
        ]);
    }
}
