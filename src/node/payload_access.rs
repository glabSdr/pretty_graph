use crate::Node;

impl Node {
    /// Get payload keys
    ///
    /// # Example
    /// ```rust
    ///let node = Node::new();
    ///node.set("key1", "value1");
    ///node.set("key2", "value2");
    ///
    ///let keys = vec!["key1", "key2"];
    ///
    ///for k in node.keys() {
    ///    assert!(keys.contains(&k));
    ///}
    /// ```
    pub fn keys(&self) -> Vec<&str> {
        self.body.borrow().payload.keys().copied().collect()
    }

    /// Get payload values
    ///
    /// # Example
    /// ```rust
    ///let node = Node::new();
    ///node.set("key1", "value1");
    ///node.set("key2", "value2");
    ///
    ///let values = vec!["value1", "value2"];
    ///
    ///for v in node.values() {
    ///    assert!(values.contains(&v));
    ///}
    /// ```
    pub fn values(&self) -> Vec<&str> {
        self.body.borrow().payload.values().copied().collect()
    }

    /// Get linked node keys
    ///
    /// # Example
    /// ```rust
    ///let node_3 = Node::new();
    ///let node_2 = linked_nodes();
    ///node_2.link("node_3", node_3);
    ///
    ///let link_names = vec!["node_1", "node_3"];
    ///
    ///for k in node_2.link_keys() {
    ///    assert!(link_names.contains(&k));
    ///}
    /// ```
    pub fn link_keys(&self) -> Vec<&str> {
        self.body.borrow().node_links.keys().copied().collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::tests::tests::linked_nodes;
    use super::*;

    #[test]
    fn test_keys() {
        let node = Node::new();
        node.set("key1", "value1");
        node.set("key2", "value2");

        let keys = vec!["key1", "key2"];

        for k in node.keys() {
            assert!(keys.contains(&k));
        }
    }

    #[test]
    fn test_values() {
        let node = Node::new();
        node.set("key1", "value1");
        node.set("key2", "value2");

        let values = vec!["value1", "value2"];

        for v in node.values() {
            assert!(values.contains(&v));
        }
    }

    #[test]
    fn test_link_keys() {
        let node_3 = Node::new();
        let node_2 = linked_nodes();
        node_2.link("node_3", node_3);

        let link_names = vec!["node_1", "node_3"];

        for k in node_2.link_keys() {
            assert!(link_names.contains(&k));
        }
    }
}