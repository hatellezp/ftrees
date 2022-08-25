use crate::storage::*;

#[derive(Clone, PartialEq)]
pub struct Node {
    id: u64,
    data: u64,
    parent: Option<u64>,
    children: Vec<u64>,
}

impl Node {
    pub fn new(id: u64, data: u64, parent: Option<u64>) -> Node {
        let children: Vec<u64> = vec![];

        Node {
            id,
            data,
            parent,
            children,
        }
    }

    pub fn set_parent(&mut self, parent: u64) {
        self.parent = Some(parent);
    }

    pub fn add_child(&mut self, child: u64) {
        self.children.push(child);
    }

    pub fn is_parent(&self, parent: &u64) -> bool {
        self.parent == Some(*parent)
    }

    pub fn is_child(&self, child: &u64) -> bool {
        self.children.contains(child)
    }
}

pub type Registry = Storage<Node>;
