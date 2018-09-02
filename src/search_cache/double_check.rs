use super::SearchCache;

pub struct DoubleCheck<C1, C2> {
    cache1: C1,
    cache2: C2,
}

impl<C1: SearchCache, C2: SearchCache> SearchCache for DoubleCheck<C1, C2> {
    #[inline]
    fn new(size: usize) -> Self {
        DoubleCheck {
            cache1: C1::new(size),
            cache2: C2::new(size),
        }
    }

    #[inline]
    fn extend(&mut self, size: usize) {
        self.cache1.extend(size);
        self.cache2.extend(size);
    }

    #[inline]
    fn mark(&mut self, index: usize) {
        self.cache1.mark(index);
        self.cache2.mark(index);
    }

    #[inline]
    fn is_filled(&self, index: usize, check: &[u32]) -> bool {
        let value1 = self.cache1.is_filled(index, check);
        let value2 = self.cache2.is_filled(index, check);
        assert_eq!(value1, value2);
        return value1;
    }

    #[inline]
    fn find_empty(&self, ch: usize, search_start: usize, check: &[u32]) -> usize {
        let value1 = self.cache1.find_empty(ch, search_start, check);
        let value2 = self.cache2.find_empty(ch, search_start, check);
        assert_eq!(value1, value2);
        return value1;
    }
}
