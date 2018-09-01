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
    fn find_base<T>(&self, check: &[u32], children: &[(u16, T)]) -> usize {
        let index1 = self.cache1.find_base(check, children);
        let index2 = self.cache2.find_base(check, children);
        assert_eq!(index1, index2);
        return index1;
    }
}
