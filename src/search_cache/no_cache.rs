use super::SearchCache;

pub struct NoCache;

impl SearchCache for NoCache {

    #[inline]
    fn new(_size: usize) -> Self { NoCache }

    #[inline]
    fn extend(&mut self, _size: usize) { }

    #[inline]
    fn mark(&mut self, _index: usize) { }

    #[inline]
    fn is_filled(&self, index: usize, check: &[u32]) -> bool {
        index < check.len() && check[index] != 0
    }

    #[inline]
    fn find_empty(&self, ch: usize, search_start: usize, check: &[u32]) -> usize {
        let mut ix = ch + search_start + 1;
        while ix < check.len() && check[ix] != 0 {
            ix += 1;
        }
        return ix - ch;
    }
}

