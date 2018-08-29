use std::i32;

use super::*;
use super::util::decode_utf16;

struct Node {
    id: u16,
    cost: i32,
    len: u8,
    next: u8,
}

impl Node {
    fn new(id: u16, cost: i32, len: u8, next: u8) -> Node {
        Node { id, cost, len, next }
    }
}

#[inline]
fn find_min_cost(src_id: u16, nodes: &[Node], matrix: &Matrix) -> Option<(usize, i32)>{
    nodes.iter()
        .map(|nd| nd.cost + matrix.get(src_id, nd.id) as i32)
        .enumerate()
        .min_by_key(|pair| pair.1)
}

pub fn analyze<D: Dictionary<Info>>(dic: &D, matrix: &Matrix, sentence: &str) {
    let sentence: Vec<u16> = sentence.encode_utf16().collect();

    let mut nodes = vec![vec![Node::new(0, 0, 0, 0)]];
    for ix in (0..sentence.len()).rev() {
        debug_assert!(nodes.len() == sentence.len() - ix);
        let mut column = vec![];
        dic.each_prefix16(&sentence[ix..], |len, info_list| {
            let len = len + 1; // 一旦こっちで辻褄をあわせる。
            let search_nodes = &nodes[nodes.len() - len];
            for info in info_list {
                if let Some((index, min_cost)) = find_min_cost(info.right_id, search_nodes, matrix) {
                    column.push(Node::new(info.left_id, min_cost + info.cost as i32, len as u8, index as u8));
                }
            }
        });
        nodes.push(column);
    }
    if let Some((index, min_cost)) = find_min_cost(0, nodes.last().unwrap(), matrix) {
        println!("cost = {}", min_cost);
        let mut x = index;
        let mut y = 0;
        while y < nodes.len() {
            let node = &nodes[nodes.len() - y - 1][x];
            if node.len == 0 { break; }
            debug_print(&sentence, y, &node);
            x = node.next as usize;
            y += node.len as usize;
        }
    } else {
        panic!("形態素解析に失敗しました。");
    }
}

fn debug_print(sentence: &[u16], start_ix: usize, node: &Node) {
    let slice = &sentence[start_ix .. start_ix + node.len as usize];
    let word = decode_utf16(slice);
    println!("id:{:>5} | cost:{:>6} | {}", node.id, node.cost, word);
}