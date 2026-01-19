use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::Node;
use crate::node_body::{MapStrNodeBody, StrNodeBody};

impl Node {
    /// Construct Node
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node = Node::new();
    /// ```
    pub fn new() -> Self {
        Self { body: Arc::new(RwLock::new(StrNodeBody::MapStrNodeBody(MapStrNodeBody { payload: HashMap::new(), node_links: HashMap::new() }))) }
    }
}




impl Node {
    /// Construct Node from payload & link vectors
    ///
    /// # Example
    /// ```rust
    /// use pretty_graph::Node;
    ///
    /// let node_1 = Node::from(None, None); // Params are optional
    /// let node_2 = Node::from(
    ///     Some(vec![
    ///         ("key1", "value1"),
    ///         ("key2", "value2")
    ///     ]),
    ///     Some(vec![
    ///         ("node_1", node_1.clone())
    ///     ])
    /// );
    /// ```
    pub fn from(payload: Option<Vec<(&'static str, &'static str)>>, node_links: Option<Vec<(&'static str, Node)>>) -> Self {
        let payload = match payload {
            Some(v) => {
                let mut hmap = HashMap::with_capacity(v.len());
                for (k, v) in v.iter() {
                    hmap.insert(k.to_string(), v.to_string());
                }

                hmap
            },
            None => HashMap::new(),
        };

        let node_links = match node_links {
            Some(v) => {
                let mut hmap = HashMap::with_capacity(v.len());
                for (k, v) in v.iter() {
                    hmap.insert(k.to_string(), v.clone());
                }

                hmap
            },
            None => HashMap::new(),
        };


        Self { body: Arc::new(RwLock::new(StrNodeBody::MapStrNodeBody(MapStrNodeBody { payload, node_links }))) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _node = Node::new();
    }

    #[test]
    fn test_from() {
        let node_1 = Node::from(None, None);
        let node_2 = Node::from(
            Some(vec![
                ("key1", "value1"),
            ]),
            Some(vec![
                ("node_1", node_1.clone())
            ])
        );

        let v1 = node_2.get("key1");
        assert_eq!(v1, Some("value1".to_string()));

        let n_1 = node_2.node_by_key("node_1");
        assert!(n_1.is_some());
    }
}