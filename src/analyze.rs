use std::i32;

use super::*;

struct Node {
    id: u16,
    cost: i32,
    len: u8,
    next: u8,
}

impl Node {
    fn new(id: u16, cost: i32, len: u8, next: u8) -> Node {
        Node {
            id,
            cost,
            len,
            next,
        }
    }
}

pub struct Analyzed {
    sentence: Vec<u16>,
    nodes: Vec<Vec<Node>>,
    pub cost: i32,
    index: u8,
}

impl Analyzed {
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter {
            x: self.index,
            y: 0,
            analyzed: &self,
        }
    }
}

pub struct Token<'a> {
    pub word: &'a [u16],
    pub id: u16,
    pub cost: i32,
}

pub struct Iter<'a> {
    x: u8,
    y: u32,
    analyzed: &'a Analyzed,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Token<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let nodes = &self.analyzed.nodes;
        let node = &nodes[nodes.len() - (self.y as usize) - 1][self.x as usize];
        if node.len == 0 {
            return None;
        }
        let index = self.y as usize;
        self.x = node.next;
        self.y += node.len as u32;
        let slice = &self.analyzed.sentence[index..self.y as usize];
        Some(Token {
            word: slice,
            id: node.id,
            cost: node.cost,
        })
    }
}

#[inline]
fn find_min_cost(src_id: u16, nodes: &[Node], matrix: &Matrix) -> Option<(usize, i32)> {
    nodes
        .iter()
        .map(|nd| nd.cost + matrix.get(src_id, nd.id) as i32)
        .enumerate()
        .min_by_key(|pair| pair.1)
}

#[inline]
pub fn analyze<D: PrefixMap<Info>>(
    sentence: &str,
    dic: &D,
    matrix: &Matrix,
) -> Result<Analyzed, ()> {
    let sentence: Vec<u16> = sentence.encode_utf16().collect();
    let mut nodes = Vec::with_capacity(sentence.len() + 1);
    nodes.push(vec![Node::new(0, 0, 0, 0)]);
    for ix in (0..sentence.len()).rev() {
        debug_assert!(nodes.len() == sentence.len() - ix);
        let mut column = vec![];
        dic.each_prefix16(&sentence[ix..], |len, info_list| {
            let search_nodes = &nodes[nodes.len() - len];
            for info in info_list {
                match find_min_cost(info.right_id, search_nodes, matrix) {
                    Some((index, min_cost)) => {
                        column.push(Node::new(
                            info.left_id,
                            min_cost + info.cost as i32,
                            len as u8,
                            index as u8,
                        ));
                    }
                    None => (),
                }
            }
        });
        if column.len() == 0 {
            // 未知語対応。とりあえず辞書にない単語はすべて一文字の固有名詞として扱う
            let len = 1;
            let search_nodes = &nodes[nodes.len() - len];
            let info = Info::new(1288, 1288, 10000);
            match find_min_cost(info.right_id, search_nodes, matrix) {
                Some((index, min_cost)) => {
                    column.push(Node::new(
                        info.left_id,
                        min_cost + info.cost as i32,
                        len as u8,
                        index as u8,
                    ));
                }
                None => (),
            }
        }
        nodes.push(column);
    }
    debug_assert_eq!(nodes.len(), sentence.len() + 1);
    if let Some((index, cost)) = find_min_cost(0, nodes.last().unwrap(), matrix) {
        Ok(Analyzed {
            sentence,
            nodes,
            cost,
            index: index as u8,
        })
    } else {
        return Err(());
    }
}
