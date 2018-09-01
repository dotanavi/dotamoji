use std::mem::size_of;

use super::SearchCache;

type Bits = usize;

const NUM_BITS: usize = 8 * size_of::<Bits>();

#[inline]
fn round_up(value: usize) -> usize {
    (value + NUM_BITS) / NUM_BITS
}

pub struct BitCache(Vec<Bits>);

impl BitCache {

    #[inline]
    fn is_filled(&self, index: usize) -> bool {
        let data = &self.0;
        let a = index / NUM_BITS;
        let b = index % NUM_BITS;
        a < data.len() && (data[a] & (1 << b)) != 0
    }

    #[inline]
    fn find_base_one(&self, ch: usize, search_start: usize) -> usize {
        let data = &self.0;
        let ix = ch + search_start + 1;
        let a = ix / NUM_BITS;
        if a >= data.len() {
            return ix - ch;
        }

        let b = ix % NUM_BITS;
        for b in b .. NUM_BITS {
            if (data[a] & (1 << b)) == 0 {
                return a * NUM_BITS + b - ch;
            }
        }

        let mut a = a + 1;
        while a < data.len() {
            if data[a] != !0 {
                let b = Bits::trailing_zeros(!data[a]) as usize;
                return a * NUM_BITS + b - ch;
            }
            a += 1;
        }

        return a * NUM_BITS + b - ch;
    }
}

impl SearchCache for BitCache {

    #[inline]
    fn new(size: usize) -> Self { BitCache(vec![0; round_up(size)]) }

    #[inline]
    fn extend(&mut self, size: usize) { self.0.resize(round_up(size), 0); }

    #[inline]
    fn mark(&mut self, index: usize) {
        let a = index / NUM_BITS;
        let b = index % NUM_BITS;
        self.0[a] |= 1 << b;
    }

    #[inline]
    fn find_base<T>(&self, _check: &[u32], children: &[(u16, T)]) -> usize {
        let ch = children[0].0 as usize;

        let mut index = 0;
        'outer: loop {
            index = self.find_base_one(ch, index);
            for &(ch, _) in &children[1..] {
                if self.is_filled(index + ch as usize) {
                    continue 'outer;
                }
            }
            return index;
        }
    }
}
