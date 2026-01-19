use crate::Node;
use crate::node_body::StrNodeBody;

impl StrNodeBody {
    pub fn link(&mut self, link_as: &str, node: Node) {
        match self {
            Self::MapStrNodeBody(body) => {
                body.node_links.insert(link_as.to_string(), node);
            },
            _ => panic!("link allowed for map nodes only")
        }
    }


    pub fn unlink(&mut self, link_as: &str) {
        match self {
            Self::MapStrNodeBody(body) => {
                body.node_links.remove(link_as);
            },
            _ => panic!("unlink allowed for map nodes only")
        }
    }

    pub fn link_keys(&self) -> Vec<String> {
        match self {
            Self::MapStrNodeBody(body) => {
                body.node_links.keys().map(|k| k.clone()).collect()
            },
            _ => panic!("link_keys allowed for map nodes only")
        }
    }

    pub fn node_by_key(&self, k: &str) -> Option<Node> {
        match self {
            Self::MapStrNodeBody(body) => {
                body.node_links.get(k).cloned()
            },
            _ => panic!("node_by_key allowed for map nodes only")
        }
    }
}
