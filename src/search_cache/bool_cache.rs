use super::SearchCache;

pub struct BoolCache(Vec<bool>);

impl BoolCache {

    #[inline]
    fn find_base_one(&self, ch: usize, search_start: usize) -> usize {
        let cache = &self.0;

        let start_pos = ch + search_start + 1;
        if start_pos < cache.len() {
            if let Some(ix2) = cache[start_pos..].iter().position(|&x| !x) {
                return ix2 + search_start + 1;
            }
            return cache.len() - ch;
        }
        return start_pos - ch;
    }
}

impl SearchCache for BoolCache {
    #[inline]
    fn new(size: usize) -> Self { BoolCache(vec![false; size]) }

    #[inline]
    fn extend(&mut self, size: usize) { self.0.resize(size, false); }

    #[inline]
    fn mark(&mut self, index: usize) { self.0[index] = true; }

    #[inline]
    fn find_base<T>(&self, _check: &[u32], children: &[(u16, T)]) -> usize {
        let cache = &self.0;
        let ch = children[0].0 as usize;

        let mut index = 0;
        'outer: loop {
            index = self.find_base_one(ch, index);
            for &(ch, _) in &children[1..] {
                let ix = index + ch as usize;
                if ix < cache.len() && cache[ix] {
                    continue 'outer;
                }
            }
            return index;
        }
    }
}
