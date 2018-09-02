use super::SearchCache;

pub struct BoolCache(Vec<bool>);

impl SearchCache for BoolCache {
    #[inline]
    fn new(size: usize) -> Self {
        BoolCache(vec![false; size])
    }

    #[inline]
    fn extend(&mut self, size: usize) {
        self.0.resize(size, false);
    }

    #[inline]
    fn mark(&mut self, index: usize) {
        self.0[index] = true;
    }

    #[inline]
    fn is_filled(&self, index: usize, _check: &[u32]) -> bool {
        let cache = &self.0;
        index < cache.len() && cache[index]
    }

    #[inline]
    fn find_empty(&self, ch: usize, search_start: usize, _check: &[u32]) -> usize {
        let cache = &self.0;

        let start_pos = ch + search_start + 1;
        if start_pos < cache.len() {
            if let Some(ix) = cache[start_pos..].iter().position(|&x| !x) {
                return ix + search_start + 1;
            }
            return cache.len() - ch;
        }
        return start_pos - ch;
    }
}
