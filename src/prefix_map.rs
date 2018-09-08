use as_chars::AsChars;

pub trait PrefixMap<K, V> {
    fn count(&self) -> usize;
    fn get<T: AsChars<K>>(&self, key: T) -> Option<&[V]>;
    fn insert<T: AsChars<K>>(&mut self, key: T, value: V);
    fn each_prefix<T: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: T, f: F);
}
