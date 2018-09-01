mod no_cache;
mod bool_cache;
mod bit_cache;
mod link_cache;
mod double_check;

pub use self::no_cache::NoCache;
pub use self::bool_cache::BoolCache;
pub use self::bit_cache::BitCache;
pub use self::link_cache::LinkCache;
pub use self::double_check::DoubleCheck;

pub trait SearchCache {
    fn new(size: usize) -> Self;
    fn extend(&mut self, size: usize);
    fn mark(&mut self, index: usize);
    fn is_filled(&self, index: usize, check: &[u32]) -> bool;
    fn find_empty(&self, ch: usize, search_start: usize, check: &[u32]) -> usize;
}
