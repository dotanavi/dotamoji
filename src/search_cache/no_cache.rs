use super::SearchCache;

pub struct NoCache;

impl NoCache {
    #[inline]
    fn find_base_one(&self, ch: usize, search_start: usize, check: &[u32]) -> usize {
        let mut ix = ch + search_start + 1;
        while ix < check.len() && check[ix] != 0 {
            ix += 1;
        }
        return ix - ch;
    }
}

impl SearchCache for NoCache {
    fn new(_size: usize) -> Self { NoCache }
    fn extend(&mut self, _size: usize) { }
    fn mark(&mut self, _index: usize) { }

    #[inline]
    fn find_base<T>(&self, check: &[u32], children: &[(u16, T)]) -> usize {
        let ch = children[0].0 as usize;

        let mut index = 0;
        'outer: loop {
            index = self.find_base_one(ch, index, check);
            for &(ch, _) in &children[1..] {
                let ix = index + ch as usize;
                if ix < check.len() && check[ix] != 0 {
                    continue 'outer;
                }
            }
            return index;
        }
    }
}

