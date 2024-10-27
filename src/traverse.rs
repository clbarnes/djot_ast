pub enum NodeType {
    Root(usize),
    Branch(usize),
    Slab,
    Leaf,
}

impl NodeType {
    pub fn is_root(&self) -> bool {
        matches!(self, Self::Root(_))
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf)
    }

    pub fn n_children(&self) -> usize {
        match self {
            NodeType::Root(n) => *n,
            NodeType::Branch(n) => *n,
            NodeType::Slab => 1,
            NodeType::Leaf => 0,
        }
    }
}

pub trait Node {
    fn node_type(&self) -> NodeType;
}
