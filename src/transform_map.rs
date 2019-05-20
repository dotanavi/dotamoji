use crate::as_chars::AsChars;
use crate::dictionary::SaveDict;
use crate::prefix_map::PrefixMap;
use std::io::Write;
use std::marker::PhantomData;
use std::time::Instant;

pub trait Transform<M1, M2> {
    fn transform(src: M1) -> M2;
}

pub enum TransformMap<M1, M2, Tr: Transform<M1, M2>> {
    Before(M1, PhantomData<Tr>),
    After(M2),
}
use self::TransformMap::*;

impl<M1: Default, M2, Tr: Transform<M1, M2>> Default for TransformMap<M1, M2, Tr> {
    fn default() -> Self {
        Before(M1::default(), PhantomData)
    }
}

impl<K, V, M1, M2, Tr> PrefixMap<K, V> for TransformMap<M1, M2, Tr>
where
    M1: PrefixMap<K, V>,
    M2: PrefixMap<K, V>,
    Tr: Transform<M1, M2>,
{
    #[inline]
    fn count(&self) -> usize {
        match self {
            Before(x, _) => x.count(),
            After(x) => x.count(),
        }
    }

    #[inline]
    fn get<T: AsChars<K>>(&self, key: T) -> Option<&[V]> {
        match self {
            Before(x, _) => x.get(key),
            After(x) => x.get(key),
        }
    }

    #[inline]
    fn insert<T: AsChars<K>>(&mut self, key: T, value: V) {
        match self {
            Before(x, _) => x.insert(key, value),
            After(x) => x.insert(key, value),
        }
    }

    #[inline]
    fn each_prefix<T: AsChars<K>, F: FnMut(usize, &[V])>(&self, key: T, f: F) {
        match self {
            Before(x, _) => x.each_prefix(key, f),
            After(x) => x.each_prefix(key, f),
        }
    }
}

impl<K, V, M1, M2, Tr> SaveDict<K, V> for TransformMap<M1, M2, Tr>
where
    M1: PrefixMap<K, V>,
    M2: SaveDict<K, V>,
    Tr: Transform<M1, M2>,
{
    fn save_to_file<W: Write>(self, file: W) -> Self {
        let dic = match self {
            Before(x, _) => {
                let start = Instant::now();
                let x = Tr::transform(x);
                eprintln!("transform: {:?}", start.elapsed());
                x
            }
            After(x) => x,
        };
        After(dic.save_to_file(file))
    }
}
