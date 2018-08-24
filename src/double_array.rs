use std;

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

    pub fn find(&self, key: &str) -> Option<&[T]> {
        let mut current_ix = 1;
        for ch in key.encode_utf16() {
            let next_ix = self.base[current_ix] as usize + ch as usize;
            if next_ix < self.check.len() && self.check[next_ix] as usize == current_ix {
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
            let next_ix = self.base[current_ix] as usize + ch as usize;
            if next_ix < self.check.len() && self.check[next_ix] as usize == current_ix {
                current_ix = next_ix;
                if let Some(v) = self.data.get(current_ix) {
                    if v.len() > 0 {
                        f(Self::decode_utf16(&chars), &v[..]);
                    }
                }
            }
        }
    }

    fn decode_utf16(chars: &[u16]) -> String {
        std::char::decode_utf16(chars.iter().cloned())
            .filter_map(Result::ok)
            .collect()
    }

    pub fn add(&mut self, key: &str, value: T) {
        let (mut current_ix, key_ix) = self.find_failed_position(key);
        for (ix, ch) in key.encode_utf16().enumerate() {
            if ix < key_ix { continue; }

            let current_base = self.base[current_ix] as usize;
            let tmp_next_ix = current_base + ch as usize;
            if tmp_next_ix < self.check.len() && self.check[tmp_next_ix] != 0 { // 衝突時
                // 1. currIdx から遷移しているすべてのノード(遷移先ノード)を取得 (index, char)
                let mut next_nodes = vec![];
                for i in current_base .. current_base + std::u16::MAX as usize {
                    if i >= self.check.len() { break; }
                    if self.check[i] as usize == current_ix {
                        next_nodes.push((i - current_base) as u16);
                    }
                }
                // 2. 遷移先ノードと currChar が遷移可能なbaseを求める
                let new_base = self.find_new_base(&next_nodes, ch);
                self.base[current_ix] = new_base;
                for ch in next_nodes {
                    let src_ix = current_base + ch as usize;
                    let src_base = self.base[src_ix] as usize;
                    let dst_ix = self.base[current_ix] as usize + ch as usize;
                    // 3. 遷移先ノードを新しい base で計算した index にコピー
                    self.base[dst_ix] = self.base[src_ix];
                    self.check[dst_ix] = self.check[src_ix];
                    self.data.swap(src_ix, dst_ix);
                    // 4. 旧遷移先ノードから更に遷移しているノードの check を新遷移先ノードの index で更新
                    for i in src_base .. src_base + std::u16::MAX as usize {
                        if i >= self.check.len() { break; }
                        if self.check[i] as usize == src_ix {
                            self.check[i] = dst_ix as u32;
                        }
                    }
                    // 5. 旧遷移先ノードの base, check, data をリセット
                    self.base[src_ix] = 1;
                    self.check[src_ix] = 0;
                    debug_assert!(self.data[src_ix].len() == 0);
                }
            }
            // currChar のノードを追加
            let next_ix = self.base[current_ix] as usize + ch as usize;
            if next_ix >= self.check.len() {
                self.extend(next_ix + 1);
            }
            self.base[next_ix] = 1;
            self.check[next_ix] = current_ix as u32;
            current_ix = next_ix;
        }
        // データを登録
        self.data[current_ix].push(value);
    }

    fn find_failed_position(&self, key: &str) -> (usize, usize) {
        let mut current_ix = 1;
        for (ix, ch) in key.encode_utf16().enumerate() {
            let next_ix = self.base[current_ix] as usize + ch as usize;
            if next_ix < self.check.len() && self.check[next_ix] as usize == current_ix {
                current_ix = next_ix;
            } else {
                return (current_ix, ix)
            }
        }
        return (current_ix, key.encode_utf16().count())
    }

    fn find_new_base(&mut self, next_nodes: &[u16], ch: u16) -> u32 {
        let mut new_base = 0;
        'out: loop {
            new_base += 1;
            for ch in next_nodes.iter().chain(std::iter::once(&ch)) {
                let new_ix = new_base as usize + *ch as usize;
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

impl<T: std::fmt::Debug> DoubleArray<T> {
    pub fn show_debug(&self) {
        for i in 0 .. self.check.len() {
            if self.base[i] == 0 { continue; }
            let ch = std::char::from_u32(i as u32 - self.base[self.check[i] as usize] as u32);
            println!("{}\t, {}\t, {}\t, {}\t, {:?}",
                i, self.base[i], self.check[i], ch.unwrap(), self.data[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // "未登録の要素を取り出そうとするとNoneを返す"
    fn test_not_registered() {
        let pt: DoubleArray<()> = DoubleArray::new();
        assert_eq!(pt.find("abc"), None);
    }

    #[test]
    // "配列の長さが足りない場合はNoneを返す"
    fn test_small() {
        let pt: DoubleArray<()> = DoubleArray::new();
        assert_eq!(pt.find("abc"), None);
    }

    #[test]
    // "途中までのキーが登録されている場合はNoneを返す"
    fn test_mid() {
        let mut pt = DoubleArray::new();
        pt.add("ab", 1);
        assert_eq!(pt.find("abc"), None);
    }

    #[test]
    // "遷移は可能だがdataが登録されていない場合はNoneを返す"
    fn test_over() {
        let mut pt = DoubleArray::new();
        pt.add("abcd", 1);
        assert_eq!(pt.find("abc"), None);
    }

    #[test]
    // "衝突しない要素の登録"
    fn test_no_conflict() {
        let mut pt = DoubleArray::new();
        pt.add("abc", 1);
        pt.add("ab", 2);
        assert_eq!(pt.find("abc"), Some(&[1][..]));
        assert_eq!(pt.find("ab"), Some(&[2][..]));
    }

    #[test]
    // "重複していない値の登録"
    fn test_dup_value() {
        let mut pt = DoubleArray::new();
        pt.add("ab", 1);
        pt.add("ab", 2);
        assert_eq!(pt.find("ab"), Some(&[1, 2][..]));
    }

    #[test]
    // "衝突する場合"
    fn test_conflict() {
        let mut pt = DoubleArray::new();
        pt.add("abc", 1);
        pt.add("ad", 2);
        pt.add("ac", 3);

        assert_eq!(pt.find("abc"), Some(&[1][..]));
        assert_eq!(pt.find("ad"), Some(&[2][..]));
        assert_eq!(pt.find("ac"), Some(&[3][..]));
    }

    #[test]
    // "マルチバイト文字"
    fn test_multibyte() {
        let mut pt = DoubleArray::new();
        pt.add("おはよう", 1);
        pt.add("およごう", 2);

        assert_eq!(pt.find("おはよう"), Some(&[1][..]));
        assert_eq!(pt.find("およごう"), Some(&[2][..]));
    }

    #[test]
    // "遷移先ノードを正確に取得できているか"
    fn test_transite() {
        let mut pt = DoubleArray::new();
        pt.add("ba", 1);
        pt.add("bb", 2);

        assert_eq!(pt.find("ba"), Some(&[1][..]));
        assert_eq!(pt.find("bb"), Some(&[2][..]));
    }

    #[test]
    // "前方一致検索。"
    fn test_prefix() {
        let mut pt = DoubleArray::new();
        pt.add("abc", 1);
        pt.add("ad", 2);
        pt.add("ac", 3);
        pt.add("a", 4);
        pt.add("a", 5);

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
