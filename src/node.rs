mod construct;
mod payload_ops;
mod link;
mod node_by;
mod payload_access;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::node_body::StrNodeBody;

#[derive(Debug, Clone)]
pub struct Node {
    body: Arc<RwLock<StrNodeBody>>
}

