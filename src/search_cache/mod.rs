mod bit_cache0;
mod bit_cache1;
mod bool_cache;
mod double_check;
mod link_cache;
mod no_cache;

pub use self::bit_cache0::BitCache as BitCache0;
pub use self::bit_cache1::BitCache as BitCache1;
pub use self::bool_cache::BoolCache;
pub use self::double_check::DoubleCheck;
pub use self::link_cache::LinkCache;
pub use self::no_cache::NoCache;

pub trait SearchCache {
    fn new(size: usize) -> Self;
    fn extend(&mut self, size: usize);
    fn mark(&mut self, index: usize);
    fn is_filled(&self, index: usize, check: &[u32]) -> bool;
    fn find_empty(&self, search_start: usize, check: &[u32]) -> usize;

    fn find_all_empties<T, F>(&self, check: &[u32], ch: usize, rest: &[T], f: F) -> usize
    where
        F: Fn(&T) -> usize,
    {
        let mut index = 0;
        'outer: loop {
            index = self.find_empty(ch + index, check) - ch;
            for x in rest {
                let ch = f(x);
                if self.is_filled(index + ch, check) {
                    continue 'outer;
                }
            }
            return index;
        }
    }
}

pub trait SearchCache2: SearchCache {
    fn unmark(&mut self, index: usize);
}
