
use super::SearchCache;

pub struct LinkCache {
    prev_links: Vec<u32>,
    next_links: Vec<u32>,
}

impl SearchCache for LinkCache {
    #[inline]
    fn new(size: usize) -> Self {
        let size = size + 1;
        let mut prev = Vec::with_capacity(size);
        prev.push(0);
        prev.extend(0 .. size as u32 - 1);
        debug_assert_eq!(prev.len(), size);

        let mut next = Vec::with_capacity(size);
        next.extend(1 .. size as u32 + 1);
        debug_assert_eq!(prev.len(), size);

        LinkCache {
            prev_links: prev,
            next_links: next,
        }
    }

    #[inline]
    fn extend(&mut self, size: usize) {
        let size = size as u32 + 1;
        let len = self.next_links.len() as u32;

        self.prev_links.extend(len - 1 .. size - 1);
        self.next_links.extend(len + 1 .. size + 1);

        let size = size as usize;
        debug_assert_eq!(self.prev_links.len(), size);
        debug_assert_eq!(self.next_links.len(), size);
    }

    #[inline]
    fn mark(&mut self, index: usize) {
        debug_assert_ne!(0, self.prev_links[index]);
        debug_assert_ne!(0, self.next_links[index]);
        let prev = self.prev_links[index] as usize;
        let next = self.next_links[index] as usize;
        // println!("mark: {} ({} <-> {})", index, prev, next);

        // debug_assert!(!self.is_filled(prev as usize), "prev[{}] is filled({:?})", prev, self.next_links.get(prev));
        // debug_assert!(!self.is_filled(next as usize), "next[{}] is filled({:?})", next, self.next_links.get(next));

        self.prev_links[next] = prev as u32;
        self.next_links[prev] = next as u32;

        self.prev_links[index] = 0;
        self.next_links[index] = 0;
    }

    #[inline]
    fn is_filled(&self, index: usize, _check: &[u32]) -> bool {
        index < self.next_links.len() && self.next_links[index] == 0
    }

    #[inline]
    fn find_empty(&self, ch: usize, search_start: usize, _check: &[u32]) -> usize {
        let links = &self.next_links;

        let mut ix = ch + search_start;
        if ix < links.len() && links[ix] != 0 {
            return links[ix] as usize - ch
        }
        ix += 1;
        while ix < links.len() && links[ix] == 0 {
            ix += 1;
        }
        return ix - ch;
    }
}

