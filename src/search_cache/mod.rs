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
    fn find_base<T>(&self, check: &[u32], children: &[(u16, T)]) -> usize;
}
