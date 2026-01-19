mod map;
mod vector;
mod universal;

use std::collections::HashMap;
use crate::Node;


#[derive(Debug)]
pub struct MapStrNodeBody {
    pub payload: HashMap<String, String>,
    pub node_links: HashMap<String, Node>
}


#[derive(Debug)]
pub struct VecStrNodeBody {
    pub payload: Vec<String>,
    pub node_links: Vec<Node>
}

#[derive(Debug)]
pub enum StrNodeBody {
    MapStrNodeBody(MapStrNodeBody),
    VecStrNodeBody(VecStrNodeBody)
}

