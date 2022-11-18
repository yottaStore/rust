use bincode::serialize;
use xxhash_rust::xxh3::xxh3_128;

use crate::coords::get_coords;
use crate::utils::{merge_visited_nodes, MergedUpdates};
const HASH_SIZE: usize = 16;

pub enum UpdateType {
    Add,
    Delete,
}

pub struct Update {
    pub update_type: UpdateType,
    pub pointer: String,
    pub weight: u32,
    pub load: u32,
}

#[derive(Clone)]
pub enum NodeType {
    // A leaf node.
    Leaf,
    // An internal node.
    Internal,
    // A root node.
    Root,
    Deleted,
}

#[derive(Clone)]
struct Node {
    node_type: NodeType,
    pointer: String,
    weight: u32,
    load: u32,
    parent: usize,
    children: Vec<usize>,
    hash: u128,
}

impl Node {
    fn update(&mut self, htree: &HTree) {
        println!("update node: {}", self.pointer);

        if self.children.len() == 0 {
            let mut serialized: Vec<u8> = Vec::with_capacity(5);
            serialized.push(self.weight as u8);
            return;
        } else {
        }
    }
}

pub struct HTree {
    prefix: String,
    nodes: Vec<Node>,
}

impl HTree {
    pub fn new(prefix: String) -> HTree {
        let root = Node {
            node_type: NodeType::Root,
            pointer: prefix.clone(),
            weight: 0,
            load: 0,
            parent: 0,
            children: Vec::with_capacity(1),
            hash: 0,
        };
        let mut h = HTree {
            prefix,
            nodes: Vec::with_capacity(4),
        };
        h.nodes.push(root);
        h
    }

    pub fn update(&mut self, updates: &[Update]) {
        let mut merged: &mut MergedUpdates = &mut Vec::with_capacity(updates.len());

        for update in updates {
            let mut visited_nodes: Vec<usize> = Vec::with_capacity(updates.len());

            match update.update_type {
                UpdateType::Add => {
                    let nodes = add_node(self, update);
                    visited_nodes.extend(nodes);
                }
                UpdateType::Delete => {
                    println!("Delete not implemented");
                }
            }

            merged = merge_visited_nodes(visited_nodes, merged);
        }
        //let nodes = &mut self.nodes;

        for level in merged.iter().rev() {
            for node in level {
                self.update_node(*node);
            }
        }
    }

    fn update_node(&mut self, pointer: usize) {
        let node = &self.nodes[pointer];
        println!("update node: {}", node.pointer);
        let mut serialized: Vec<u8> = Vec::with_capacity(5);

        if node.children.len() == 0 {
            //serialized.append(&mut serialize(&(&node.node_type as usize)).unwrap());
            serialized.append(&mut serialize(&node.pointer).unwrap());
            serialized.append(&mut serialize(&node.weight).unwrap());
            serialized.append(&mut serialize(&node.load).unwrap());
            serialized.append(&mut serialize(&node.parent).unwrap());

            //node.hash = xxh3_128(&serialized);
        } else {
            //let size = node.children.len() * HASH_SIZE;
            //let mut serialized: Vec<u8> = Vec::with_capacity(size);
            serialized.append(&mut serialize(&node.pointer).unwrap());
            serialized.append(&mut serialize(&node.weight).unwrap());
            serialized.append(&mut serialize(&node.load).unwrap());
            serialized.append(&mut serialize(&node.parent).unwrap());
            for child in &node.children {
                let hash = &self.nodes[*child].hash;
                let mut ser_hash = serialize(&hash).unwrap().clone();
                serialized.append(&mut ser_hash);
            }
        }
        let node = &mut self.nodes[pointer];
        node.hash = xxh3_128(&serialized.clone());
    }

    pub fn verify(self) {
        // TODO: implement verify
    }

    pub fn print(&self) {
        println!("HTree {{ nodes: {} }}", self.nodes.len());
    }
}

fn add_node(htree: &mut HTree, update: &Update) -> Vec<usize> {
    let Update {
        pointer,
        weight,
        load,
        ..
    } = update;

    let coords = get_coords(pointer, &htree.prefix);

    let mut visited_nodes: Vec<usize> = Vec::with_capacity(4);

    let mut last_node = 0;
    visited_nodes.push(last_node);

    'outer: for coord in coords.iter() {
        let mut node_pos = 0;
        for child in &htree.nodes[last_node].children {
            let child_node = &htree.nodes[*child];
            if child_node.pointer == coord.to_string() {
                last_node = *child;
                visited_nodes.push(last_node);
                continue 'outer;
            }
            if child_node.pointer > coord.to_string() {
                last_node = *child;
                visited_nodes.push(last_node);
                break;
            }
            node_pos += 1;
        }
        // create node if not found
        let node = Node {
            node_type: NodeType::Leaf,
            pointer: coord.to_string(),
            weight: *weight,
            load: *load,
            parent: last_node,
            children: Vec::with_capacity(1),
            hash: 0,
        };

        let node_index = htree.nodes.len();
        htree.nodes.push(node);
        visited_nodes.push(node_index);
        htree.nodes[last_node]
            .children
            .insert(node_pos as usize, node_index);
        last_node = node_index;
    }

    visited_nodes
}
