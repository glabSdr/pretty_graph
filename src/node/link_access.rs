use crate::Node;


impl Node {
    /// Get linked node keys
    ///
    /// # Example
    /// ```rust
    ///let node_3 = Node::new();
    ///let node_2 = Node::new();
    ///let node_1 = Node::new();
    ///
    ///node_1.link("node_2", node_2);
    ///node_1.link("node_3", node_3);
    ///
    ///for k in node_1.link_keys() {
    ///    // ...
    ///}
    /// ```
    pub fn link_keys(&self) -> Vec<&str> {
        self.body.read().unwrap().node_links.keys().copied().collect()
    }

    /// Get linked nodes
    ///
    /// # Example
    /// ```rust
    ///let node_3 = Node::new();
    ///let node_2 = Node::new();
    ///let node_1 = Node::new();
    ///
    ///node_1.link("node_2", node_2);
    ///node_1.link("node_3", node_3);
    ///
    ///for k in node_1.linked_nodes() {
    ///    // ...
    ///}
    /// ```
    pub fn linked_nodes(&self) -> Vec<Node> {
        self.body.read().unwrap().node_links.values().cloned().collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::Node;
    use crate::tests::tests::linked_nodes;

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

    #[test]
    fn test_linked_nodes() {
        let node_3 = Node::from(Some(vec![
            ("key", "value")
        ]), None);

        let node_2 = Node::from(Some(vec![
            ("key", "value")
        ]), None);

        let node_1 = Node::new();
        node_1.link("node_3", node_3);
        node_1.link("node_2", node_2);
        
        for node in node_1.linked_nodes() {
            let v = node.get("key");
            assert_eq!(Some("value"), v);
        }
    }
}