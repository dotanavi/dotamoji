use crate::as_chars::AsUsize;
use crate::double_array::DoubleArray;
use crate::search_cache::{BitCache1, NoCache, SearchCache2};
use crate::transform_map::{Transform, TransformMap};

pub enum Shrink {}

impl<K: AsUsize, V, C: SearchCache2> Transform<DoubleArray<K, V, C>, DoubleArray<K, V, NoCache>>
    for Shrink
{
    fn transform(src: DoubleArray<K, V, C>) -> DoubleArray<K, V, NoCache> {
        DoubleArray::from_raw_parts(src.base, src.check, src.data)
    }
}

pub type FastBuildDoubleArray<K, V> =
    TransformMap<DoubleArray<K, V, BitCache1>, DoubleArray<K, V, NoCache>, Shrink>;
