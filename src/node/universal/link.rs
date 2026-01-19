use crate::Node;


impl Node {
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
        self.body.read().unwrap().linked_nodes()
    }
}


#[cfg(test)]
mod tests {
    use crate::Node;

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
            assert_eq!(Some("value".to_string()), v);
        }
    }

    #[test]
    fn test_vector_linked_nodes() {
        let node_2 = Node::new();
        let node_3 = Node::new();

        let node_1 = Node::vec_from(None, Some(vec![node_2, node_3]));
        let _ = node_1.linked_nodes();
    }
}