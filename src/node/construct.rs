use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::Node;
use crate::node_body::StrNodeBody;

impl Node {
    /// Construct Node
    ///
    /// # Example
    /// ```rust
    /// use heap_node::Node;
    ///
    /// let node = Node::new();
    /// ```
    pub fn new() -> Self {
        Self { body: Rc::new(RefCell::new(StrNodeBody { payload: HashMap::new(), node_links: HashMap::new() })) }
    }
}


impl Node {
    /// Construct Node from payload & link vectors
    ///
    /// # Example
    /// ```rust
    /// use heap_node::Node;
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
                let mut hmap: HashMap<&str, &str> = HashMap::with_capacity(v.len());
                for (k, v) in v.iter() {
                    hmap.insert(*k, *v);
                }

                hmap
            },
            None => HashMap::new(),
        };

        let node_links = match node_links {
            Some(v) => {
                let mut hmap: HashMap<&str, Node> = HashMap::with_capacity(v.len());
                for (k, v) in v.iter() {
                    hmap.insert(*k, v.clone());
                }

                hmap
            },
            None => HashMap::new(),
        };


        Self { body: Rc::new(RefCell::new(StrNodeBody { payload, node_links })) }
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
        assert_eq!(v1, Some("value1"));

        let n_1 = node_2.node_by_key("node_1");
        assert!(n_1.is_some());
    }
}