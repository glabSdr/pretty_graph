mod map;
mod vector;
mod universal;

use std::sync::{Arc, RwLock};
use crate::node_body::StrNodeBody;

#[derive(Debug, Clone)]
pub struct Node {
    body: Arc<RwLock<StrNodeBody>>
}