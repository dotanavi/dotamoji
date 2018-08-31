pub mod no_cache;
pub mod bool_cache;
pub mod bit_cache;

pub trait SearchCache {
    fn new(size: usize) -> Self;
    fn extend(&mut self, size: usize);
    fn mark(&mut self, index: usize);
    fn find_base<T>(&self, check: &[u32], children: &[(u16, T)]) -> usize;
}
