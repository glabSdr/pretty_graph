use std::collections::HashMap;
use crate::Node;


#[derive(Debug)]
pub struct StrNodeBody {
    pub payload: HashMap<String, String>,
    pub node_links: HashMap<String, Node>
}