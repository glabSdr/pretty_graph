use std::sync::{Arc, RwLock};
use crate::Node;
use crate::node_body::{StrNodeBody, VecStrNodeBody};

impl Node {
    /// Construct Node
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new_vec();
    /// ```
    pub fn new_vec() -> Self {
        Self { body: Arc::new(RwLock::new(StrNodeBody::VecStrNodeBody(VecStrNodeBody { payload: Vec::new(), node_links: Vec::new() }))) }
    }
}

impl Node {
    /// Construct Node from payload & link vectors
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::new();
    /// let node_2 = Node::from(
    ///     Some(vec![
    ///         "value1",
    ///         "value2"
    ///     ]),
    ///     Some(vec![
    ///         node_1
    ///     ]);
    /// );
    /// ```
    pub fn vec_from(payload: Option<Vec<&str>>, node_links: Option<Vec<Node>>) -> Self {
        let payload: Vec<String> = match payload {
            Some(v) => v.iter().map(|v| v.to_string()).collect(),
            None => Vec::new(),
        };

        let node_links = node_links.unwrap_or(Vec::new());

        Self { body: Arc::new(RwLock::new(StrNodeBody::VecStrNodeBody(VecStrNodeBody { payload, node_links }))) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vec() {
        let _node = Node::new_vec();
    }

    #[test]
    fn test_vec_from() {
        let node_1 = Node::new();
        let _node_2 = Node::vec_from(
            Some(vec![
                "value1"
            ]),
            Some(vec![
                node_1
            ])
        );
    }

    #[test]
    fn test_empty_vec_from() {
        let _node_2 = Node::vec_from(None, None);
    }
}