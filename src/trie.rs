use super::PrefixMapOld;
use as_chars::AsChars;

pub trait Node<T> {
    fn new() -> Self;

    fn search(&self, ch: u16) -> Result<usize, usize>;

    fn next_node(&self, index: usize) -> &Self;

    fn search_or_create(&mut self, ch: u16) -> &mut Self;

    fn count_data(&self) -> usize;

    fn get_data(&self) -> &[T];

    fn push_data(&mut self, data: T);

    fn dig_get(&self, mut iter: impl Iterator<Item = u16>) -> Option<&[T]> {
        if let Some(ch) = iter.next() {
            if let Ok(index) = self.search(ch) {
                return self.next_node(index).dig_get(iter);
            }
            None
        } else {
            let data = self.get_data();
            if data.len() > 0 {
                Some(data)
            } else {
                None
            }
        }
    }

    fn dig_set(&mut self, mut iter: impl Iterator<Item = u16>, data: T) {
        if let Some(ch) = iter.next() {
            self.search_or_create(ch).dig_set(iter, data);
        } else {
            self.push_data(data);
        }
    }

    fn dig_yield<I: Iterator<Item = u16>, F: FnMut(usize, &[T])>(
        &self,
        depth: usize,
        mut iter: I,
        mut f: F,
    ) {
        let data = self.get_data();
        if data.len() > 0 {
            f(depth, data);
        }
        if let Some(ch) = iter.next() {
            if let Ok(index) = self.search(ch) {
                return self.next_node(index).dig_yield(depth + 1, iter, f);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeA<T> {
    pub children: Vec<(u16, NodeA<T>)>,
    pub data: Vec<T>,
}

impl<T> Node<T> for NodeA<T> {
    #[inline]
    fn new() -> Self {
        Self {
            children: vec![],
            data: vec![],
        }
    }

    #[inline]
    fn search(&self, ch: u16) -> Result<usize, usize> {
        self.children.binary_search_by_key(&ch, |&(c, _)| c)
    }

    #[inline]
    fn next_node(&self, index: usize) -> &Self {
        &self.children[index].1
    }

    #[inline]
    fn search_or_create(&mut self, ch: u16) -> &mut Self {
        let index = match self.search(ch) {
            Ok(index) => index,
            Err(index) => {
                self.children.insert(index, (ch, Node::new()));
                index
            }
        };
        &mut self.children[index].1
    }

    #[inline]
    fn get_data(&self) -> &[T] {
        &self.data
    }

    #[inline]
    fn push_data(&mut self, data: T) {
        self.data.push(data);
    }

    fn count_data(&self) -> usize {
        let child_count: usize = self.children.iter().map(|(_, n)| n.count_data()).sum();
        child_count + self.data.len()
    }
}

#[derive(Serialize, Deserialize)]
pub struct NodeB<T> {
    pub labels: Vec<u16>,
    pub nodes: Vec<NodeB<T>>,
    pub data: Vec<T>,
}

impl<T> Node<T> for NodeB<T> {
    #[inline]
    fn new() -> Self {
        Self {
            labels: vec![],
            nodes: vec![],
            data: vec![],
        }
    }

    #[inline]
    fn search(&self, ch: u16) -> Result<usize, usize> {
        self.labels.binary_search_by_key(&ch, |&c| c)
    }

    #[inline]
    fn next_node(&self, index: usize) -> &Self {
        &self.nodes[index]
    }

    #[inline]
    fn search_or_create(&mut self, ch: u16) -> &mut Self {
        let index = match self.search(ch) {
            Ok(index) => index,
            Err(index) => {
                self.labels.insert(index, ch);
                self.nodes.insert(index, Node::new());
                index
            }
        };
        &mut self.nodes[index]
    }

    #[inline]
    fn get_data(&self) -> &[T] {
        &self.data
    }

    #[inline]
    fn push_data(&mut self, data: T) {
        self.data.push(data);
    }

    fn count_data(&self) -> usize {
        let child_count: usize = self.nodes.iter().map(|n| n.count_data()).sum();
        child_count + self.data.len()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Trie<N> {
    pub(crate) root: N,
}

impl<T, N: Node<T>> PrefixMapOld<T> for Trie<N> {
    #[inline]
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    #[inline]
    fn count(&self) -> usize {
        self.root.count_data()
    }

    #[inline]
    fn get(&self, key: impl AsChars<u16>) -> Option<&[T]> {
        self.root.dig_get(key.as_chars())
    }

    #[allow(unused_variables)]
    fn each_prefix<F: FnMut(&[u16], &[T])>(&self, key: &str, f: F) {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn each_prefix16<F: FnMut(usize, &[T])>(&self, key: &[u16], f: F) {
        self.root.dig_yield(0, key.as_chars(), f);
    }

    #[inline]
    fn insert(&mut self, key: impl AsChars<u16>, value: T) {
        self.root.dig_set(key.as_chars(), value);
    }
}
