use crate::Node;
use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn linked_nodes(&self) -> Vec<Node> {
        match self {
            StrNodeBody::MapStrNodeBody(body) => body.node_links.values().cloned().collect(),
            StrNodeBody::VecStrNodeBody(body) => body.node_links.clone()
        }
    }
}
