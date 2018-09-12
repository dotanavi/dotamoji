use std::mem::size_of;

use super::{SearchCache, SearchCache2};

type Bits = usize;

const NUM_BITS: usize = 8 * size_of::<Bits>();

#[inline]
fn round_up(value: usize) -> usize {
    (value + NUM_BITS) / NUM_BITS
}

pub struct BitCache(Vec<Bits>);

impl BitCache {

    #[inline]
    fn bits2(&self, a: usize, b: usize) -> Bits {
        let data = &self.0;
        if a + 1 < data.len() {
            if b == 0 {
                return data[a];
            }
            let low = data[a] >> b;
            let high = data[a + 1] << (NUM_BITS - b);
            return high | low;
        }
        if a < data.len() {
            let low = data[a] >> b;
            return low;
        }
        return 0;
    }

    #[inline]
    pub fn bits(&self, ix: usize) -> Bits {
        self.bits2(ix / NUM_BITS, ix % NUM_BITS)
    }

    #[inline]
    pub fn find_empty_with_bits(&self, search_start: usize) -> (usize, Bits) {
        let data = &self.0;
        let ix = search_start + 1;
        let a = ix / NUM_BITS;
        let b = ix % NUM_BITS;
        if a >= data.len() {
            return (ix, 0);
        }

        let masked = data[a] | ((1 << b) - 1);
        if masked != !0 {
            let b = Bits::trailing_zeros(!masked) as usize;
            let ret_ix = a * NUM_BITS + b;
            let ret_bits = self.bits2(a, b);
            return (ret_ix, ret_bits);
        }

        let mut a = a + 1;
        while a < data.len() {
            if data[a] != !0 {
                let b = Bits::trailing_zeros(!data[a]) as usize;
                let ret_ix = a * NUM_BITS + b;
                let ret_bits = self.bits2(a, b);
                return (ret_ix, ret_bits);
            }
            a += 1;
        }

        return (a * NUM_BITS, 0);
    }

}

impl SearchCache for BitCache {
    #[inline]
    fn new(size: usize) -> Self {
        BitCache(vec![0; round_up(size)])
    }

    #[inline]
    fn extend(&mut self, size: usize) {
        self.0.resize(round_up(size), 0);
    }

    #[inline]
    fn mark(&mut self, index: usize) {
        let a = index / NUM_BITS;
        let b = index % NUM_BITS;
        self.0[a] |= 1 << b;
    }

    #[inline]
    fn is_filled(&self, index: usize, _check: &[u32]) -> bool {
        let data = &self.0;
        let a = index / NUM_BITS;
        let b = index % NUM_BITS;
        a < data.len() && (data[a] & (1 << b)) != 0
    }

    #[inline]
    fn find_empty(&self, search_start: usize, _check: &[u32]) -> usize {
        let data = &self.0;
        let ix = search_start + 1;
        let a = ix / NUM_BITS;
        let b = ix % NUM_BITS;
        if a >= data.len() {
            return ix;
        }

        // for b in b..NUM_BITS {
        //     if (data[a] & (1 << b)) == 0 {
        //         return a * NUM_BITS + b;
        //     }
        // }
        let masked = data[a] | ((1 << b) - 1);
        if masked != !0 {
            let b = Bits::trailing_zeros(!masked) as usize;
            return a * NUM_BITS + b;
        }

        let mut a = a + 1;
        while a < data.len() {
            if data[a] != !0 {
                let b = Bits::trailing_zeros(!data[a]) as usize;
                return a * NUM_BITS + b;
            }
            a += 1;
        }

        return a * NUM_BITS;
    }

    fn find_all_empties<T, F>(&self, _check: &[u32], ch: usize, rest: &[T], f: F) -> usize
    where
        F: Fn(&T) -> usize,
    {
        let mut index = 0;
        'outer: loop {
            let (ch_index, bits) = self.find_empty_with_bits(ch + index);
            let mut bits = bits;
            index = ch_index - ch;
            for x in rest {
                let ch = f(x);
                let bits2 = self.bits(index + ch);
                bits |= bits2;
                if bits == !0 {
                    index += NUM_BITS - 1;
                    continue 'outer;
                }
            }
            return index + Bits::trailing_zeros(!bits) as usize;
        }
    }
}

impl SearchCache2 for BitCache {
    #[inline]
    fn unmark(&mut self, index: usize) {
        let a = index / NUM_BITS;
        let b = index % NUM_BITS;
        self.0[a] &= !(1 << b);
    }
}
