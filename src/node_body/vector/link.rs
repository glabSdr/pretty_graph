use crate::Node;
use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn push_node(&mut self, v: Node) {
        match self {
            Self::VecStrNodeBody(body) => {
                body.node_links.push(v);
            },
            _ => panic!("push_node allowed for vector nodes only")
        }
    }
}