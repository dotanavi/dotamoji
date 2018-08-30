// use std::cell::{RefCell};
use super::{PrefixMap, AsUtf16};

#[derive(Serialize, Deserialize)]
struct Assoc<T> {
    key: u16,
    value: Box<Node<T>>,
}

#[derive(Serialize, Deserialize)]
struct Node<T> {
    children: Vec<Assoc<T>>,
    data: Vec<T>,
}

impl <T> Node<T> {
    #[inline]
    fn new() -> Self { Node { children: vec![], data: vec![] } }

    fn count_data(&self) -> usize {
        let child_count: usize = self.children.iter()
            .map(|assoc| assoc.value.count_data())
            .sum();
        child_count + self.data.len()
    }

    fn dig_get(&self, mut iter: impl Iterator<Item = u16>) -> Option<&[T]> {
        if let Some(ch) = iter.next() {
            for assoc in self.children.iter() {
                if assoc.key == ch {
                    return assoc.value.dig_get(iter);
                }
            }
            None
        } else {
            if self.data.len() > 0 {
                Some(&self.data)
            } else {
                None
            }
        }
    }

    fn dig_set(&mut self, mut iter: impl Iterator<Item = u16>, data: T) {
        if let Some(ch) = iter.next() {
            for assoc in self.children.iter_mut() {
                if assoc.key == ch {
                    return assoc.value.dig_set(iter, data);
                }
            }
            let mut node = Node::new();
            node.dig_set(iter, data);
            self.children.push(Assoc { key: ch, value: Box::new(node) });
        } else {
            self.data.push(data);
        }
    }

    fn dig_yield<I: Iterator<Item = u16>, F: FnMut(usize, &[T])>(&self, depth: usize, mut iter: I, mut f: F) {
        if self.data.len() > 0 {
            f(depth, &self.data);
        }
        if let Some(ch) = iter.next() {
            for assoc in self.children.iter() {
                if assoc.key == ch {
                    return assoc.value.dig_yield(depth + 1, iter, f);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Trie<T>{
    root: Node<T>,
}

impl <T> PrefixMap<T> for Trie<T> {

    #[inline]
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    #[inline]
    fn count(&self) -> usize {
        self.root.count_data()
    }

    #[inline]
    fn get(&self, key: impl AsUtf16) -> Option<&[T]> {
        self.root.dig_get(key.as_utf16())
    }

    #[allow(unused_variables)]
    fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, f: F) {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn each_prefix16<F: FnMut(usize, &[T])>(&self, key: &[u16], f: F) {
        self.root.dig_yield(0, key.as_utf16(), f);
    }

    #[inline]
    fn insert(&mut self, key: impl AsUtf16, value: T) {
        self.root.dig_set(key.as_utf16(), value);
    }
}

