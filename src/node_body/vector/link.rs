use crate::Node;
use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn node_by_index(&self, i: usize) -> Option<Node> {
        match self {
            Self::VecStrNodeBody(body) => {
                body.node_links.get(i).cloned()
            },
            _ => panic!("push_node allowed for vector nodes only")
        }
    }

    pub fn push_node(&mut self, v: Node) {
        match self {
            Self::VecStrNodeBody(body) => {
                body.node_links.push(v);
            },
            _ => panic!("push_node allowed for vector nodes only")
        }
    }

    pub fn pop_node(&mut self) -> Option<Node> {
        match self {
            Self::VecStrNodeBody(body) => {
                body.node_links.pop()
            },
            _ => panic!("push_node allowed for vector nodes only")
        }
    }

    pub fn remove_by_node_by_index(&mut self, i: usize) -> Node {
        match self {
            Self::VecStrNodeBody(body) => {
                body.node_links.remove(i)
            },
            _ => panic!("remove_by_node_by_index allowed for vector nodes only")
        }
    }
}

