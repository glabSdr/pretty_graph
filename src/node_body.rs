use std::collections::HashMap;
use crate::Node;


#[derive(Debug)]
pub struct StrNodeBody {
    pub payload: HashMap<&'static str, &'static str>,
    pub node_links: HashMap<&'static str, Node>
}